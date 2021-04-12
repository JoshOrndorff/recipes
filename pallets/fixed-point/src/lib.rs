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

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_arithmetic::{traits::Saturating, Permill};
use substrate_fixed::types::U16F16;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as FixedPoint {
		/// Permill accumulator, value starts at 1 (multiplicative identity)
		PermillAccumulator get(fn permill_value): Permill = Permill::one();
		/// Substrate-fixed accumulator, value starts at 1 (multiplicative identity)
		FixedAccumulator get(fn fixed_value): U16F16 = U16F16::from_num(1);
		/// Manual accumulator, value starts at 1 (multiplicative identity)
		ManualAccumulator get(fn manual_value): u32 = 1 << 16;
	}
}

decl_event!(
	pub enum Event {
		// For all varients of the event, the contained data is
		// (new_factor, new_product)
		/// Permill accumulator has been updated.
		PermillUpdated(Permill, Permill),
		/// Substrate-fixed accumulator has been updated.
		FixedUpdated(U16F16, U16F16),
		/// Manual accumulator has been updated.
		ManualUpdated(u32, u32),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Some math operation overflowed
		Overflow,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Update the Permill accumulator implementation's value by multiplying it
		/// by the new factor given in the extrinsic
		#[weight = 10_000]
		fn update_permill(origin, new_factor: Permill) -> DispatchResult {
			ensure_signed(origin)?;

			let old_accumulated = Self::permill_value();

			// There is no need to check for overflow here. Permill holds values in the range
			// [0, 1] so it is impossible to ever overflow.
			let new_product = old_accumulated.saturating_mul(new_factor);

			// Write the new value to storage
			PermillAccumulator::put(new_product);

			// Emit event
			Self::deposit_event(Event::PermillUpdated(new_factor, new_product));
			Ok(())
		}

		/// Update the Substrate-fixed accumulator implementation's value by multiplying it
		/// by the new factor given in the extrinsic
		#[weight = 10_000]
		fn update_fixed(origin, new_factor: U16F16) -> DispatchResult {
			ensure_signed(origin)?;

			let old_accumulated = Self::fixed_value();

			// Multiply, handling overflow
			let new_product = old_accumulated.checked_mul(new_factor)
				.ok_or(Error::<T>::Overflow)?;

			// Write the new value to storage
			FixedAccumulator::put(new_product);

			// Emit event
			Self::deposit_event(Event::FixedUpdated(new_factor, new_product));
			Ok(())
		}

		/// Update the manually-implemented accumulator's value by multiplying it
		/// by the new factor given in the extrinsic
		#[weight = 10_000]
		fn update_manual(origin, new_factor: u32) -> DispatchResult {
			ensure_signed(origin)?;

			// To ensure we don't overflow unnecessarily, the values are cast up to u64 before multiplying.
			// This intermediate format has 48 integer positions and 16 fractional.
			let old_accumulated : u64 = Self::manual_value() as u64;
			let new_factor_u64 : u64 = new_factor as u64;

			// Perform the multiplication on the u64 values
			// This intermediate format has 32 integer positions and 32 fractional.
			let raw_product : u64 = old_accumulated * new_factor_u64;

			// Right shift to restore the convention that 16 bits are fractional.
			// This is a lossy conversion.
			// This intermediate format has 48 integer positions and 16 fractional.
			let shifted_product : u64 = raw_product >> 16;

			// Ensure that the product fits in the u32, and error if it doesn't
			if shifted_product > (u32::max_value() as u64) {
				return Err(Error::<T>::Overflow.into())
			}

			let final_product = shifted_product as u32;

			// Write the new value to storage
			ManualAccumulator::put(final_product);

			// Emit event
			Self::deposit_event(Event::ManualUpdated(new_factor, final_product));
			Ok(())
		}
	}
}
