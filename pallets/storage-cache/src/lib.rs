//! A pallet that demonstrates caching values from storage in memory
//! Takeaway: minimize calls to runtime storage

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn some_copy_value)]
	pub(super) type SomeCopyValue<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn king_member)]
	pub(super) type KingMember<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn group_members)]
	pub(super) type GroupMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// swap old value with new value (new_value, time_now)
		InefficientValueChange(u32, T::BlockNumber),
		// '' (new_value, time_now)
		BetterValueChange(u32, T::BlockNumber),
		// swap old king with new king (old, new)
		InefficientKingSwap(T::AccountId, T::AccountId),
		// '' (old, new)
		BetterKingSwap(T::AccountId, T::AccountId),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		///  (Copy) inefficient way of updating value in storage
		///
		/// storage value -> storage_value * 2 + input_val
		#[pallet::weight(10_000)]
		pub fn increase_value_no_cache(
			origin: OriginFor<T>,
			some_val: u32,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			let original_call = <SomeCopyValue<T>>::get();
			let some_calculation = original_call
				.checked_add(some_val)
				.ok_or("addition overflowed1")?;
			// this next storage call is unnecessary and is wasteful
			let unnecessary_call = <SomeCopyValue<T>>::get();
			// should've just used `original_call` here because u32 is copy
			let another_calculation = some_calculation
				.checked_add(unnecessary_call)
				.ok_or("addition overflowed2")?;
			<SomeCopyValue<T>>::put(another_calculation);
			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(Event::InefficientValueChange(another_calculation, now));
			Ok(().into())
		}

		/// (Copy) more efficient value change
		///
		/// storage value -> storage_value * 2 + input_val
		#[pallet::weight(10_000)]
		pub fn increase_value_w_copy(
			origin: OriginFor<T>,
			some_val: u32,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			let original_call = <SomeCopyValue<T>>::get();
			let some_calculation = original_call
				.checked_add(some_val)
				.ok_or("addition overflowed1")?;
			// uses the original_call because u32 is copy
			let another_calculation = some_calculation
				.checked_add(original_call)
				.ok_or("addition overflowed2")?;
			<SomeCopyValue<T>>::put(another_calculation);
			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(Event::BetterValueChange(another_calculation, now));
			Ok(().into())
		}

		///  (Clone) inefficient implementation
		/// swaps the king account with Origin::signed() if
		/// (1) other account is member &&
		/// (2) existing king isn't
		#[pallet::weight(10_000)]
		pub fn swap_king_no_cache(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let new_king = ensure_signed(origin)?;
			let existing_king = <KingMember<T>>::get();

			// only places a new account if
			// (1) the existing account is not a member &&
			// (2) the new account is a member
			ensure!(
				!Self::is_member(&existing_king),
				"current king is a member so maintains priority"
			);
			ensure!(
				Self::is_member(&new_king),
				"new king is not a member so doesn't get priority"
			);

			// BAD (unnecessary) storage call
			let old_king = <KingMember<T>>::get();
			// place new king
			<KingMember<T>>::put(new_king.clone());

			Self::deposit_event(Event::InefficientKingSwap(old_king, new_king));
			Ok(().into())
		}

		///  (Clone) better implementation
		/// swaps the king account with Origin::signed() if
		/// (1) other account is member &&
		/// (2) existing king isn't
		#[pallet::weight(10_000)]
		pub fn swap_king_with_cache(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let new_king = ensure_signed(origin)?;
			let existing_king = <KingMember<T>>::get();
			// prefer to clone previous call rather than repeat call unnecessarily
			let old_king = existing_king.clone();

			// only places a new account if
			// (1) the existing account is not a member &&
			// (2) the new account is a member
			ensure!(
				!Self::is_member(&existing_king),
				"current king is a member so maintains priority"
			);
			ensure!(
				Self::is_member(&new_king),
				"new king is not a member so doesn't get priority"
			);

			// <no (unnecessary) storage call here>
			// place new king
			<KingMember<T>>::put(new_king.clone());

			Self::deposit_event(Event::BetterKingSwap(old_king, new_king));
			Ok(().into())
		}

		// ---- for testing purposes ----
		#[pallet::weight(10_000)]
		pub fn set_copy(origin: OriginFor<T>, val: u32) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			<SomeCopyValue<T>>::put(val);
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn set_king(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;
			<KingMember<T>>::put(user);
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn mock_add_member(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let added = ensure_signed(origin)?;
			ensure!(!Self::is_member(&added), "member already in group");
			<GroupMembers<T>>::append(added);
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		<GroupMembers<T>>::get().contains(who)
	}
}
