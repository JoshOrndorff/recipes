#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

//! Simple Token Transfer
//! 1. set total supply
//! 2. establish ownership upon configuration of circulating tokens
//! 3. coordinate token transfers with the runtime functions

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Token was initialized by user
		Initialized(T::AccountId),
		/// Tokens successfully transferred between users
		Transfer(T::AccountId, T::AccountId, u64), // (from, to, value)
	}

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type Balances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::type_value]
	pub(super) fn TotalSupplyDefaultValue<T: Config>() -> u64 {
		21000000
	}

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub(super) type TotalSupply<T: Config> =
		StorageValue<_, u64, ValueQuery, TotalSupplyDefaultValue<T>>;

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to initialize the token after it had already been initialized.
		AlreadyInitialized,
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initialize the token
		/// transfers the total_supply amout to the caller
		#[pallet::weight(10_000)]
		pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(!Self::is_init(), <Error<T>>::AlreadyInitialized);

			<Balances<T>>::insert(sender, Self::total_supply());

			Init::<T>::put(true);
			Ok(().into())
		}

		/// Transfer tokens from one account to another
		#[pallet::weight(10_000)]
		pub fn transfer(
			_origin: OriginFor<T>,
			to: T::AccountId,
			value: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances
			let updated_from_balance = sender_balance
				.checked_sub(value)
				.ok_or(<Error<T>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance
				.checked_add(value)
				.expect("Entire supply fits in u64; qed");

			// Write new balances to storage
			<Balances<T>>::insert(&sender, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(Event::Transfer(sender, to, value));
			Ok(().into())
		}
	}
}
