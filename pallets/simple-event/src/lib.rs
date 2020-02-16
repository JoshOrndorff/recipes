#![cfg_attr(not(feature = "std"), no_std)]

/// Simple Event (not generic over types)
use frame_support::{decl_event, decl_module, dispatch::DispatchResult};
use system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

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
