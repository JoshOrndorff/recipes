//! Generating randomness with weak entropy
#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::H256;
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, traits::Randomness};
use frame_system::{self as system, ensure_signed};
use parity_scale_codec::{Encode};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;

	/// Connection to Collective Flip pallet. Typically the type would be called something like
	/// `RandomnessSource` but because we are using two sources in this pallet, we will name
	/// them exlicitly
	type CollectiveFlipRandomnessSource: Randomness<H256>;

	/// Connection to Babe pallet. Typically the type would be called something like
	/// `RandomnessSource` but because we are using two sources in this pallet, we will name
	/// them exlicitly
	type BabeRandomnessSource: Randomness<H256>;
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

		fn call_collective_flip(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// Update the nonce and encode it to a byte-array
			let nonce = Nonce::get();
			Nonce::put(nonce.wrapping_add(1));
			let encoded_nonce = nonce.encode();

			let random_seed = T::CollectiveFlipRandomnessSource::random_seed();
			let random_result = T::CollectiveFlipRandomnessSource::random(&encoded_nonce);

			Self::deposit_event(Event::CollectiveFlip(random_seed, random_result));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event {
		/// Randomness taken from Collective Flip. First element is raw seed, second is using nonce.
		CollectiveFlip(H256, H256),
		/// Randomness taken from Babe VRF Outputs. First element is raw seed, second is using nonce.
		BabeVRF(H256, H256),
	}
);
