//! Demonstration of Event variants that use type(s) from the pallet's configuration trait

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, dispatch::DispatchResult};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// A simple call that does little more than emit an event
		#[weight = 10_000]
		fn do_something(origin, input: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// could do something with the input here instead
			let new_number = input;

			Self::deposit_event(RawEvent::EmitInput(user, new_number));
			Ok(())
		}
	}
}

// AccountId, u32 both are inputs `=>` declaration with `<T>`
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		/// Some input was sent
		EmitInput(AccountId, u32),
	}
);
