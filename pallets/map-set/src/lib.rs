#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet that implements a storage set on top of a storage map and demonstrates performance
//! tradeoffs when using vec sets.

use account_set::AccountSet;
use frame_support::storage::IterableStorageMap;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::prelude::*;

#[cfg(test)]
mod tests;

/// A maximum number of members. When membership reaches this number, no new members may join.
pub const MAX_MEMBERS: u32 = 16;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as MapSet {
		//Currently we map to '()' because '()' is not encoded anymore as 0 bytes and the underlying storage
		Members get(fn members): map hasher(blake2_128_concat) T::AccountId => ();
		// The total number of members stored in the map.
		// Because the map does not store its size internally, we must store it separately
		MemberCount: u32;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// Added a member
		MemberAdded(AccountId),
		/// Removed a member
		MemberRemoved(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Cannot join as a member because you are already a member
		AlreadyMember,
		/// Cannot give up membership because you are not currently a member
		NotMember,
		/// Cannot add another member because the limit is already reached
		MembershipLimitReached,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;

		/// Adds a member to the membership set
		#[weight = 10_000]
		fn add_member(origin) -> DispatchResult {
			let new_member = ensure_signed(origin)?;

			let member_count = MemberCount::get();
			ensure!(member_count < MAX_MEMBERS, Error::<T>::MembershipLimitReached);

			// We don't want to add duplicate members, so we check whether the potential new
			// member is already present in the list. Because the membership is stored as a hash
			// map this check is constant time O(1)
			ensure!(!Members::<T>::contains_key(&new_member), Error::<T>::AlreadyMember);

			// Insert the new member and emit the event
			Members::<T>::insert(&new_member, ());
			MemberCount::put(member_count + 1); // overflow check not necessary because of maximum
			Self::deposit_event(RawEvent::MemberAdded(new_member));
			Ok(())
		}

		/// Removes a member.
		#[weight = 10_000]
		fn remove_member(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;

			ensure!(Members::<T>::contains_key(&old_member), Error::<T>::NotMember);

			Members::<T>::remove(&old_member);
			MemberCount::mutate(|v| *v -= 1);
			Self::deposit_event(RawEvent::MemberRemoved(old_member));
			Ok(())
		}

	}
}

impl<T: Config> AccountSet for Module<T> {
	type AccountId = T::AccountId;

	fn accounts() -> BTreeSet<T::AccountId> {
		<Members<T> as IterableStorageMap<T::AccountId, ()>>::iter()
			.map(|(acct, _)| acct)
			.collect::<BTreeSet<_>>()
	}
}
