//! Simple Crowdfund Example
//! - example of using `child-trie` in practice
//! - designed to be a more simple version of polkadot/runtime/crowdfund
use parity_scale_codec::{Decode, Encode};
use sp_core::{Blake2Hasher, Hasher};
use rstd::prelude::*;
use sp_runtime::{
    traits::{AccountIdConversion, Saturating, Zero},
    ModuleId,
};
use sp_storage::well_known_keys::CHILD_STORAGE_KEY_PREFIX;
use support::{
    decl_event, decl_module, decl_storage, ensure,
    storage::child,
    traits::{
        Currency, ExistenceRequirement, Get, OnUnbalanced, ReservableCurrency, WithdrawReason,
        WithdrawReasons,
    },
};
use system::ensure_signed;

const PALLET_ID: ModuleId = ModuleId(*b"ex/cfund");

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type Currency: ReservableCurrency<Self::AccountId>;

    /// The amount to be held on deposit by the owner of a crowdfund
    type SubmissionDeposit: Get<BalanceOf<Self>>;

    /// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
    /// least ExistentialDeposit.
    type MinContribution: Get<BalanceOf<Self>>;

    /// The period of time (in blocks) after an unsuccessful crowdfund ending when
    /// contributors are able to withdraw their funds. After this period, their funds are lost.
    type RetirementPeriod: Get<Self::BlockNumber>;

    /// What to do with funds that were not withdrawn.
    type OrphanedFunds: OnUnbalanced<NegativeImbalanceOf<Self>>;
}

/// Simple index for identifying a fund.
pub type FundIndex = u32;

#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FundInfo<AccountId, Balance, BlockNumber> {
    /// The ownning account which placed the deposit
    owner: AccountId,
    /// The amount of deposit placed
    deposit: Balance,
    /// The total amount raised
    raised: Balance,
    /// Block number at which contributions are first accepted
    start: BlockNumber,
    /// Block number after which funding must have succeeded
    end: BlockNumber,
    /// Upper bound on `raised`
    cap: Balance,
}

decl_storage! {
    trait Store for Module<T: Trait> as ChildTrie {
        /// Info on all of the funds.
        Funds get(funds):
            map FundIndex => Option<FundInfo<T::AccountId, BalanceOf<T>, T::BlockNumber>>;

        /// The total number of funds that have so far been allocated.
        FundCount get(fund_count): FundIndex;

        /// The funds that have had additional contributions during the last block. This is used
        /// in order to determine which funds should submit new or updated bids.
        NewRaise get(new_raise): Vec<FundIndex>;
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
        Dissolved(FundIndex, BlockNumber),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn create(origin,
            #[compact] cap: BalanceOf<T>,
            #[compact] start: T::BlockNumber,
            #[compact] end: T::BlockNumber,
        ) {
            let owner = ensure_signed(origin)?;

            let now = <system::Module<T>>::block_number();

            ensure!(start < end, "must start before it ends");
            ensure!(end > now, "end must be in the future");

            let deposit = T::SubmissionDeposit::get();
            let imb = T::Currency::withdraw(
                &owner,
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
                owner,
                deposit,
                raised: Zero::zero(),
                start,
                end,
                cap,
            });

            Self::deposit_event(RawEvent::Created(index, now));
        }

        fn contribute(origin, #[compact] index: FundIndex, #[compact] value: BalanceOf<T>) {
            let who = ensure_signed(origin)?;

            ensure!(value >= T::MinContribution::get(), "contribution too small");
            let mut fund = Self::funds(index).ok_or("invalid fund index")?;

            // Make sure crowdfund has not ended
            let now = <system::Module<T>>::block_number();
            ensure!(fund.end > now, "contribution period ended");

            // Add value if cap is not exceeded
            ensure!(fund.raised + value < fund.cap, "contributions exceed cap");
            T::Currency::transfer(&who, &Self::fund_account_id(index), value)?;
            fund.raised += value;

            let balance = Self::contribution_get(index, &who);
            let balance = balance.saturating_add(value);
            Self::contribution_put(index, &who, &balance);

            Self::deposit_event(RawEvent::Contributed(who, index, balance, now));
        }

        /// Withdraw full balance of a contributor to a fund
        fn withdraw(origin, #[compact] index: FundIndex) {
            let who = ensure_signed(origin)?;

            let mut fund = Self::funds(index).ok_or("invalid fund index")?;
            let now = <system::Module<T>>::block_number();
            ensure!(fund.end < now, "no more withdrawals");
            // dcb4p: add withdrawal period `=>` could structure as an auction or ico

            let balance = Self::contribution_get(index, &who);
            ensure!(balance > Zero::zero(), "no contributions stored");

            // TODO: is this appropriate for all structures like this or
            // - is this just for polkadot/crowdfund?
            let _ = T::Currency::resolve_into_existing(&who, T::Currency::withdraw(
                &Self::fund_account_id(index),
                balance,
                WithdrawReasons::from(WithdrawReason::Transfer),
                ExistenceRequirement::AllowDeath
            )?);

            Self::contribution_kill(index, &who);
            fund.raised = fund.raised.saturating_sub(balance);

            <Funds<T>>::insert(index, &fund);

            Self::deposit_event(RawEvent::Withdrew(who, index, balance, now));
        }

        fn dissolve(origin, #[compact] index: FundIndex) {
            let _ = ensure_signed(origin)?;

            let fund = Self::funds(index).ok_or("invalid fund index")?;

            // Check that enough time has passed to remove from storage
            let now = <system::Module<T>>::block_number();
            ensure!(now >= fund.end + T::RetirementPeriod::get(), "retirement period not over");

            let account = Self::fund_account_id(index);

            let _ = T::Currency::resolve_into_existing(&fund.owner, T::Currency::withdraw(
                &account,
                fund.deposit,
                WithdrawReasons::from(WithdrawReason::Transfer),
                ExistenceRequirement::AllowDeath,
            )?);

            T::OrphanedFunds::on_unbalanced(T::Currency::withdraw(
                &account,
                fund.raised,
                WithdrawReasons::from(WithdrawReason::Transfer),
                ExistenceRequirement::AllowDeath
            )?);

            Self::crowdfund_kill(index);
            <Funds<T>>::remove(index);

            Self::deposit_event(RawEvent::Dissolved(index, now));
        }

        // fn on_finalize(n: T::BlockNumber)
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

    pub fn id_from_index(index: FundIndex) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"ex/cfund");
        buf.extend_from_slice(&index.to_le_bytes()[..]);

        CHILD_STORAGE_KEY_PREFIX
            .into_iter()
            .chain(b"default:")
            .chain(Blake2Hasher::hash(&buf[..]).as_ref().into_iter())
            .cloned()
            .collect()
    }

    pub fn contribution_put(index: FundIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
        let id = Self::id_from_index(index);
        who.using_encoded(|b| child::put(id.as_ref(), b, &balance));
    }

    pub fn contribution_get(index: FundIndex, who: &T::AccountId) -> BalanceOf<T> {
        let id = Self::id_from_index(index);
        who.using_encoded(|b| child::get_or_default::<BalanceOf<T>>(id.as_ref(), b))
    }

    pub fn contribution_kill(index: FundIndex, who: &T::AccountId) {
        let id = Self::id_from_index(index);
        who.using_encoded(|b| child::kill(id.as_ref(), b));
    }

    pub fn crowdfund_kill(index: FundIndex) {
        let id = Self::id_from_index(index);
        child::kill_storage(id.as_ref());
    }
}
