//! A pallet to demonstrate the `ReservableCurrency` trait
//! borrows collateral locking logic from pallet_treasury
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_event, decl_module,
	dispatch::DispatchResult,
	traits::{Currency, ExistenceRequirement::AllowDeath, ReservableCurrency},
};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

// balance type using reservable currency type
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: frame_system::Config + Sized {
	// overarching event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// Currency type for this pallet.
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
		BlockNumber = <T as frame_system::Config>::BlockNumber,
	{
		LockFunds(AccountId, Balance, BlockNumber),
		UnlockFunds(AccountId, Balance, BlockNumber),
		// sender, dest, amount, block number
		TransferFunds(AccountId, AccountId, Balance, BlockNumber),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Reserves the specified amount of funds from the caller
		#[weight = 10_000]
		pub fn reserve_funds(origin, amount: BalanceOf<T>) -> DispatchResult {
			let locker = ensure_signed(origin)?;

			T::Currency::reserve(&locker, amount)
					.map_err(|_| "locker can't afford to lock the amount requested")?;

			let now = <frame_system::Module<T>>::block_number();

			Self::deposit_event(RawEvent::LockFunds(locker, amount, now));
			Ok(())
		}

		/// Unreserves the specified amount of funds from the caller
		#[weight = 10_000]
		pub fn unreserve_funds(origin, amount: BalanceOf<T>) -> DispatchResult {
			let unlocker = ensure_signed(origin)?;

			T::Currency::unreserve(&unlocker, amount);
			// ReservableCurrency::unreserve does not fail (it will lock up as much as amount)

			let now = <frame_system::Module<T>>::block_number();

			Self::deposit_event(RawEvent::UnlockFunds(unlocker, amount, now));
			Ok(())
		}

		/// Transfers funds. Essentially a wrapper around the Currency's own transfer method
		#[weight = 10_000]
		pub fn transfer_funds(origin, dest: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			T::Currency::transfer(&sender, &dest, amount, AllowDeath)?;

			let now = <frame_system::Module<T>>::block_number();

			Self::deposit_event(RawEvent::TransferFunds(sender, dest, amount, now));
			Ok(())
		}

		/// Atomically unreserves funds and and transfers them.
		/// might be useful in closed economic systems
		#[weight = 10_000]
		pub fn unreserve_and_transfer(
			origin,
			to_punish: T::AccountId,
			dest: T::AccountId,
			collateral: BalanceOf<T>
		) -> DispatchResult {
			let _ = ensure_signed(origin)?; // dangerous because can be called with any signature (so dont do this in practice ever!)

						// If collateral is bigger than to_punish's reserved_balance, store what's left in overdraft.
			let overdraft = T::Currency::unreserve(&to_punish, collateral);

			T::Currency::transfer(&to_punish, &dest, collateral - overdraft, AllowDeath)?;

			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::TransferFunds(to_punish, dest, collateral - overdraft, now));

			Ok(())
		}
	}
}
