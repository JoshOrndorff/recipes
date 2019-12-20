// imbalance rustdocs: https://substrate.dev/rustdocs/master/frame_support/traits/trait.Imbalance.html
// WARNING: never use this code in production (for demonstration/teaching purposes only)
// it only checks for signed extrinsics to enable arbitrary minting/slashing!!!
use support::traits::{
	Currency, OnUnbalanced, Imbalance, ReservableCurrency,
};
use support::{decl_event, decl_module};
use system::ensure_signed;

// balance type using reservable currency type
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

pub trait Trait: system::Trait + Sized {
    // overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Currency type for this pallet.
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    /// Handler for the unbalanced increment when rewarding (minting rewards)
    type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

    /// Handler for the unbalanced decrement when slashing (burning collateral)
    type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        SlashFunds(AccountId, Balance, BlockNumber),
        RewardFunds(AccountId, Balance, BlockNumber),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn slash_funds(origin, to_punish: T::AccountId, collateral: BalanceOf<T>) {
            let _ = ensure_signed(origin)?;

            let imbalance = T::Currency::slash_reserved(&to_punish, collateral.clone()).0;
            T::Slash::on_unbalanced(imbalance);

            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::SlashFunds(to_punish, collateral, now));
        }

        // see reward_validator of staking/lib.rs for more details
        pub fn reward_funds(origin, to_reward: T::AccountId, reward: BalanceOf<T>) {
            let _ = ensure_signed(origin)?;

            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

            let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
            total_imbalance.maybe_subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::RewardFunds(to_reward, reward, now));
        }
    }
}
