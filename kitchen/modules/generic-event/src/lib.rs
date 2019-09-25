#![cfg_attr(not(feature = "std"), no_std)]

/// Event generic over multiple types (u32, AccountId)
use support::{decl_event, decl_module, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn do_something(origin, input: u32) -> Result {
            let user = ensure_signed(origin)?;

            // could do something with the input here instead
            let new_number = input;

            Self::deposit_event(RawEvent::EmitInput(user, new_number));
            Ok(())
        }
    }
}

// AccountId, u32 both are inputs `=>` generic event declaration
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        EmitInput(AccountId, u32),
    }
);
