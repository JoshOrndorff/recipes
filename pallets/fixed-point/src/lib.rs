#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet that demonstrates the fundamentals of Fixed Point arithmetic.
//! This pallet implements three multiplicative accumulators using fixed point.
//!
//! ## Manual Implementation
//! Here we use simple u32 values, and just keep in mind that the high-order 16 bits
//! represent the integer part while the low 16 bits represent fractional places.
//!
//! ## Permill Implementation
//! Here we use Substrate's built-in Permill type. We'll use the saturating_mul function
//!
//!
//! ## Substrate-fixed Implementation
//! Here we use an external crate called substrate-fixed which implements more advanced
//! mathematical operations including transcendental functions.

use sp_arithmetic::Permill;
use frame_support::{
	decl_event,
	decl_error,
	decl_module,
	decl_storage,
	dispatch::DispatchResult,
};
use frame_system::{self as system, ensure_signed};
use substrate_fixed::types::U32F32;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		/// Manual accumulator
		ManualAccumulator get(fn manual_value): u32;
		/// Permill accumulator
		PermillAccumulator get(fn permill_value): Permill;
		/// Substrate-fixed accumulator
		FixedAccumulator get(fn fixed_value): U32F32;
	}
}

decl_event!(
	pub enum Event {
		// For all varients of the event, the contained data is
		// (new_factor, new_product)
		/// Manual accumulator has been updated.
		ManualUpdated(u32, u32),
		/// Permill accumulator has been updated.
		PermillUpdated(Permill, Permill),
		/// Substrate-fixed accumulator has been updated.
		FixedUpdated(U32F32, U32F32),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Some math operation overflowed
		Overflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Update the manually-implemented accumulator's value by multiplying it
		/// by the new factor given in the extrinsic
		fn update_manual(origin, new_factor: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// To ensure we don't overflow, the values are cast up to u64 before multiplying
			let old_accumulated = Self::manual_value() as u64;
			let new_factor_u64 = new_factor as u64;

			// Perform the multiplication on the u64 values
			let raw_product = old_accumulated * new_factor_u64;

			// Convert back to a u32 to store the new value. We right shift to maintain the
			// convention that 16 bits are fractional
			let shifted_product = raw_product >> 16;

			// Ensure that the product fits in the u32, and error if it doesn't
			if shifted_product > u32::max_value() as u64 {
				return Err(Error::<T>::Overflow.into())
			}

			// Write the new value to storage
			ManualAccumulator::put(shifted_product as u32);

			// Emit event
			Self::deposit_event(Event::ManualUpdated(new_factor, shifted_product as u32));
			Ok(())
		}
	}
}
