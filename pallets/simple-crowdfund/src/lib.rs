//! Simple Crowdfund
//!
//! This pallet demonstrates a simple on-chain crowdfunding mechanism.
//! It is based on Polkadot's crowdfund pallet, but is simplified and decoupled
//! from the parachain logic.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, ensure,
	storage::child,
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
};
use frame_system::ensure_signed;
use parity_scale_codec::{Decode, Encode};
use sp_core::Hasher;
use sp_runtime::{
	traits::{AccountIdConversion, Saturating, Zero},
	ModuleId,
};
use sp_std::prelude::*;

#[cfg(test)]
mod tests;

const PALLET_ID: ModuleId = ModuleId(*b"ex/cfund");

/// The pallet's configuration trait
pub trait Config: frame_system::Config {
	/// The ubiquious Event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

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

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type FundInfoOf<T> =
	FundInfo<AccountIdOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FundInfo<AccountId, Balance, BlockNumber> {
	/// The account that will receive the funds if the campaign is successful
	beneficiary: AccountId,
	/// The amount of deposit placed
	deposit: Balance,
	/// The total amount raised
	raised: Balance,
	/// Block number after which funding must have succeeded
	end: BlockNumber,
	/// Upper bound on `raised`
	goal: Balance,
}

decl_storage! {
	trait Store for Module<T: Config> as ChildTrie {
		/// Info on all of the funds.
		Funds get(fn funds):
			map hasher(blake2_128_concat) FundIndex => Option<FundInfoOf<T>>;

		/// The total number of funds that have so far been allocated.
		FundCount get(fn fund_count): FundIndex;

		// Additional information is stored in a child trie. See the helper
		// functions in the impl<T: Config> Module<T> block below
	}
}

decl_event! {
	pub enum Event<T> where
		Balance = BalanceOf<T>,
		<T as frame_system::Config>::AccountId,
		<T as frame_system::Config>::BlockNumber,
	{
		Created(FundIndex, BlockNumber),
		Contributed(AccountId, FundIndex, Balance, BlockNumber),
		Withdrew(AccountId, FundIndex, Balance, BlockNumber),
		Retiring(FundIndex, BlockNumber),
		Dissolved(FundIndex, BlockNumber, AccountId),
		Dispensed(FundIndex, BlockNumber, AccountId),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Crowdfund must end after it starts
		EndTooEarly,
		/// Must contribute at least the minimum amount of funds
		ContributionTooSmall,
		/// The fund index specified does not exist
		InvalidIndex,
		/// The crowdfund's contribution period has ended; no more contributions will be accepted
		ContributionPeriodOver,
		/// You may not withdraw or dispense funds while the fund is still active
		FundStillActive,
		/// You cannot withdraw funds because you have not contributed any
		NoContribution,
		/// You cannot dissolve a fund that has not yet completed its retirement period
		FundNotRetired,
		/// Cannot dispense funds from an unsuccessful fund
		UnsuccessfulFund,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;

		/// Create a new fund
		#[weight = 10_000]
		fn create(
			origin,
			beneficiary: AccountIdOf<T>,
			goal: BalanceOf<T>,
			end: T::BlockNumber,
		) {
			let creator = ensure_signed(origin)?;
			let now = <frame_system::Module<T>>::block_number();

			ensure!(end > now, Error::<T>::EndTooEarly);

			let deposit = T::SubmissionDeposit::get();
			let imb = T::Currency::withdraw(
				&creator,
				deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;

			let index = FundCount::get();
			// not protected against overflow, see safemath section
			FundCount::put(index + 1);

			// No fees are paid here if we need to create this account; that's why we don't just
			// use the stock `transfer`.
			T::Currency::resolve_creating(&Self::fund_account_id(index), imb);

			<Funds<T>>::insert(index, FundInfo {
				beneficiary,
				deposit,
				raised: Zero::zero(),
				end,
				goal,
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
			let now = <frame_system::Module<T>>::block_number();
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
			let now = <frame_system::Module<T>>::block_number();
			ensure!(fund.end < now, Error::<T>::FundStillActive);

			let balance = Self::contribution_get(index, &who);
			ensure!(balance > Zero::zero(), Error::<T>::NoContribution);

			// Return funds to caller without charging a transfer fee
			let _ = T::Currency::resolve_into_existing(&who, T::Currency::withdraw(
				&Self::fund_account_id(index),
				balance,
				WithdrawReasons::TRANSFER,
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
		/// they inherit the deposit.
		#[weight = 10_000]
		fn dissolve(origin, index: FundIndex) {
			let reporter = ensure_signed(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Check that enough time has passed to remove from storage
			let now = <frame_system::Module<T>>::block_number();
			ensure!(now >= fund.end + T::RetirementPeriod::get(), Error::<T>::FundNotRetired);

			let account = Self::fund_account_id(index);

			// Dissolver collects the deposit and any remaining funds
			let _ = T::Currency::resolve_creating(&reporter, T::Currency::withdraw(
				&account,
				fund.deposit + fund.raised,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?);

			// Remove the fund info from storage
			<Funds<T>>::remove(index);
			// Remove all the contributor info from storage in a single write.
			// This is possible thanks to the use of a child tree.
			Self::crowdfund_kill(index);

			Self::deposit_event(RawEvent::Dissolved(index, now, reporter));
		}

		/// Dispense a payment to the beneficiary of a successful crowdfund.
		/// The beneficiary receives the contributed funds and the caller receives
		/// the deposit as a reward to incentivize clearing settled crowdfunds out of storage.
		#[weight = 10_000]
		fn dispense(origin, index: FundIndex) {
			let caller = ensure_signed(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Check that enough time has passed to remove from storage
			let now = <frame_system::Module<T>>::block_number();

			ensure!(now >= fund.end, Error::<T>::FundStillActive);

			// Check that the fund was actually successful
			ensure!(fund.raised >= fund.goal, Error::<T>::UnsuccessfulFund);

			let account = Self::fund_account_id(index);

			// Beneficiary collects the contributed funds
			let _ = T::Currency::resolve_creating(&fund.beneficiary, T::Currency::withdraw(
				&account,
				fund.raised,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?);

			// Caller collects the deposit
			let _ = T::Currency::resolve_creating(&caller, T::Currency::withdraw(
				&account,
				fund.deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?);

			// Remove the fund info from storage
			<Funds<T>>::remove(index);
			// Remove all the contributor info from storage in a single write.
			// This is possible thanks to the use of a child tree.
			Self::crowdfund_kill(index);

			Self::deposit_event(RawEvent::Dispensed(index, now, caller));
		}
	}
}

impl<T: Config> Module<T> {
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

	/// Remove the entire record of contributions in the associated child trie in a single
	/// storage write.
	pub fn crowdfund_kill(index: FundIndex) {
		let id = Self::id_from_index(index);
		// The None here means we aren't setting a limit to how many keys to delete.
		// Limiting can be useful, but is beyond the scope of this recipe. For more info, see
		// https://crates.parity.io/frame_support/storage/child/fn.kill_storage.html
		child::kill_storage(&id, None);
	}
}
