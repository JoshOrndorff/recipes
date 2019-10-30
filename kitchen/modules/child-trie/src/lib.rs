//! Child Trie API
//! - auxiliary runtime methods for using child storage
//! - see modules::{smpl-crowdfund,} for examples of using this API with objects in the module
use primitives::{Blake2Hasher, Hasher};
use substrate_primitives::storage::well_known_keys::CHILD_STORAGE_KEY_PREFIX;
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, storage::child,
    traits::Get,
};
use system::ensure_signed;

use parity_scale_codec::{Decode, Encode};
use rstd::prelude::*;

pub trait Trait: system::Trait {}

pub type ObjectCount = u32;
// balances type in modules/smpl-crowdfund
pub type ValAppended = u32;

#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ExampleObject;

decl_storage! {
    trait Store for Module<T: Trait> as ChildTrie {
        ExampleObjects get(example_objects):
            map ObjectCount => Option<ExampleObject>;

        TheObjectCount get(the_object_count): ObjectCount;
    }
}

decl_module! {pub struct Module<T: Trait> for enum Call where origin: T::Origin {}}

impl<T: Trait> Module<T> {
    pub fn id_from_index(index: ObjectCount) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"exchildtr");
        buf.extend_from_slice(&index.to_le_bytes()[..]);

        CHILD_STORAGE_KEY_PREFIX
            .into_iter()
            .chain(b"default:")
            .chain(Blake2Hasher::hash(&buf[..]).as_ref().into_iter())
            .cloned()
            .collect()
    }

    pub fn kv_put(index: ObjectCount, who: &T::AccountId, value_to_put: ValAppended) {
        let id = Self::id_from_index(index);
        who.using_encoded(|b| child::put(id.as_ref(), b, &value_to_put));
    }

    pub fn kv_get(index: ObjectCount, who: &T::AccountId) -> ValAppended {
        let id = Self::id_from_index(index);
        who.using_encoded(|b| child::get_or_default::<ValAppended>(id.as_ref(), b))
    }

    pub fn kv_kill(index: ObjectCount, who: &T::AccountId) {
        let id = Self::id_from_index(index);
        who.using_encoded(|b| child::kill(id.as_ref(), b));
    }

    pub fn burn_trie(caller: T::AccountId, index: ObjectCount) {
        let id = Self::id_from_index(index);
        child::kill_storage(id.as_ref());
    }
}
