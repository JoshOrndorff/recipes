//! A pallet to demonstrate the `LockableCurrency` trait
//! borrows collateral locking logic from pallet_staking

use frame_support::{
	decl_event, decl_module,
	dispatch::DispatchResult,
	traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReason, WithdrawReasons},
};
use frame_system::{self as system, ensure_signed};

const EXAMPLE_ID: LockIdentifier = *b"example ";

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait {
	/// The lockable currency type
	type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

	/// The overarching event type
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
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
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Locks the specified amount of tokens from the caller
		#[weight = 10_000]
		fn lock_capital(origin, amount: BalanceOf<T>) -> DispatchResult {
			let user = ensure_signed(origin)?;

			T::Currency::set_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::except(WithdrawReason::TransactionPayment),
			);

			Self::deposit_event(RawEvent::Locked(user, amount));
			Ok(())
		}

		/// Extends the lock period
		#[weight = 10_000]
		fn extend_lock(origin, amount: BalanceOf<T>) -> DispatchResult {
			let user = ensure_signed(origin)?;

			T::Currency::extend_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::except(WithdrawReason::TransactionPayment),
			);

			Self::deposit_event(RawEvent::ExtendedLock(user, amount));
			Ok(())
		}

		/// Releases all locked tokens
		#[weight = 10_000]
		fn unlock_all(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			T::Currency::remove_lock(EXAMPLE_ID, &user);

			Self::deposit_event(RawEvent::Unlocked(user));
			Ok(())
		}

		// use dilution and imbalances types
	}
}
