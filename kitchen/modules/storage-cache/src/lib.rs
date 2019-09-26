// storage cache example
// -- minimize calls to runtime storage

#![cfg_attr(not(feature = "std"), no_std)]

/// Single Value Storage
use support::{decl_module, decl_storage, dispatch::Result, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as StorageCache {
        CopyType get(copy_type): u32;
        NotCopyType get(not_copy_type): Vec<u32>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // don't do this
        fn no_cache_with_copy(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <CopyType>::get();
            let some_calculation = original_call + some_val; // doesn't check for overflow either!
            // this next storage call is unnecessary and is wasteful
            let unnecessary_call = <CopyType>::get();
            // should've just used first_call here because u32 is copy
            let another_calculation = some_calculation + unnecessary_call;
            <CopyType>::put(another_calculation);
            Ok(())
        }
        
        // do this instead
        fn cache_with_copy(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <CopyType>::get();
            let some_calculation = original_call + some_val; // doesn't check for overflow either!
            // uses the original_call because u32 is copy
            let another_calculation = some_calculation + original_call;
            <CopyType>::put(another_calculation);
            Ok(())
        }

        // dont do this
        fn no_cache_with_no_copy(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <NotCopyType>::get();
            // vector moved
            let vec_len = original_call.len();
            // this next storage call is unnecessary and is wasteful
            let unnecessary_call = <NotCopyType>::get();
            let mut new_vec = Vec::new();
            for i in 0..vec_len { // avoid iteration in the runtime...
                new_vec.push(some_val + unnecessary_call[i]);
            }
            <NotCopyType>::put(new_vec);
            Ok(())
        }

        // do this instead
        fn cache_with_no_copy(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <NotCopyType>::get();
            // save the future call to runtime storage
            let vec_len = original_call.len();
            let mut new_vec = Vec::new();
            for i in 0..vec_len { // avoid iteration in the runtime
                // use the same call as previous
                new_vec.push(some_val + original_call[i]);
            }
            <NotCopyType>::put(new_vec);
            Ok(())
        }

        // ---- for testing purposes ----
        fn set_value(origin, val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            <CopyType>::put(val);
            Ok(())
        }

        fn set_vector(origin, vec: Vec<u32>) -> Result {
            let _ = ensure_signed(origin)?;
            <NotCopyType>::put(vec);
            Ok(())
        }
    }
}
