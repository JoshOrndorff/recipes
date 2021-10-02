//! A pallet to demonstrate the `ReservableCurrency` trait
//! borrows collateral locking logic from pallet_treasury
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::AllowDeath, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Currency type for this pallet.
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		LockFunds(T::AccountId, BalanceOf<T>, T::BlockNumber),
		UnlockFunds(T::AccountId, BalanceOf<T>, T::BlockNumber),
		// sender, dest, amount, block number
		TransferFunds(T::AccountId, T::AccountId, BalanceOf<T>, T::BlockNumber),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Reserves the specified amount of funds from the caller
		#[pallet::weight(10_000)]
		pub fn reserve_funds(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let locker = ensure_signed(origin)?;

			T::Currency::reserve(&locker, amount)
				.map_err(|_| "locker can't afford to lock the amount requested")?;

			let now = <frame_system::Module<T>>::block_number();

			Self::deposit_event(Event::LockFunds(locker, amount, now));
			Ok(().into())
		}

		/// Unreserves the specified amount of funds from the caller
		#[pallet::weight(10_000)]
		pub fn unreserve_funds(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let unlocker = ensure_signed(origin)?;

			T::Currency::unreserve(&unlocker, amount);
			// ReservableCurrency::unreserve does not fail (it will lock up as much as amount)

			let now = <frame_system::Module<T>>::block_number();

			Self::deposit_event(Event::UnlockFunds(unlocker, amount, now));
			Ok(().into())
		}

		/// Transfers funds. Essentially a wrapper around the Currency's own transfer method
		#[pallet::weight(10_000)]
		pub fn transfer_funds(
			origin: OriginFor<T>,
			dest: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			T::Currency::transfer(&sender, &dest, amount, AllowDeath)?;

			let now = <frame_system::Module<T>>::block_number();

			Self::deposit_event(Event::TransferFunds(sender, dest, amount, now));
			Ok(().into())
		}

		/// Atomically unreserves funds and and transfers them.
		/// might be useful in closed economic systems
		#[pallet::weight(10_000)]
		pub fn unreserve_and_transfer(
			origin: OriginFor<T>,
			to_punish: T::AccountId,
			dest: T::AccountId,
			collateral: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?; // dangerous because can be called with any signature (so dont do this in practice ever!)

			// If collateral is bigger than to_punish's reserved_balance, store what's left in overdraft.
			let overdraft = T::Currency::unreserve(&to_punish, collateral);

			T::Currency::transfer(&to_punish, &dest, collateral - overdraft, AllowDeath)?;

			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(Event::TransferFunds(
				to_punish,
				dest,
				collateral - overdraft,
				now,
			));

			Ok(().into())
		}
	}
}
