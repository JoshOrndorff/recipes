#![cfg_attr(not(feature = "std"), no_std)]

/// List via Maps
/// Substrate does not natively support a list type since it may encourage 
/// dangerous habits. Unless explicitly guarded against, a list will add 
/// unbounded `O(n)` complexity to an operation that will only charge `O(1)` 
/// fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)). 
/// This opens an economic attack vector on your chain.

use support::{ensure, decl_module, decl_storage, decl_event, StorageValue, StorageMap, EnumerableStorageMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		TheList get(the_list): map u32 => T::AccountId;
		TheCounter get(the_counter): u32;

		LinkedList get(linked_list): linked_map u32 => T::AccountId;
		LinkedCounter get(linked_counter): u32;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// member with AccountId added
		MemberAdded(AccountId),
		// member with AccountId removed
		MemberRemoved(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// initialize the default event for this module
		fn deposit_event<T>() = default;

		fn add_member(origin) -> Result {
			let who = ensure_signed(origin)?;

			// increment the counter
			<TheCounter>::mutate(|count| *count + 1);

			// add member at the largest_index
			let largest_index = <TheCounter>::get();
			<TheList<T>>::insert(largest_index, who.clone());

			Self::deposit_event(RawEvent::MemberAdded(who));

			Ok(())
		} 

		fn remove_member_unbounded(origin, index: u32) -> Result {
			let who = ensure_signed(origin)?;

			// verify existence
			ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
			let removed_member = <TheList<T>>::get(index);
			<TheList<T>>::remove(index);

			Self::deposit_event(RawEvent::MemberRemoved(removed_member));

			Ok(())
		}

		fn remove_member_ordered(origin, index: u32) -> Result {
			let who = ensure_signed(origin)?;

			ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");

			let largest_index = <TheCounter>::get();
			let member_to_remove = <TheList<T>>::take(index);
			// swap
			if index != largest_index {
				let temp = <TheList<T>>::take(largest_index);
				<TheList<T>>::insert(index, temp);
				<TheList<T>>::insert(largest_index, member_to_remove.clone());
			}
			// pop
			<TheList<T>>::remove(largest_index);
			<TheCounter>::mutate(|count| *count - 1);

			Self::deposit_event(RawEvent::MemberRemoved(member_to_remove.clone()));

			Ok(())
		}

		fn add_member_linked(origin) -> Result {
			let who = ensure_signed(origin)?;

			// increment the counter
			<LinkedCounter>::mutate(|count| *count + 1);

			// add member at the largest_index
			let largest_index = <LinkedCounter>::get();
			<TheList<T>>::insert(largest_index, who.clone());

			Ok(())
		}

		fn remove_member_linked(origin, index: u32) -> Result {
			let who = ensure_signed(origin)?;

			ensure!(<LinkedList<T>>::exists(index), "A member does not exist at this index");

			let head_index = <LinkedList<T>>::head().unwrap();
			let member_to_remove = <LinkedList<T>>::take(index);
			let head_member = <LinkedList<T>>::get(head_index);
			<LinkedList<T>>::insert(index, head_member);
			<LinkedList<T>>::remove(head_index);

			Ok(())
		}
	}
}