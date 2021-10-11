#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
//! A pallet that implements a storage set on top of a storage map and demonstrates performance
//! tradeoffs when using vec sets.

use account_set::AccountSet;
use frame_support::storage::IterableStorageMap;
use sp_std::collections::btree_set::BTreeSet;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// A maximum number of members. When membership reaches this number, no new members may join.
	pub const MAX_MEMBERS: u32 = 16;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub(super) type Members<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, (), ValueQuery>;

	#[pallet::storage]
	pub(super) type MemberCount<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Added a member
		MemberAdded(T::AccountId),
		/// Removed a member
		MemberRemoved(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Cannot join as a member because you are already a member
		AlreadyMember,
		/// Cannot give up membership because you are not currently a member
		NotMember,
		/// Cannot add another member because the limit is already reached
		MembershipLimitReached,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a member to the membership set
		#[pallet::weight(10_000)]
		pub fn add_member(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let new_member = ensure_signed(origin)?;

			let member_count = MemberCount::<T>::get();
			ensure!(
				member_count < MAX_MEMBERS,
				Error::<T>::MembershipLimitReached
			);

			// We don't want to add duplicate members, so we check whether the potential new
			// member is already present in the list. Because the membership is stored as a hash
			// map this check is constant time O(1)
			ensure!(
				!Members::<T>::contains_key(&new_member),
				Error::<T>::AlreadyMember
			);

			// Insert the new member and emit the event
			Members::<T>::insert(&new_member, ());
			MemberCount::<T>::put(member_count + 1); // overflow check not necessary because of maximum
			Self::deposit_event(Event::MemberAdded(new_member));
			Ok(().into())
		}

		/// Removes a member.
		#[pallet::weight(10_000)]
		pub fn remove_member(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let old_member = ensure_signed(origin)?;

			ensure!(
				Members::<T>::contains_key(&old_member),
				Error::<T>::NotMember
			);

			Members::<T>::remove(&old_member);
			MemberCount::<T>::mutate(|v| *v -= 1);
			Self::deposit_event(Event::MemberRemoved(old_member));
			Ok(().into())
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
