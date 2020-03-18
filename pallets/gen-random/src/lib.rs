//! Generating randomness with weak entropy
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]

use primitives::{ed25519, Blake2Hasher, Hasher, H256};
/// Generating Randomness example(s)
use support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageValue};
use system::ensure_signed;
use parity_scale_codec::{Encode, Decode};

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PGeneric {
		Nonce get(fn nonce): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn use_weak_entropy(origin) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// TODO needs updated for
			// https://github.com/paritytech/substrate/pull/3699
			// and
			// https://github.com/paritytech/substrate/pull/3792
			let random_seed = <system::Module<T>>::random_seed();
			let nonce = <Nonce>::get();
			let new_random = (random_seed, nonce)
				.using_encoded(|b| Blake2Hasher::hash(b))
				.using_encoded(|mut b| u64::decode(&mut b))
				.expect("Hash must be bigger than 8 bytes; Qed");
			<Nonce>::put(nonce + 1);
			Self::deposit_event(Event::WeakEntropy(new_random));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event {
		RNGenerate(u32),
		WeakEntropy(u64),
	}
);
