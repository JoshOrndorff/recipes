#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet to demonstrate configurable pallet constants.
//! This pallet has a single storage value that can be added to by calling the
//! `add_value` extrinsic.
//!
//! The value added cannot exceed a maximum which is specified as a configuration constant.
//! The stored value is cleared (set to zero) at a regular interval which is specified
//! as a configuration constant.

use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::Get,
};
use frame_system::ensure_signed;
use sp_runtime::traits::Zero;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;

	/// Maximum amount added per invocation
	type MaxAddend: Get<u32>;

	/// Frequency with which the stored value is deleted
	type ClearFrequency: Get<Self::BlockNumber>;
}

decl_storage! {
	trait Store for Module<T: Config> as ConfigurableConstants {
		SingleValue get(fn single_value): u32;
	}
}

decl_event!(
	pub enum Event {
		/// The value has ben added to. The parameters are
		/// ( initial amount, amount added, final amount)
		Added(u32, u32, u32),
		/// The value has been cleared. The parameter is the value before clearing.
		Cleared(u32),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		const MaxAddend: u32 = T::MaxAddend::get();

		const ClearFrequency: T::BlockNumber = T::ClearFrequency::get();

		/// Add to the stored value. The `val_to_add` parameter cannot exceed the specified manimum.
		#[weight = 10_000]
		fn add_value(origin, val_to_add: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= maximum add amount constant");

			// previous value got
			let c_val = <SingleValue>::get();

			// checks for overflow when new value added
			let result = match c_val.checked_add(val_to_add) {
				Some(r) => r,
				None => return Err(DispatchError::Other("Addition overflowed")),
			};
			<SingleValue>::put(result);
			Self::deposit_event(Event::Added(c_val, val_to_add, result));
			Ok(())
		}

		/// For testing purposes
		/// Sets the stored value to a given value
		#[weight = 10_000]
		fn set_value(origin, value: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			<SingleValue>::put(value);
			Ok(())
		}

		fn on_finalize(n: T::BlockNumber) {
			if (n % T::ClearFrequency::get()).is_zero() {
				let c_val = <SingleValue>::get();
				<SingleValue>::put(0u32);
				Self::deposit_event(Event::Cleared(c_val));
			}
		}
	}
}
