#![cfg_attr(not(feature = "std"), no_std)]

//! A simple pallet with two storage values. The pallet itself does not teach any new concepts.
//! Rather we use this pallet as demonstration case as we demonstrate custom runtime APIs.
//! This pallet supports a runtime API which will allow querying the runtime for the sum of
//! the two storage items.

use frame_support::{decl_event, decl_module, decl_storage, dispatch};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

/// The module's configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as SumStorage {
		Thing1 get(fn thing1): u32;
		Thing2 get(fn thing2): u32;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Sets the first simple storage value
		#[weight = 10_000]
		pub fn set_thing_1(origin, val: u32) -> dispatch::DispatchResult {
			let _ = ensure_signed(origin)?;

			Thing1::put(val);

			Self::deposit_event(Event::ValueSet(1, val));
			Ok(())
		}

		/// Sets the second stored value
		#[weight = 10_000]
		pub fn set_thing_2(origin, val: u32) -> dispatch::DispatchResult {
			let _ = ensure_signed(origin)?;

			Thing2::put(val);

			Self::deposit_event(Event::ValueSet(2, val));
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	pub fn get_sum() -> u32 {
		Thing1::get() + Thing2::get()
	}
}

decl_event!(
	pub enum Event {
		ValueSet(u32, u32),
	}
);
