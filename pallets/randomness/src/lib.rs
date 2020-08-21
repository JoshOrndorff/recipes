//! Generating (insecure) randomness
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_event, decl_module, decl_storage, dispatch::DispatchResult, traits::Randomness,
};
use frame_system::{self as system, ensure_signed};
use parity_scale_codec::Encode;
use sp_core::H256;
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;

	/// The pallet doesn't know what the source of randomness is; it can be anything that
	/// implements the trait. When installing this pallet in a runtime, you
	/// must make sure to give it a randomness source that suits its needs.
	type RandomnessSource: Randomness<H256>;
}

decl_storage! {
	trait Store for Module<T: Trait> as RandomnessPallet {
		/// A nonce to use as a subject when drawing randomness
		Nonce get(fn nonce): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Grab a random seed and random value from the randomness collective flip pallet
		#[weight = 10_000]
		fn consume_randomness(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// Using a subject is recommended to prevent accidental re-use of the seed
			// (This does not add security or entropy)
			let subject = Self::encode_and_update_nonce();

			let random_seed = T::RandomnessSource::random_seed();
			let random_result = T::RandomnessSource::random(&subject);

			Self::deposit_event(Event::RandomnessConsumed(random_seed, random_result));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event {
		/// First element is raw seed, second is using nonce.
		RandomnessConsumed(H256, H256),
	}
);

impl<T: Trait> Module<T> {
	/// Reads the nonce from storage, increments the stored nonce, and returns
	/// the encoded nonce to the caller.
	fn encode_and_update_nonce() -> Vec<u8> {
		let nonce = Nonce::get();
		Nonce::put(nonce.wrapping_add(1));
		nonce.encode()
	}
}
