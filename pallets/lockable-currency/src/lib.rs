//! A pallet to demonstrate the `LockableCurrency` trait
//! borrows collateral locking logic from pallet_staking
#![allow(clippy::unused_unit)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	const EXAMPLE_ID: LockIdentifier = *b"example ";

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The lockable currency type
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		/// The overarching event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Locked(T::AccountId, BalanceOf<T>),
		ExtendedLock(T::AccountId, BalanceOf<T>),
		Unlocked(T::AccountId),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Locks the specified amount of tokens from the caller
		#[pallet::weight(10_000)]
		fn lock_capital(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			T::Currency::set_lock(EXAMPLE_ID, &user, amount, WithdrawReasons::all());

			Self::deposit_event(Event::Locked(user, amount));
			Ok(().into())
		}

		/// Extends the lock period
		#[pallet::weight(10_000)]
		fn extend_lock(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			T::Currency::extend_lock(EXAMPLE_ID, &user, amount, WithdrawReasons::all());

			Self::deposit_event(Event::ExtendedLock(user, amount));
			Ok(().into())
		}

		/// Releases all locked tokens
		#[pallet::weight(10_000)]
		fn unlock_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			T::Currency::remove_lock(EXAMPLE_ID, &user);

			Self::deposit_event(Event::Unlocked(user));
			Ok(().into())
		}
	}
}
