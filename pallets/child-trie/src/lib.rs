#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::string_lit_as_bytes)]
//! Child Trie API
//! - auxiliary runtime methods for using child storage
//! - see smpl-crowdfund for examples of using this API with objects in the pallet
use frame_support::{decl_module, decl_storage, storage::child};
use sp_core::Hasher;

use frame_system as system;
use parity_scale_codec::{Decode, Encode};
use sp_std::prelude::*;

pub trait Trait: system::Trait {}

pub type ObjectCount = u32;
pub type ValAppended = u32;

#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ExampleObject;

decl_storage! {
	trait Store for Module<T: Trait> as ChildTrie {
		ExampleObjects get(fn example_objects):
			map hasher(twox_64_concat) ObjectCount => Option<ExampleObject>;

		TheObjectCount get(fn the_object_count): ObjectCount;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

/// Child trie unique id for a crowdfund is built from the hash part of the fund id.
// pub fn trie_unique_id(fund_id: &[u8]) -> child::ChildInfo {
// 	let start = CHILD_STORAGE_KEY_PREFIX.len() + b"default:".len();
// 	child::ChildInfo::new_default(&fund_id[start..])
// }

impl<T: Trait> Module<T> {
	/// Find the ID associated with the Child Trie
	/// to access the respective trie
	/// (see invocations in the other methods below for context)
	pub fn id_from_index(index: ObjectCount) -> child::ChildInfo {
		let mut buf = Vec::new();
		buf.extend_from_slice(b"exchildtr");
		buf.extend_from_slice(&index.to_le_bytes()[..]);

		child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
	}

	pub fn kv_put(index: ObjectCount, who: &T::AccountId, value_to_put: ValAppended) {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::put(&id, b, &value_to_put));
	}

	pub fn kv_get(index: ObjectCount, who: &T::AccountId) -> ValAppended {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::get_or_default::<ValAppended>(&id, b))
	}

	pub fn kv_kill(index: ObjectCount, who: &T::AccountId) {
		let id = Self::id_from_index(index);
		who.using_encoded(|b| child::kill(&id, b));
	}

	pub fn kill_trie(index: ObjectCount) {
		let id = Self::id_from_index(index);
		child::kill_storage(&id);
	}
}
