#![cfg_attr(not(feature = "std"), no_std)]

// demonstrates how to use append instead of mutate
// https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageValue.html#tymethod.append
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageValue};
use system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
		Members get(fn members): Vec<T::AccountId>;
		CurrentValues get(fn current_values): Vec<u32>;
		NewValues get(fn new_values): Vec<u32>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		// added member
		MemberAdded(AccountId),
		// removed member
		MemberRemoved(AccountId),
		// mutate to append
		MutateToAppend(AccountId),
		// append
		AppendVec(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		// don't do this
		// (unless appending new entries AND mutating existing entries)
		fn mutate_to_append(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// this decodes the existing vec, appends the new values, and re-encodes the whole thing
			<CurrentValues>::mutate(|v| v.extend_from_slice(&Self::new_values()));
			Self::deposit_event(RawEvent::MutateToAppend(user));
			Ok(())
		}

		// do this instead
		fn append_new_entries(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// this encodes the new values and appends them to the already encoded existing evc
			<CurrentValues>::append(Self::new_values())?;
			Self::deposit_event(RawEvent::AppendVec(user));
			Ok(())
		}

		fn add_member(origin) -> DispatchResult {
			let new_member = ensure_signed(origin)?;
			ensure!(!Self::is_member(&new_member), "must not be a member to be added");
			<Members<T>>::append(vec![new_member.clone()])?;
			Self::deposit_event(RawEvent::MemberAdded(new_member));
			Ok(())
		}

		fn remove_member(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;
			ensure!(Self::is_member(&old_member), "must be a member in order to leave");
			<Members<T>>::mutate(|v| v.retain(|i| i != &old_member));
			Self::deposit_event(RawEvent::MemberRemoved(old_member));
			Ok(())
		}
		// also see `append_or_insert`, `append_or_put` in pallet-elections/phragmen, democracy
	}
}

impl<T: Trait> Module<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		<Members<T>>::get().contains(who)
	}
}
