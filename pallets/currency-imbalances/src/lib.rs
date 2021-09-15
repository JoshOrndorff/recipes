#![cfg_attr(not(feature = "std"), no_std)]

//! A Pallet to demonstrate using currency imbalances
//!
//! WARNING: never use this code in production (for demonstration/teaching purposes only)
//! it only checks for signed extrinsics to enable arbitrary minting/slashing!!!

use frame_support::{
	decl_event, decl_module,
	traits::{Currency, Imbalance, OnUnbalanced, ReservableCurrency},
};
use frame_system::ensure_signed;

// balance type using reservable currency type
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

pub trait Config: frame_system::Config + Sized {
	/// The overarching event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

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
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
		BlockNumber = <T as frame_system::Config>::BlockNumber,
	{
		SlashFunds(AccountId, Balance, BlockNumber),
		RewardFunds(AccountId, Balance, BlockNumber),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Slashes the specified amount of funds from the specified account
		#[weight = 10_000]
		pub fn slash_funds(origin, to_punish: T::AccountId, collateral: BalanceOf<T>) {
			let _ = ensure_signed(origin)?;

			let imbalance = T::Currency::slash_reserved(&to_punish, collateral).0;
			T::Slash::on_unbalanced(imbalance);

			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::SlashFunds(to_punish, collateral, now));
		}

		/// Awards the specified amount of funds to the specified account
		#[weight = 10_000]
		pub fn reward_funds(origin, to_reward: T::AccountId, reward: BalanceOf<T>) {
			let _ = ensure_signed(origin)?;

			let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

			let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
			total_imbalance.maybe_subsume(r);
			T::Reward::on_unbalanced(total_imbalance);

			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::RewardFunds(to_reward, reward, now));
		}
	}
}
