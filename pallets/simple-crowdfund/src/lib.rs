//! Simple Crowdfund
//!
//! This pallet demonstrates a simple on-chain crowd-funding mechanism.
//! It is based on Polkadot's crowdfund pallet, but is simplified and decoupled
//! from the parachain logic.

use parity_scale_codec::{Decode, Encode};
use sp_core::Hasher;
use sp_std::prelude::*;
use sp_runtime::{
	traits::{AccountIdConversion, Saturating, Zero},
	ModuleId,
};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, ensure,
	storage::child,
	traits::{
		Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReason,
		WithdrawReasons,
	},
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

const PALLET_ID: ModuleId = ModuleId(*b"ex/cfund");

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type FundInfoOf<T> = FundInfo<BalanceOf<T>, <T as system::Trait>::BlockNumber>;

/// The pallet's configuration trait
pub trait Trait: system::Trait {
	/// The ubiquious Event type
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// The currency in which the crowdfunds will be denominated
	type Currency: ReservableCurrency<Self::AccountId>;

	/// The amount to be held on deposit by the owner of a crowdfund
	type SubmissionDeposit: Get<BalanceOf<Self>>;

	/// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
	/// least ExistentialDeposit.
	type MinContribution: Get<BalanceOf<Self>>;

	/// The period of time (in blocks) after an unsuccessful crowdfund ending during which
	/// contributors are able to withdraw their funds. After this period, their funds are lost.
	type RetirementPeriod: Get<Self::BlockNumber>;
}

/// Simple index for identifying a fund.
pub type FundIndex = u32;

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FundInfo<Balance, BlockNumber> {
	/// The amount of deposit placed
	deposit: Balance,
	/// The total amount raised
	raised: Balance,
	/// Block number after which funding must have succeeded
	end: BlockNumber,
	/// Upper bound on `raised`
	cap: Balance,
}

decl_storage! {
	trait Store for Module<T: Trait> as ChildTrie {
		/// Info on all of the funds.
		Funds get(fn funds):
			map hasher(blake2_128_concat) FundIndex => Option<FundInfoOf<T>>;

		/// The total number of funds that have so far been allocated.
		FundCount get(fn fund_count): FundIndex;

		// Additional information is stored i na child trie. See the helper
		// functions in the impl<T: Trait> Module<T> block below
	}
}

decl_event! {
	pub enum Event<T> where
		Balance = BalanceOf<T>,
		<T as system::Trait>::AccountId,
		<T as system::Trait>::BlockNumber,
	{
		Created(FundIndex, BlockNumber),
		Contributed(AccountId, FundIndex, Balance, BlockNumber),
		Withdrew(AccountId, FundIndex, Balance, BlockNumber),
		Retiring(FundIndex, BlockNumber),
		Dissolved(FundIndex, BlockNumber, AccountId),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Crowdfund must end after it starts
		EndTooEarly,
		/// Must contribute at least the minimum amount of funds
		ContributionTooSmall,
		/// The fund index specified does not exist
		InvalidIndex,
		/// The crowdfund's contribution period has ended; no more contributions will be accepted
		ContributionPeriodOver,
		/// You may not withdraw funds while the fund is still active
		FundStillActive,
		/// You cannot withdraw funds because you have not contributed any
		NoContribution,
		/// You cannot dissolve a fund that has not yet completed its retirement period
		FundNotRetired,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Create a new fund
		#[weight = 10_000]
		fn create(
			origin,
			cap: BalanceOf<T>,
			end: T::BlockNumber,
		) {
			let creator = ensure_signed(origin)?;
			let now = <system::Module<T>>::block_number();

			ensure!(end > now, Error::<T>::EndTooEarly);

			let deposit = T::SubmissionDeposit::get();
			let imb = T::Currency::withdraw(
				&creator,
				deposit,
				WithdrawReasons::from(WithdrawReason::Transfer),
				ExistenceRequirement::AllowDeath,
			)?;

			let index = FundCount::get();
			// not protected against overflow, see safemath section
			FundCount::put(index + 1);

			// No fees are paid here if we need to create this account; that's why we don't just
			// use the stock `transfer`.
			T::Currency::resolve_creating(&Self::fund_account_id(index), imb);

			<Funds<T>>::insert(index, FundInfo {
				deposit,
				raised: Zero::zero(),
				end,
				cap,
			});

			Self::deposit_event(RawEvent::Created(index, now));
		}

		/// Contribute funds to an existing fund
		#[weight = 10_000]
		fn contribute(origin, index: FundIndex, value: BalanceOf<T>) {
			let who = ensure_signed(origin)?;

			ensure!(value >= T::MinContribution::get(), Error::<T>::ContributionTooSmall);
			let mut fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Make sure crowdfund has not ended
			let now = <system::Module<T>>::block_number();
			ensure!(fund.end > now, Error::<T>::ContributionPeriodOver);

			// Add contribution to the fund
			T::Currency::transfer(
				&who,
				&Self::fund_account_id(index),
				value,
				ExistenceRequirement::AllowDeath
			)?;
			fund.raised += value;
			Funds::<T>::insert(index, &fund);

			let balance = Self::contribution_get(index, &who);
			let balance = balance.saturating_add(value);
			Self::contribution_put(index, &who, &balance);

			Self::deposit_event(RawEvent::Contributed(who, index, balance, now));
		}

		/// Withdraw full balance of a contributor to a fund
		#[weight = 10_000]
		fn withdraw(origin, #[compact] index: FundIndex) {
			let who = ensure_signed(origin)?;

			let mut fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;
			let now = <system::Module<T>>::block_number();
			ensure!(fund.end < now, Error::<T>::FundStillActive);

			let balance = Self::contribution_get(index, &who);
			ensure!(balance > Zero::zero(), Error::<T>::NoContribution);

			// Return funds to caller without charging a transfer fee
			let _ = T::Currency::resolve_into_existing(&who, T::Currency::withdraw(
				&Self::fund_account_id(index),
				balance,
				WithdrawReasons::from(WithdrawReason::Transfer),
				ExistenceRequirement::AllowDeath
			)?);

			// Update storage
			Self::contribution_kill(index, &who);
			fund.raised = fund.raised.saturating_sub(balance);
			<Funds<T>>::insert(index, &fund);

			Self::deposit_event(RawEvent::Withdrew(who, index, balance, now));
		}

		/// Dissolve an entire crowdfund after its retirement period has expired.
		/// Anyone can call this function, and they are incentivized to do so because
		/// They inheret the deposit.
		#[weight = 10_000]
		fn dissolve(origin, index: FundIndex) {
			let reporter = ensure_signed(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Check that enough time has passed to remove from storage
			let now = <system::Module<T>>::block_number();
			ensure!(now >= fund.end + T::RetirementPeriod::get(), Error::<T>::FundNotRetired);

			let account = Self::fund_account_id(index);

			// Dissolver collects the deposit and any remaining funds
			let _ = T::Currency::resolve_into_existing(&reporter, T::Currency::withdraw(
				&account,
				fund.deposit + fund.raised,
				WithdrawReasons::from(WithdrawReason::Transfer),
				ExistenceRequirement::AllowDeath,
			)?);

			// Remove the fund info from storage
			<Funds<T>>::remove(index);
			// Remove all the contributor info from storage in a single write.
			// This is possible thanks to the use of a child tree.
			Self::crowdfund_kill(index);

			Self::deposit_event(RawEvent::Dissolved(index, now, reporter));
		}
	}
}

impl<T: Trait> Module<T> {
	/// The account ID of the fund pot.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn fund_account_id(index: FundIndex) -> T::AccountId {
		PALLET_ID.into_sub_account(index)
	}

	/// Find the ID associated with the fund
	///
	/// Each fund stores information about its contributors and their contributions in a child trie
	/// This helper function calculates the id of the associated child trie.
	pub fn id_from_index(index: FundIndex) -> child::ChildInfo {
		let mut buf = Vec::new();
		buf.extend_from_slice(b"crowdfnd");
		buf.extend_from_slice(&index.to_le_bytes()[..]);

		child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
	}

	/// Record a contribution in the associated child trie.
	pub fn contribution_put(index: FundIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::put(&id, b, &balance));
	}

	/// Lookup a contribution in the associated child trie.
	pub fn contribution_get(index: FundIndex, who: &T::AccountId) -> BalanceOf<T> {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::get_or_default::<BalanceOf<T>>(&id, b))
	}

	/// Remove a contribution from an associated child trie.
	pub fn contribution_kill(index: FundIndex, who: &T::AccountId) {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::kill(&id, b));
	}

	/// Remove the enire record of contributions in the associated child trie in a single
	/// storage write.
	pub fn crowdfund_kill(index: FundIndex) {
		let id = Self::id_from_index(index);
		child::kill_storage(&id);
	}
}
