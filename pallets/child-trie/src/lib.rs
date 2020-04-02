//! Child Trie API
//! - auxiliary runtime methods for using child storage
//! - see smpl-crowdfund for examples of using this API with objects in the pallet
use primitives::{Blake2Hasher, Hasher};
use primitives::storage::well_known_keys::CHILD_STORAGE_KEY_PREFIX;
use support::{decl_module, decl_storage, storage::child};

use parity_scale_codec::{Decode, Encode};
use rstd::prelude::*;

pub trait Trait: system::Trait {}

pub type ObjectCount = u32;
pub type ValAppended = u32;

#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ExampleObject;

decl_storage! {
	trait Store for Module<T: Trait> as ChildTrie {
		ExampleObjects get(example_objects):
			map hasher(twox_64_concat) ObjectCount => Option<ExampleObject>;

		TheObjectCount get(the_object_count): ObjectCount;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

/// Child trie unique id for a crowdfund is built from the hash part of the fund id.
pub fn trie_unique_id(fund_id: &[u8]) -> child::ChildInfo {
	let start = CHILD_STORAGE_KEY_PREFIX.len() + b"default:".len();
	child::ChildInfo::new_default(&fund_id[start..])
}

impl<T: Trait> Module<T> {
	/// Find the ID associated with the Child Trie
	/// to access the respective trie
	/// (see invocations in the other methods below for context)
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
		who.using_encoded(|b| child::put(
				id.as_ref(),
				trie_unique_id(id.as_ref()),
				b,
				&value_to_put
		));
	}

	pub fn kv_get(index: ObjectCount, who: &T::AccountId) -> ValAppended {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::get_or_default::<ValAppended>(
				id.as_ref(),
				trie_unique_id(id.as_ref()),
				b
		))
	}

	pub fn kv_kill(index: ObjectCount, who: &T::AccountId) {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::kill(
				id.as_ref(),
				trie_unique_id(id.as_ref()),
				b
		));
	}

	pub fn kill_trie(index: ObjectCount) {
		let id = Self::id_from_index(index);
		child::kill_storage(
			id.as_ref(),
			trie_unique_id(id.as_ref()),
		);
	}
}
