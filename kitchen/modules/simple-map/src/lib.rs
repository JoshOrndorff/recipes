#![cfg_attr(not(feature = "std"), no_std)]

// Simple Storage Map
// https://crates.parity.io/srml_support/storage/trait.StorageMap.html
use support::{ensure, decl_module, decl_storage, decl_event, StorageMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as SimpleMap {
		SimpleMap get(simple_map): map T::AccountId => u32;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// insert new entry
        EntrySet(AccountId, u32),
        EntryGot(AccountId, u32),
        EntryTook(AccountId, u32),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        fn set_single_entry(origin, entry: u32) -> Result {
            // only a user can set their entry
            let user = ensure_signed(origin)?;

            <SimpleMap<T>>::insert(user.clone(), entry);

            Self::deposit_event(RawEvent::EntrySet(user, entry));
            Ok(())
        }

        fn get_single_entry(origin, account: T::AccountId) -> Result {
            // anyone (signed extrinsic) can get an entry
            let getter = ensure_signed(origin)?;

            ensure!(<SimpleMap<T>>::exists(account.clone()), "an entry does not exist for this user");
            let entry = <SimpleMap<T>>::get(account);
            Self::deposit_event(RawEvent::EntryGot(getter, entry));
            Ok(())
        }

        fn take_single_entry(origin) -> Result {
            // only the user can take their own entry
            let user = ensure_signed(origin)?;

            ensure!(<SimpleMap<T>>::exists(user.clone()), "an entry does not exist for this user");
            let entry = <SimpleMap<T>>::take(user.clone());
            // ensure!(!<SimpleMap<T>>::exists(user.clone()), "the take did not succeed");
            Self::deposit_event(RawEvent::EntryTook(user, entry));
            Ok(())
        }

        fn mutate_single_entry(origin, add_this_val: u32) -> Result {
            // only the user can mutate their own entry
            let user = ensure_signed(origin)?;

            // adds `add_this_val` to the entry
            <SimpleMap<T>>::mutate(user.clone(), |entry| *entry += add_this_val);
            // warning: does NOT check for overflow

            Ok(())
        }

        fn compare_and_swap_single_entry(origin, old_entry: u32, new_entry: u32) -> Result {
            // only a user that knows their previous entry can set the new entry
            let user = ensure_signed(origin)?;

            // compare
            ensure!(old_entry == <SimpleMap<T>>::get(user.clone()), "cas failed because old_entry == existing_entry");
            // and swap
            <SimpleMap<T>>::insert(user, new_entry);
            Ok(())
        }
	}
}