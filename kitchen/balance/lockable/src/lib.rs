/// staking/lib.rs simplified
use support::{
    decl_event, decl_module, decl_storage,
    dispatch::Result,
    ensure,
    traits::{
        Currency, Get, Imbalance, LockIdentifier, LockableCurrency, OnDilution, OnFreeBalanceZero,
        OnUnbalanced, ReservableCurrency, Time, WithdrawReason, WithdrawReasons,
    },
    StorageMap, StorageValue,
};
use system::ensure_signed;

const EXAMPLE_ID: LockIdentifier = *b"example ";

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

pub trait Trait: system::Trait {
    /// The lockable currency type
    type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

    /// The overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Some tokens minted
    type OnRewardMinted: OnDilution<BalanceOf<Self>>;

    /// Handler for the unbalanced increment when rewarding a staker
    type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

    /// Handler for the unbalanced reduction when slashing a staker
    type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;

    /// Period for Single Lock Invocation (could be a voting or application period for proposals)
    type LockPeriod: Get<Self::BlockNumber>;
}

decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        <T as system::Trait>::AccountId
    {   
        Locked(AccountId, Balance),
        ExtendedLock(AccountId, Balance),
        Unlocked(AccountId),
        Burned(AccountId, Balance),
        Minted(AccountId, Balance),
        Diluted(Balance),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        const LockPeriod: T::BlockNumber = T::LockPeriod::get();

        fn lock_capital(origin, amount: BalanceOf<T>) -> Result {
            let user = ensure_signed(origin)?;

            T::Currency::set_lock(
                EXAMPLE_ID,
                user.clone(),
                amount,
                T::LockPeriod::get(),
                WithdrawReasons::except(WithdrawReason::TransactionPayment),
                // TODO: check out https://crates.parity.io/srml_support/traits/struct.WithdrawReasons.html
            );

            Self::deposit_event(RawEvent::Locked(user, amount));
            Ok(())
        }

        fn extend_lock(origin, amount: BalanceOf<T>) -> Result {
            let user = ensure_signed(origin)?;

            T::Currency::extend_lock(
                EXAMPLE_ID,
                user.clone(),
                amount,
                T::LockPeriod::get(),
                WithdrawReasons::except(WithdrawReason::TransactionPayment),
            );

            Self::deposit_event(RawEvent::Unlocked(user, amount));
            Ok(())
        }

        fn unlock_all(origin, amount: BalanceOf<T>) -> Result {
            let user = ensure_signed(origin)?;

            T::Currency::remove_lock(EXAMPLE_ID, user.clone());

            Self::deposit_event(RawEvent::Unlocked(user));
            Ok(())
        }

        // use dilution and imbalances types
    }
}
