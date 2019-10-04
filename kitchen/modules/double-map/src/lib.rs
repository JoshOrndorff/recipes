#![cfg_attr(not(feature = "std"), no_std)]

// Double Map Example w/ remove_prefix
// https://crates.parity.io/srml_support/storage/trait.StorageDoubleMap.html
// `double_map` maps two keys to a single value. 
// the first key might be a group identifier
// the second key might be a unique identifier
// `remove_prefix` enables clean removal of all values with the group identifier 

use support::{
    ensure, decl_module, decl_storage, decl_event,
    storage::{StorageDoubleMap, StorageMap, StorageValue},
    dispatch::Result,
};
use system::ensure_signed;
use rstd::prelude::*;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub type GroupIndex = u32; // this is Encode (which is necessary for double_map)

decl_storage! {
	trait Store for Module<T: Trait> as Dmap {
        // member score (double map)
        MemberScore: double_map GroupIndex, twox_128(T::AccountId) => u32;
        // get group ID for member
        GroupMembership get(group_membership): map T::AccountId => GroupIndex;
        // for fast membership checks, see check-membership recipe for more details
        AllMembers get(all_members): Vec<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// remove a single member with AccountId
        RemoveSingleMember(AccountId),
        // remove all members with GroupId
        RemoveGroup(GroupIndex),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        fn remove_my_score(origin) -> Result {
            let member_to_remove = ensure_signed(origin)?;
            ensure!(Self::is_member(&member_to_remove), "not a member, can't remove");

            let group_id = <GroupMembership<T>>::get(member_to_remove.clone());
            <MemberScore<T>>::remove(&group_id, &member_to_remove);
            Self::deposit_event(RawEvent::RemoveSingleMember(member_to_remove));
            Ok(())
        }

        fn remove_group_score(origin, group: GroupIndex) -> Result {
            let member = ensure_signed(origin)?;

            let group_id = <GroupMembership<T>>::get(member);
            // check that the member is in the group (could be improved by requiring n-of-m member support)
            ensure!(group_id == group, "member isn't in the group, can't remove it");

            // allows us to remove all group members from MemberScore at once
            <MemberScore<T>>::remove_prefix(&group_id);

            Self::deposit_event(RawEvent::RemoveGroup(group_id));
            Ok(())
        }
	}
}

impl<T: Trait> Module<T> {
    // for fast membership checks (see check-membership recipe for more details)
    fn is_member(who: &T::AccountId) -> bool {
        <AllMembers<T>>::get().contains(who)
    }
}
