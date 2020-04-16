//! A pallet that demonstrates caching values from storage in memory
//! Takeaway: minimize calls to runtime storage

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	ensure,
	weights::SimpleDispatchInfo,
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as StorageCache {
		// copy type
		SomeCopyValue get(fn some_copy_value): u32;

		// clone type
		KingMember get(fn king_member): T::AccountId;
		GroupMembers get(fn group_members): Vec<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		BlockNumber = <T as system::Trait>::BlockNumber,
	{
		// swap old value with new value (new_value, time_now)
		InefficientValueChange(u32, BlockNumber),
		// '' (new_value, time_now)
		BetterValueChange(u32, BlockNumber),
		// swap old king with new king (old, new)
		InefficientKingSwap(AccountId, AccountId),
		// '' (old, new)
		BetterKingSwap(AccountId, AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		///  (Copy) inefficient way of updating value in storage
		///
		/// storage value -> storage_value * 2 + input_val
		#[weight = SimpleDispatchInfo::default()]
		fn increase_value_no_cache(origin, some_val: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let original_call = <SomeCopyValue>::get();
			let some_calculation = original_call.checked_add(some_val).ok_or("addition overflowed1")?;
			// this next storage call is unnecessary and is wasteful
			let unnecessary_call = <SomeCopyValue>::get();
			// should've just used `original_call` here because u32 is copy
			let another_calculation = some_calculation.checked_add(unnecessary_call).ok_or("addition overflowed2")?;
			<SomeCopyValue>::put(another_calculation);
			let now = <system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
			Ok(())
		}

		/// (Copy) more efficient value change
		///
		/// storage value -> storage_value * 2 + input_val
		#[weight = SimpleDispatchInfo::default()]
		fn increase_value_w_copy(origin, some_val: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let original_call = <SomeCopyValue>::get();
			let some_calculation = original_call.checked_add(some_val).ok_or("addition overflowed1")?;
			// uses the original_call because u32 is copy
			let another_calculation = some_calculation.checked_add(original_call).ok_or("addition overflowed2")?;
			<SomeCopyValue>::put(another_calculation);
			let now = <system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::BetterValueChange(another_calculation, now));
			Ok(())
		}

		///  (Clone) inefficient implementation
		/// swaps the king account with Origin::signed() if
		/// (1) other account is member &&
		/// (2) existing king isn't
		#[weight = SimpleDispatchInfo::default()]
		fn swap_king_no_cache(origin) -> DispatchResult {
			let new_king = ensure_signed(origin)?;
			let existing_king = <KingMember<T>>::get();

			// only places a new account if
			// (1) the existing account is not a member &&
			// (2) the new account is a member
			ensure!(!Self::is_member(&existing_king), "current king is a member so maintains priority");
			ensure!(Self::is_member(&new_king), "new king is not a member so doesn't get priority");

			// BAD (unnecessary) storage call
			let old_king = <KingMember<T>>::get();
			// place new king
			<KingMember<T>>::put(new_king.clone());

			Self::deposit_event(RawEvent::InefficientKingSwap(old_king, new_king));
			Ok(())
		}

		///  (Clone) better implementation
		/// swaps the king account with Origin::signed() if
		/// (1) other account is member &&
		/// (2) existing king isn't
		#[weight = SimpleDispatchInfo::default()]
		fn swap_king_with_cache(origin) -> DispatchResult {
			let new_king = ensure_signed(origin)?;
			let existing_king = <KingMember<T>>::get();
			// prefer to clone previous call rather than repeat call unnecessarily
			let old_king = existing_king.clone();

			// only places a new account if
			// (1) the existing account is not a member &&
			// (2) the new account is a member
			ensure!(!Self::is_member(&existing_king), "current king is a member so maintains priority");
			ensure!(Self::is_member(&new_king), "new king is not a member so doesn't get priority");

			// <no (unnecessary) storage call here>
			// place new king
			<KingMember<T>>::put(new_king.clone());

			Self::deposit_event(RawEvent::BetterKingSwap(old_king, new_king));
			Ok(())
		}

		// ---- for testing purposes ----
		#[weight = SimpleDispatchInfo::default()]
		fn set_copy(origin, val: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			<SomeCopyValue>::put(val);
			Ok(())
		}

		#[weight = SimpleDispatchInfo::default()]
		fn set_king(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;
			<KingMember<T>>::put(user);
			Ok(())
		}

		#[weight = SimpleDispatchInfo::default()]
		fn mock_add_member(origin) -> DispatchResult {
			let added = ensure_signed(origin)?;
			ensure!(!Self::is_member(&added), "member already in group");
			<GroupMembers<T>>::append(vec![added])?;
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		<GroupMembers<T>>::get().contains(who)
	}
}
