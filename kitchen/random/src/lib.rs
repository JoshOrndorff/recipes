#![cfg_attr(not(feature = "std"), no_std)]

/// Generating Randomness example(s)
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue};
use system::ensure_signed;
use primitives::{ed25519, Hasher, Blake2Hasher, H256};

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as PGeneric {
        Nonce get(nonce): u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn weak_entropy(origin) -> u32{
            let nonce = <Nonce>::get();
            let new_random = (<system::Module<T>>::random_seed(), nonce)
                .using_encoded(|b| Blake2Hasher::hash(b))
                .using_encoded(|mut b| u64::decode(&mut b))
                .expect("Hash must be bigger than 8 bytes; Qed");
            let new_nonce = <Nonce>::get() + 1;
            <Nonce>::put(new_nonce);
            Self::deposit_event(RawEvent::RNGenerate(new_random));
            new_random
        }
    }
}

decl_event!(
    pub enum Event {
        RNGenerate(u32),
    }
);