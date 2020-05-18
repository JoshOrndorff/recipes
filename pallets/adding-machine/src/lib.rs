#![cfg_attr(not(feature = "std"), no_std)]

//! Adding Machine
//! A simple adding machine which checks for overflow and unlucky numbers.
//! Throws errors accordingly
use frame_support::{decl_error, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {}

decl_storage! {
	trait Store for Module<T: Trait> as AddingMachine {
		Sum get(fn sum): u32;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Thirteen is unlucky and prohibited
		UnluckyThirteen,
		/// Sum would have overflowed if we had added
		SumTooLarge,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// Adds the supplies value to the stored value.
		/// Checks for unlucky number 13.
		/// Checks for addition overflow using an explicit match
		#[weight = 10_000]
		fn add(origin, val_to_add: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// First check for unlucky number 13
			ensure!(val_to_add != 13, <Error<T>>::UnluckyThirteen);

			// Now check for overflow while adding
			let result = match Self::sum().checked_add(val_to_add) {
				Some(r) => r,
				None => return Err(<Error<T>>::SumTooLarge.into()),
			};

			// Write the new sum to storage
			Sum::put(result);

			Ok(())
		}

		/// Adds the supplies value to the stored value.
		/// Checks for unlucky number 13.
		/// Checks for addition overflow concisely using `ok_or`
		#[weight = 10_000]
		fn add_alternate(origin, val_to_add: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			ensure!(val_to_add != 13, <Error<T>>::UnluckyThirteen);

			// Using `ok_or()` to check if the returned value is `Ok` and unwrap the value.
			//   If not, returns error from the function.
			let result = Self::sum().checked_add(val_to_add).ok_or(<Error<T>>::SumTooLarge)?;

			Sum::put(result);
			Ok(())
		}

		/// Resets the stoage value to zero
		#[weight = 10_000]
		fn reset(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			Sum::put(0);

			Ok(())
		}
	}
}
