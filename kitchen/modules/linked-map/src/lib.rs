#![cfg_attr(not(feature = "std"), no_std)]

/// List via Maps
/// Substrate does not natively support a list type since it may encourage
/// dangerous habits. Unless explicitly guarded against, a list will add
/// unbounded `O(n)` complexity to an operation that will only charge `O(1)`
/// fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)).
/// This opens an economic attack vector on your chain.

use support::{ensure, decl_module, decl_storage, decl_event, StorageValue, StorageMap, StorageLinkedMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as List {
		TheList get(fn the_list): map u32 => T::AccountId;
		TheCounter get(fn the_counter): u32;

		LinkedList get(fn linked_list): linked_map u32 => T::AccountId;
		LinkedCounter get(fn linked_counter): u32;
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
		fn deposit_event() = default;

		fn add_member(origin) -> Result {
			let who = ensure_signed(origin)?;

			// increment the counter
			let new_count = <TheCounter>::get() + 1;

			// add member at the largest_index
			<TheList<T>>::insert(new_count, who.clone());
			// incremement counter
			<TheCounter>::put(new_count);

			// (keep linked list synced)
			<LinkedList<T>>::insert(new_count, who.clone());
			// increment the counter
			<LinkedCounter>::put(new_count);

			Self::deposit_event(RawEvent::MemberAdded(who));

			Ok(())
		}

		// worst option
		// -- only works if the list is *unbounded*
		fn remove_member_unbounded(origin, index: u32) -> Result {
			let _ = ensure_signed(origin)?;

			// verify existence
			ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
			// for event emission (could be removed to minimize calls)
			let removed_member = <TheList<T>>::get(index);
			<TheList<T>>::remove(index);
			// assumes that we do not need to adjust the list because every add just increments counter

			Self::deposit_event(RawEvent::MemberRemoved(removed_member));

			Ok(())
		}

		// ok option
		// swap and pop
		// -- better than `remove_member_unbounded`
		// -- this pattern becomes unwieldy fast!
		fn remove_member_bounded(origin, index: u32) -> Result {
			let _ = ensure_signed(origin)?;

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
			<TheCounter>::put(largest_index - 1);

			Self::deposit_event(RawEvent::MemberRemoved(member_to_remove.clone()));

			Ok(())
		}

		// best option (atm)
		// this uses the enumerable storage map to simplify `swap and pop`
		// should be generally preferred
		fn remove_member_linked(origin, index: u32) -> Result {
			let _ = ensure_signed(origin)?;

			ensure!(<LinkedList<T>>::exists(index), "A member does not exist at this index");

			let head_index = <LinkedList<T>>::head().unwrap();
			let member_to_remove = <LinkedList<T>>::take(index);
			let head_member = <LinkedList<T>>::take(head_index);
			<LinkedList<T>>::insert(index, head_member);
			<LinkedList<T>>::insert(head_index, member_to_remove);
			<LinkedList<T>>::remove(head_index);

			Ok(())
		}
	}
}
