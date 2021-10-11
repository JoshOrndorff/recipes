#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
//! A pallet to demonstrate configurable pallet constants.
//! This pallet has a single storage value that can be added to by calling the
//! `add_value` extrinsic.
//!
//! The value added cannot exceed a maximum which is specified as a configuration constant.
//! The stored value is cleared (set to zero) at a regular interval which is specified
//! as a configuration constant.

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Zero;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event> + IsType<<Self as frame_system::Config>::Event>;

		/// Maximum amount added per invocation
		type MaxAddend: Get<u32>;

		/// Frequency with which the stored value is deleted
		type ClearFrequency: Get<Self::BlockNumber>;
	}

	#[pallet::storage]
	#[pallet::getter(fn single_value)]
	pub(super) type SingleValue<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		/// The value has ben added to. The parameters are
		/// ( initial amount, amount added, final amount)
		Added(u32, u32, u32),
		/// The value has been cleared. The parameter is the value before clearing.
		Cleared(u32),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_finalize(n: T::BlockNumber) {
			if (n % T::ClearFrequency::get()).is_zero() {
				let c_val = SingleValue::<T>::get();
				SingleValue::<T>::put(0u32);
				Self::deposit_event(Event::Cleared(c_val));
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add to the stored value. The `val_to_add` parameter cannot exceed the specified manimum.
		#[pallet::weight(10_000)]
		pub fn add_value(origin: OriginFor<T>, val_to_add: u32) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			ensure!(
				val_to_add <= T::MaxAddend::get(),
				"value must be <= maximum add amount constant"
			);

			// previous value got
			let c_val = SingleValue::<T>::get();

			// checks for overflow when new value added
			let result = match c_val.checked_add(val_to_add) {
				Some(r) => r,
				None => {
					return Err(DispatchErrorWithPostInfo {
						post_info: PostDispatchInfo::from(()),
						error: DispatchError::Other("Addition overflowed"),
					})
				}
			};
			SingleValue::<T>::put(result);
			Self::deposit_event(Event::Added(c_val, val_to_add, result));
			Ok(().into())
		}

		/// For testing purposes
		/// Sets the stored value to a given value
		#[pallet::weight(10_000)]
		pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			SingleValue::<T>::put(value);
			Ok(().into())
		}
	}
}
