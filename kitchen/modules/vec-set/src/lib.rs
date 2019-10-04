#![cfg_attr(not(feature = "std"), no_std)]

// demonstrates how to use append instead of mutate
// https://crates.parity.io/srml_support/storage/trait.StorageValue.html#tymethod.append
use support::{ensure, decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;
use rstd::prelude::*;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
        Members get(members): Vec<T::AccountId>;
	    CurrentValues get(current_values): Vec<u32>;
        NewValues get(new_values): Vec<u32>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        // added member
        MemberAdded(AccountId),
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

        fn add_member(origin) -> Result {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_member(&new_member), "must not be a member to be added");
            let mut members = <Members<T>>::get();
            members.append(&mut vec![new_member.clone()]);
            Self::deposit_event(RawEvent::MemberAdded(new_member));
            Ok(())
        }

        fn member_removed(origin) -> Result {
            let old_member = ensure_signed(origin)?;
            ensure!(Self::is_member(&old_member), "must be a member in order to exit");
            <Members<T>>::mutate(|v| v.retain(|i| i != &old_member));
            Ok(())
        }
        // also see `append_or_insert`, `append_or_put` in srml-elections/phragmen, democracy
    }
}

impl<T: Trait> Module<T> {
    pub fn is_member(who: &T::AccountId) -> bool {
        <Members<T>>::get().contains(who)
    } // TODO: child trie for more efficient membership storage structure
}