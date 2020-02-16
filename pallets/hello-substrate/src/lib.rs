#![cfg_attr(not(feature = "std"), no_std)]

//! A simple Substrate pallet that demonstrates declaring dispatchable functions, and
//! Printing text to the terminal.

use frame_support::{ decl_module, dispatch::DispatchResult };
use frame_system::{ self as system, ensure_signed };
use sp_runtime::print;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		pub fn say_hello(origin) -> DispatchResult {
			// Ensure that the caller is a regular keypair account
			let _ = ensure_signed(origin)?;

			// Print something from the runtime
			print("Hello World");

			// Indicate that this call succeeded
			Ok(())
		}
	}
}
