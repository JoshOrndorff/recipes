#![cfg_attr(not(feature = "std"), no_std)]

//! Single Value Storage
use frame_support::{
	decl_module, decl_storage,
	dispatch::DispatchResult,
	weights::SimpleDispatchInfo,
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {}

decl_storage! {
	trait Store for Module<T: Trait> as SingleValue {
		StoredValue get(fn stored_value): u32;
		StoredAccount get(fn stored_account): T::AccountId;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// Set the storaged u32 value
		#[weight = SimpleDispatchInfo::default()]
		fn set_value(origin, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			// Write the supplied value into blockchain storage
			StoredValue::put(value);

			Ok(())
		}

		/// Set the stored Account Id. The syntax is slightly more complex than it was for the
		/// stored u32 because the `AccountId` type comes from the pallet's configuration trait.
		#[weight = SimpleDispatchInfo::default()]
		fn set_account(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Write the supplied value into blockchain storage
			<StoredAccount<T>>::put(&who);

			Ok(())
		}
	}
}
