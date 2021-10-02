#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
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
pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{traits::Saturating, Permill};
	use substrate_fixed::types::U16F16;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::type_value]
	pub(super) fn PermillAccumulatorDefaultValue<T: Config>() -> Permill {
		Permill::one()
	}

	#[pallet::storage]
	#[pallet::getter(fn permill_value)]
	pub(super) type PermillAccumulator<T: Config> =
		StorageValue<_, Permill, ValueQuery, PermillAccumulatorDefaultValue<T>>;

	#[pallet::type_value]
	pub(super) fn FixedAccumulatorDefaultValue<T: Config>() -> U16F16 {
		U16F16::from_num(1)
	}
	#[pallet::storage]
	#[pallet::getter(fn fixed_value)]
	pub(super) type FixedAccumulator<T: Config> =
		StorageValue<_, U16F16, ValueQuery, FixedAccumulatorDefaultValue<T>>;

	#[pallet::type_value]
	pub(super) fn ManualAccumulatorDefaultValue<T: Config>() -> u32 {
		1 << 16
	}
	#[pallet::storage]
	#[pallet::getter(fn manual_value)]
	pub(super) type ManualAccumulator<T: Config> =
		StorageValue<_, u32, ValueQuery, ManualAccumulatorDefaultValue<T>>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// For all varients of the event, the contained data is
		// (new_factor, new_product)
		/// Permill accumulator has been updated.
		PermillUpdated(Permill, Permill),
		/// Substrate-fixed accumulator has been updated.
		FixedUpdated(U16F16, U16F16),
		/// Manual accumulator has been updated.
		ManualUpdated(u32, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Update the Permill accumulator implementation's value by multiplying it
		/// by the new factor given in the extrinsic
		#[pallet::weight(10_000)]
		pub fn update_permill(
			origin: OriginFor<T>,
			new_factor: Permill,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			let old_accumulated = Self::permill_value();

			// There is no need to check for overflow here. Permill holds values in the range
			// [0, 1] so it is impossible to ever overflow.
			let new_product = old_accumulated.saturating_mul(new_factor);

			// Write the new value to storage
			PermillAccumulator::<T>::put(new_product);

			// Emit event
			Self::deposit_event(Event::PermillUpdated(new_factor, new_product));
			Ok(().into())
		}

		/// Update the Substrate-fixed accumulator implementation's value by multiplying it
		/// by the new factor given in the extrinsic
		#[pallet::weight(10_000)]
		pub fn update_fixed(
			origin: OriginFor<T>,
			new_factor: U16F16,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			let old_accumulated = Self::fixed_value();

			// Multiply, handling overflow
			let new_product = old_accumulated
				.checked_mul(new_factor)
				.ok_or(Error::<T>::Overflow)?;

			// Write the new value to storage
			FixedAccumulator::<T>::put(new_product);

			// Emit event
			Self::deposit_event(Event::FixedUpdated(new_factor, new_product));
			Ok(().into())
		}

		/// Update the manually-implemented accumulator's value by multiplying it
		/// by the new factor given in the extrinsic
		#[pallet::weight(10_000)]
		pub fn update_manual(origin: OriginFor<T>, new_factor: u32) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			// To ensure we don't overflow unnecessarily, the values are cast up to u64 before multiplying.
			// This intermediate format has 48 integer positions and 16 fractional.
			let old_accumulated: u64 = Self::manual_value() as u64;
			let new_factor_u64: u64 = new_factor as u64;

			// Perform the multiplication on the u64 values
			// This intermediate format has 32 integer positions and 32 fractional.
			let raw_product: u64 = old_accumulated * new_factor_u64;

			// Right shift to restore the convention that 16 bits are fractional.
			// This is a lossy conversion.
			// This intermediate format has 48 integer positions and 16 fractional.
			let shifted_product: u64 = raw_product >> 16;

			// Ensure that the product fits in the u32, and error if it doesn't
			if shifted_product > (u32::max_value() as u64) {
				return Err(Error::<T>::Overflow.into());
			}

			let final_product = shifted_product as u32;

			// Write the new value to storage
			ManualAccumulator::<T>::put(final_product);

			// Emit event
			Self::deposit_event(Event::ManualUpdated(new_factor, final_product));
			Ok(().into())
		}
	}
}
