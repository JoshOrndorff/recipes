//! Double Map Example with remove_prefix
//! `double_map` maps two keys to a single value.
//! the first key might be a group identifier
//! the second key might be a unique identifier
//! `remove_prefix` enables clean removal of all values with the group identifier

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	ensure,
	storage::{StorageDoubleMap, StorageMap, StorageValue},
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

pub type GroupIndex = u32; // this is Encode (which is necessary for double_map)

decl_storage! {
	trait Store for Module<T: Config> as Dmap {
		/// Member score (double map)
		MemberScore get(fn member_score):
			double_map hasher(blake2_128_concat) GroupIndex, hasher(blake2_128_concat) T::AccountId => u32;
		/// Get group ID for member
		GroupMembership get(fn group_membership): map hasher(blake2_128_concat) T::AccountId => GroupIndex;
		/// For fast membership checks, see check-membership recipe for more details
		AllMembers get(fn all_members): Vec<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// New member for `AllMembers` group
		NewMember(AccountId),
		/// Put member score (id, index, score)
		MemberJoinsGroup(AccountId, GroupIndex, u32),
		/// Remove a single member with AccountId
		RemoveMember(AccountId),
		/// Remove all members with GroupId
		RemoveGroup(GroupIndex),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Join the `AllMembers` vec before joining a group
		#[weight = 10_000]
		fn join_all_members(origin) -> DispatchResult {
			let new_member = ensure_signed(origin)?;
			ensure!(!Self::is_member(&new_member), "already a member, can't join");
			<AllMembers<T>>::append(&new_member);

			Self::deposit_event(RawEvent::NewMember(new_member));
			Ok(())
		}

		/// Put MemberScore (for testing purposes)
		#[weight = 10_000]
		fn join_a_group(origin, index: GroupIndex, score: u32) -> DispatchResult {
			let member = ensure_signed(origin)?;
			ensure!(Self::is_member(&member), "not a member, can't remove");
			<MemberScore<T>>::insert(&index, &member, score);
			<GroupMembership<T>>::insert(&member, &index);

			Self::deposit_event(RawEvent::MemberJoinsGroup(member, index, score));
			Ok(())
		}

		/// Remove a member
		#[weight = 10_000]
		fn remove_member(origin) -> DispatchResult {
			let member_to_remove = ensure_signed(origin)?;
			ensure!(Self::is_member(&member_to_remove), "not a member, can't remove");
			let group_id = <GroupMembership<T>>::take(member_to_remove.clone());
			<MemberScore<T>>::remove(&group_id, &member_to_remove);

			Self::deposit_event(RawEvent::RemoveMember(member_to_remove));
			Ok(())
		}

		/// Remove group score
		#[weight = 10_000]
		fn remove_group_score(origin, group: GroupIndex) -> DispatchResult {
			let member = ensure_signed(origin)?;

			let group_id = <GroupMembership<T>>::get(member);
			// check that the member is in the group
			ensure!(group_id == group, "member isn't in the group, can't remove it");

			// remove all group members from MemberScore at once
			<MemberScore<T>>::remove_prefix(&group_id);

			Self::deposit_event(RawEvent::RemoveGroup(group_id));
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	// for fast membership checks (see check-membership recipe for more details)
	fn is_member(who: &T::AccountId) -> bool {
		Self::all_members().contains(who)
	}
}
