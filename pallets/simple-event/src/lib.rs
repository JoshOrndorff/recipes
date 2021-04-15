//! Demonstration of Event variants that use only primative types
//! These events do not use types from the pallet's configuration trait

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// A simple call that does little more than emit an event
		#[weight = 10_000]
		fn do_something(origin, input: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// In practice, you could do some processing with the input here.
			let new_number = input;

			// emit event
			Self::deposit_event(Event::EmitInput(new_number));
			Ok(())
		}
	}
}

// uses u32 and not types from Trait so does not require `<T>`
decl_event!(
	pub enum Event {
		EmitInput(u32),
	}
);
