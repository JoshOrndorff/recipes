//! Demonstration of Event variants that use type(s) from the pallet's configuration trait

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some input was sent
		EmitInput(T::AccountId, u32),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A simple call that does little more than emit an event
		#[pallet::weight(10_000)]
		pub fn do_something(origin: OriginFor<T>, input: u32) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// could do something with the input here instead
			let new_number = input;

			Self::deposit_event(Event::EmitInput(user, new_number));
			Ok(().into())
		}
	}
}
