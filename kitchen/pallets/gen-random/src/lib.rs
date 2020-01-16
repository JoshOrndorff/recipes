/// Generating randomness with weak entropy
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]

use primitives::{ed25519, Blake2Hasher, Hasher, H256};
use primitives::{Blake2Hasher, Hasher};
/// Generating Randomness example(s)
use support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageValue};
use system::ensure_signed;
use parity_scale_codec::{Encode, Decode};
use support::{decl_event, decl_module, decl_storage, StorageValue, dispatch::DispatchResult};
use system::{self, ensure_signed};

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

            let random_seed = <system::Module<T>>::random_seed();
            let nonce = <Nonce>::get();
            let new_random = (random_seed, nonce)
                .using_encoded(|b| Blake2Hasher::hash(b))
                .using_encoded(|mut b| u64::decode(&mut b))
                .expect("Hash must be bigger than 8 bytes; Qed");
            let new_nonce = <Nonce>::get() + 1;
            <Nonce>::put(new_nonce);
            Self::deposit_event(RawEvent::RNGenerate(new_random));
            new_random
            Self::deposit_event(Event::WeakEntropy(new_random));
            <Nonce>::put(nonce + 1);
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

decl_storage! {
    trait Store for Module<T: Trait> as RNG {
        Nonce get(fn nonce): u64;
    }
}
