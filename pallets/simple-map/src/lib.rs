//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as SimpleMap {
		SimpleMap get(fn simple_map): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// A user has set their entry
		EntrySet(AccountId, u32),

		/// A user has read their entry, leaving it in storage
		EntryGot(AccountId, u32),

		/// A user has read their entry, removing it from storage
		EntryTaken(AccountId, u32),

		/// A user has read their entry, incremented it, and written the new entry to storage
		/// Parameters are (user, old_entry, new_entry)
		EntryIncreased(AccountId, u32, u32),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The requested user has not stored a value yet
		NoValueStored,

		/// The value cannot be incremented further because it has reached the maximum allowed value
		MaxValueReached,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		/// Set the value stored at a particular key
		#[weight = 10_000]
		fn set_single_entry(origin, entry: u32) -> DispatchResult {
			// A user can only set their own entry
			let user = ensure_signed(origin)?;

			<SimpleMap<T>>::insert(&user, entry);

			Self::deposit_event(RawEvent::EntrySet(user, entry));
			Ok(())
		}

		/// Read the value stored at a particular key and emit it in an event
		#[weight = 10_000]
		fn get_single_entry(origin, account: T::AccountId) -> DispatchResult {
			// Any user can get any other user's entry
			let getter = ensure_signed(origin)?;

			ensure!(<SimpleMap<T>>::contains_key(&account), Error::<T>::NoValueStored);
			let entry = <SimpleMap<T>>::get(account);
			Self::deposit_event(RawEvent::EntryGot(getter, entry));
			Ok(())
		}

		/// Read the value stored at a particular key, while removing it from the map.
		/// Also emit the read value in an event
		#[weight = 10_000]
		fn take_single_entry(origin) -> DispatchResult {
			// A user can only take (delete) their own entry
			let user = ensure_signed(origin)?;

			ensure!(<SimpleMap<T>>::contains_key(&user), Error::<T>::NoValueStored);
			let entry = <SimpleMap<T>>::take(&user);
			Self::deposit_event(RawEvent::EntryTaken(user, entry));
			Ok(())
		}

		/// Increase the value associated with a particular key
		#[weight = 10_000]
		fn increase_single_entry(origin, add_this_val: u32) -> DispatchResult {
			// A user can only mutate their own entry
			let user = ensure_signed(origin)?;

			ensure!(<SimpleMap<T>>::contains_key(&user), Error::<T>::NoValueStored);
			let original_value = <SimpleMap<T>>::get(&user);

			let new_value = original_value.checked_add(add_this_val).ok_or(Error::<T>::MaxValueReached)?;
			<SimpleMap<T>>::insert(&user, new_value);

			Self::deposit_event(RawEvent::EntryIncreased(user, original_value, new_value));

			Ok(())
		}
	}
}
