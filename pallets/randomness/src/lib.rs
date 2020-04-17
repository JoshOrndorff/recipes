//! Generating (insecure) randomness
#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::H256;
use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	traits::Randomness,
	weights::SimpleDispatchInfo,
};
use frame_system::{self as system, ensure_signed};
use parity_scale_codec::Encode;
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;


/// The pallet's configuration trait.
/// This trait includes two randomness sources. In production you will only ever need one. This pallet
/// includes both merely to demonstrate both.
pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;

	/// Connection to Collective Flip pallet. Typically the type would be called something like
	/// `RandomnessSource` but because we are using two sources in this pallet, we will name
	/// them explicitly
	type CollectiveFlipRandomnessSource: Randomness<H256>;

	/// Connection to Babe pallet. Typically the type would be called something like
	/// `RandomnessSource` but because we are using two sources in this pallet, we will name
	/// them explicitly
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

		/// Grab a random seed and random value from the randomness collective flip pallet
		#[weight = SimpleDispatchInfo::default()]
		fn call_collective_flip(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// Using a subject is recommended to prevent accidental re-use of the seed
			// (This does not add security or entropy)
			let subject = Self::encode_and_update_nonce();

			let random_seed = T::CollectiveFlipRandomnessSource::random_seed();
			let random_result = T::CollectiveFlipRandomnessSource::random(&subject);

			Self::deposit_event(Event::CollectiveFlip(random_seed, random_result));
			Ok(())
		}

		/// Grab a random seed and random value from the babe pallet
		#[weight = SimpleDispatchInfo::default()]
		fn call_babe_vrf(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// Using a subject is recommended to prevent accidental re-use of the seed
			// (This does not add security or entropy)
			let subject = Self::encode_and_update_nonce();

			let random_seed = T::BabeRandomnessSource::random_seed();
			let random_result = T::BabeRandomnessSource::random(&subject);

			Self::deposit_event(Event::BabeVRF(random_seed, random_result));
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

impl<T: Trait> Module<T> {
	/// Reads the nonce from storage, increments the stored nonce, and returns
	/// the encoded nonce to the caller.
	fn encode_and_update_nonce() -> Vec<u8> {
		let nonce = Nonce::get();
		Nonce::put(nonce.wrapping_add(1));
		nonce.encode()
	}
}
