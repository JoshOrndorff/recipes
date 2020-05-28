#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet that implements a storage set on top of a storage map and demonstrates performance
//! tradeoffs when using vec sets.

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_std::collections::btree_set::BTreeSet;
use account_set::AccountSet;

#[cfg(test)]
mod tests;


pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
		// The set of all members. The bool value is useless and will always be
		// true. It is necessary because the underlying storage can't distinguish
		// between 0-byte values and non-existant values so () can't be used.
		Members get(fn members): map hasher(blake2_128_concat) T::AccountId => bool;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		/// Added a member
		MemberAdded(AccountId),
		/// Removed a member
		MemberRemoved(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Cannot join as a member because you are already a member
		AlreadyMember,
		/// Cannot give up membership because you are not currently a member
		NotMember,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;

		/// Adds a member to the membership set
		#[weight = 10_000]
		fn add_member(origin) -> DispatchResult {
			let new_member = ensure_signed(origin)?;

			// We don't want to add duplicate members, so we check whether the potential new
			// member is already present in the list. Because the membership is stored as a hash
			// map this check is constant time O(1)
			ensure!(!Members::<T>::contains_key(&new_member), Error::<T>::AlreadyMember);

			// Insert the new member and emit the event
			Members::<T>::insert(&new_member, true);
			Self::deposit_event(RawEvent::MemberAdded(new_member));
			Ok(())
		}

		/// Removes a member.
		#[weight = 10_000]
		fn remove_member(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;

			ensure!(Members::<T>::contains_key(&old_member), Error::<T>::NotMember);

			Members::<T>::remove(&old_member);
			Self::deposit_event(RawEvent::MemberRemoved(old_member));
			Ok(())
		}

		// also see `append_or_insert`, `append_or_put` in pallet-elections/phragmen, democracy
	}
}

// impl<T: Trait> AccountSet for Module<T> {
// 	type AccountId = T::AccountId;
//
// 	fn accounts() -> BTreeSet<T::AccountId> {
// 		Self::members().into_iter().collect::<BTreeSet<_>>()
// 	}
// }
