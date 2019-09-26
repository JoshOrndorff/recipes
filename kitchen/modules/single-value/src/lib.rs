#![cfg_attr(not(feature = "std"), no_std)]

/// Single Value Storage
use support::{decl_module, decl_storage, dispatch::Result, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as SingleValue {
        MyValue: u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin, value: u32) -> Result {
            // check sender signature to verify permissions
            let _ = ensure_signed(origin)?;
            <MyValue>::put(value);
            Ok(())
        }
    }
}
