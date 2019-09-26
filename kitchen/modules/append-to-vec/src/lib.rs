#![cfg_attr(not(feature = "std"), no_std)]

// demonstrates how to use append instead of mutate
// https://crates.parity.io/srml_support/storage/trait.StorageValue.html#tymethod.append
use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
	    CurrentValues get(current_values): Vec<u32>;
        NewValues get(new_values): Vec<u32>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        // mutate to append
        MutateToAppend(AccountId),
        // append
        AppendVec(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        // don't do this
        // (unless appending new entries AND mutating existing entries)
        fn mutate_to_append(origin) -> Result {
            let user = ensure_signed(origin)?;

            // this decodes the existing vec, appends the new values, and re-encodes the whole thing
            <CurrentValues>::mutate(|v| v.extend_from_slice(&Self::new_values()));
            Self::deposit_event(RawEvent::MutateToAppend(user));
            Ok(())
        }

        // do this instead
        fn append_new_entries(origin) -> Result {
            let user = ensure_signed(origin)?;

            // this encodes the new values and appends them to the already encoded existing evc
            let mut current_values = Self::current_values();
            current_values.append(&mut Self::new_values());
            Self::deposit_event(RawEvent::AppendVec(user));
            Ok(())
        } // more examples in srml/elections-phragmen        
    }
}// todo: append_or_insert, append_or_put