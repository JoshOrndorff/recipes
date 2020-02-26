#![cfg_attr(not(feature = "std"), no_std)]

// Simple Storage Map
// https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageMap.html
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as SimpleMap {
		SimpleMap get(fn simple_map): map hasher(blake2_256) T::AccountId => u32;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		// insert new entry
		EntrySet(AccountId, u32),
		EntryGot(AccountId, u32),
		EntryTook(AccountId, u32),
		// increase (old_entry, new_entry) (by logic in increase which adds the input param)
		IncreaseEntry(u32, u32),
		// CompareAndSwap (old_entry, new_entry)
		CAS(u32, u32),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn set_single_entry(origin, entry: u32) -> DispatchResult {
			// only a user can set their entry
			let user = ensure_signed(origin)?;

			<SimpleMap<T>>::insert(user.clone(), entry);

			Self::deposit_event(RawEvent::EntrySet(user, entry));
			Ok(())
		}

		fn get_single_entry(origin, account: T::AccountId) -> DispatchResult {
			// anyone (signed extrinsic) can get an entry
			let getter = ensure_signed(origin)?;

			ensure!(<SimpleMap<T>>::contains_key(account.clone()), "an entry does not exist for this user");
			let entry = <SimpleMap<T>>::get(account);
			Self::deposit_event(RawEvent::EntryGot(getter, entry));
			Ok(())
		}

		fn take_single_entry(origin) -> DispatchResult {
			// only the user can take their own entry
			let user = ensure_signed(origin)?;

			ensure!(<SimpleMap<T>>::contains_key(user.clone()), "an entry does not exist for this user");
			let entry = <SimpleMap<T>>::take(user.clone());
			Self::deposit_event(RawEvent::EntryTook(user, entry));
			Ok(())
		}

		fn increase_single_entry(origin, add_this_val: u32) -> DispatchResult {
			// only the user can mutate their own entry
			let user = ensure_signed(origin)?;
			let original_value = <SimpleMap<T>>::get(&user);
			let new_value = original_value.checked_add(add_this_val).ok_or("value overflowed")?;
			<SimpleMap<T>>::insert(user, new_value);

			Self::deposit_event(RawEvent::IncreaseEntry(original_value, new_value));

			Ok(())
		}

		fn compare_and_swap_single_entry(origin, old_entry: u32, new_entry: u32) -> DispatchResult {
			// only a user that knows their previous entry can set the new entry
			let user = ensure_signed(origin)?;

			// compare
			ensure!(old_entry == <SimpleMap<T>>::get(user.clone()), "cas failed bc old_entry inputted by user != existing_entry");
			// and swap
			<SimpleMap<T>>::insert(user, new_entry);
			Self::deposit_event(RawEvent::CAS(old_entry, new_entry));
			Ok(())
		}
	}
}
