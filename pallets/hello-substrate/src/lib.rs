#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::print;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Increase the value associated with a particular key
		#[pallet::weight(10_000)]
		pub fn say_hello(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Ensure that the caller is a regular keypair account
			let caller = ensure_signed(origin)?;

			// Print a message
			print("Hello World");
			// Inspecting a variable as well
			debug::info!("Request sent by: {:?}", caller);

			// Indicate that this call succeeded
			Ok(().into())
		}
	}
}
