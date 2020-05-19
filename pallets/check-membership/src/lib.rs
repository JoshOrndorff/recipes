#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::string_lit_as_bytes)]

//! Permissioned Function with Generic Event
//! a permissioned funtion which can only be called by the "owner". An event is emitted
//! when the function is successfully executed.
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PGeneric {
		Members get(fn members): Vec<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		AddMember(AccountId),
		RemoveMember(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Adds the caller to the membership set unless the caller is already present
		#[weight = 10_000]
		fn add_member(origin) -> DispatchResult {
			let new_member = ensure_signed(origin)?;

			// Ensure that the caller is not already a member
			ensure!(!Self::is_member(&new_member), "already a member");

			<Members<T>>::append(&new_member);
			Self::deposit_event(RawEvent::AddMember(new_member));
			Ok(())
		}

		/// Removes the caller from the membership set
		#[weight = 10_000]
		fn remove_member(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;
			ensure!(Self::is_member(&old_member), "not a member so can't be taken out of the set");
			// keep all members except for the member in question
			<Members<T>>::mutate(|mem| mem.retain(|m| m != &old_member));
			Self::deposit_event(RawEvent::RemoveMember(old_member));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		Self::members().contains(who)
	}
}
