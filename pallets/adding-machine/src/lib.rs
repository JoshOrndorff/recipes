#![cfg_attr(not(feature = "std"), no_std)]

//! Adding Machine
//! A simple adding machine which checks for overflow and unlucky numbers.
//! Throws errors accordingly
use frame_support::{decl_error, decl_module, decl_storage, ensure, dispatch::DispatchResult};
use system::ensure_signed;

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

		fn reset(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			Sum::put(0);

			Ok(())
		}
	}
}
