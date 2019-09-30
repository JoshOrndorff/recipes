#![cfg_attr(not(feature = "std"), no_std)]

/// Simple Event (not generic over types)
use support::{decl_event, decl_module, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn do_something(origin, input: u32) -> Result {
            let _ = ensure_signed(origin)?;

            // could do something with the input here instead
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
