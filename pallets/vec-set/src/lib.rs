#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

//! A pallet that implements a storage set on top of a sorted vec and demonstrates performance
//! tradeoffs when using map sets.

use account_set::AccountSet;

pub use pallet::*;
use sp_std::collections::btree_set::BTreeSet;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	/// A maximum number of members. When membership reaches this number, no new members may join.
	pub const MAX_MEMBERS: usize = 16;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub(super) type Members<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
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

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a member to the membership set unless the max is reached
		#[pallet::weight(10_000)]
		pub fn add_member(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let new_member = ensure_signed(origin)?;

			let mut members = Members::<T>::get();
			ensure!(
				members.len() < MAX_MEMBERS,
				Error::<T>::MembershipLimitReached
			);

			// We don't want to add duplicate members, so we check whether the potential new
			// member is already present in the list. Because the list is always ordered, we can
			// leverage the binary search which makes this check O(log n).
			match members.binary_search(&new_member) {
				// If the search succeeds, the caller is already a member, so just return
				Ok(_) => Err(Error::<T>::AlreadyMember.into()),
				// If the search fails, the caller is not a member and we learned the index where
				// they should be inserted
				Err(index) => {
					members.insert(index, new_member.clone());
					Members::<T>::put(members);
					Self::deposit_event(Event::MemberAdded(new_member));
					Ok(().into())
				}
			}
		}

		/// Removes a member.
		#[pallet::weight(10_000)]
		pub fn remove_member(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let old_member = ensure_signed(origin)?;

			let mut members = Members::<T>::get();

			// We have to find out if the member exists in the sorted vec, and, if so, where.
			match members.binary_search(&old_member) {
				// If the search succeeds, the caller is a member, so remove her
				Ok(index) => {
					members.remove(index);
					Members::<T>::put(members);
					Self::deposit_event(Event::MemberRemoved(old_member));
					Ok(().into())
				}
				// If the search fails, the caller is not a member, so just return
				Err(_) => Err(Error::<T>::NotMember.into()),
			}
		}

		// also see `append_or_insert`, `append_or_put` in pallet-elections/phragmen, democracy
	}
}

impl<T: Config> AccountSet for Module<T> {
	type AccountId = T::AccountId;

	fn accounts() -> BTreeSet<T::AccountId> {
		Self::members().into_iter().collect::<BTreeSet<_>>()
	}
}
