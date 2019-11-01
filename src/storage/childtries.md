# Cheap Membership Proofs: Child Tries
*[`kitchen/modules/child-trie`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/child-trie)*, *[`kitchen/modules/smpl-crowdfund`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/smpl-crowdfund)*

* [Runtime Child Storage](#storj)
* [Crowdfund Example](#smplcrwd)

A [trie](https://en.wikipedia.org/wiki/Trie) is an ordered tree structure for managing dynamic sets. For any given parent node, all descendants (children) share a common prefix associated with the parent.

This construction lends itself to efficient removal of subgroups of a dataset (similar to [`double_map`](./double.md)). By associating a common prefix with related data, the dataset can be partitioned to effectively batch deletions.

In addition, proofs of inclusion are relatively small in size and can be reused to verify membership of the trie in the future.

Every change in the leaves percolates up to the root, thereby providing a complete, succinct history of all changes to the underlying data structure in the form of the trie root hash.

## Runtime Child Storage <a name = "storj"></a>

To interact with child tries, there are methods exposed in [runtime child storage](https://crates.parity.io/srml_support/storage/child/index.html). Of the methods listed in the documentation, it is worth emphasizing the method associated with batch deletion.

```rust
/// Remove all `storage_key` key/values
pub fn kill_storage(storage_key: &[u8]) {
	runtime_io::kill_child_storage(storage_key)
}

/// Remove value associated with `key` in trie with `storage_key`
pub fn kill(storage_key: &[u8], key: &[u8]) {
	runtime_io::clear_child_storage(storage_key, key);
}
```

[`kill_storage`](https://crates.parity.io/srml_support/storage/child/fn.kill_storage.html) deletes all  `(key, value)` pairs associated with the `storage_key`. [Documentation](https://crates.parity.io/srml_support/storage/child/index.html) shows that the basic API for interacting with a given child trie follows this format:

```rust
// pseudocode
child::do(trie_id, key, value);
```

To put an object in a child trie, the code would look something like 

```rust
fn kv_put(index: ObjectCount, who: &T::AccountId, value_type: &ValueType) {
    let mut buf = Vec::new();
		buf.extend_from_slice(b"exchildtr");
		buf.extend_from_slice(&index.to_le_bytes()[..]);

	let id = CHILD_STORAGE_KEY_PREFIX.into_iter()
        .chain(b"default:")
        .chain(T::Hashing::hash(&buf[..]).as_ref().into_iter())
        .cloned()
        .collect();
    
	who.using_encoded(|b| child::put(id.as_ref(), b, value_type));
}
```

The code in [`kitchen/modules/child-trie`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/child-trie) demonstrates a minimal way of organizing the basic child-trie api methods (as done in [`polkadot/runtime/crowdfund`](https://github.com/paritytech/polkadot/blob/master/runtime/src/crowdfund.rs)). It separates out the generation of the child trie id from the index with a runtime method `id_from_index`.

```rust
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
```

This results in less code for each method:

```rust
pub fn kv_put(index: ObjectCount, who: &T::AccountId, value_type: ValueType) {
    let id = Self::id_from_index(index);
    who.using_encoded(|b| child::put(id.as_ref(), b, &value_type));
}
```

## smpl-crowdfund <a name = "smplcrwd"></a>
*[`kitchen/modules/smpl-crowdfund`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/smpl-crowdfund)*

Child tries are useful for batch deletion of `(key, value)` pairs associated with a specific `trie_id`. This is relevant to the [polkadot/crowdfund](https://github.com/paritytech/polkadot/blob/master/runtime/src/crowdfund.rs) module, which tracks `(AccountId, BalanceOf<T>)` associated with a specific crowdfund. `BalanceOf<T>` represents the contributions of an `AccountId`. The identifier for each crowdfund is defined

```rust
type FundIndex = u32
```

With these three types, this storage item effectively manages `(FundIndex, AccountId, BalanceOf<T>)`. By maintaining a separate `child` for every `FundIndex`, this api allows for efficient batch deletions when crowdfunds are ended and dissolved.

```rust
// polkadot/runtime/crowdfund
pub fn crowdfund_kill(index: FundIndex) {
    let id = Self::id_from_index(index);
    child::kill_storage(id.as_ref());
}
```

The child trie api is useful when data associated with an identifier needs to be isolated to facilitate efficient batch removal. In this case, all the information associated with a given crowdfund should be removed when the crowdfund is dissolved.

## caveat coder

Each individual call to read/write to the child trie is more expensive than it would be for `map` or `double_map`. This cost is poorly amortized over a large number of calls, but can be significantly reduced by following a proper batch execution strategy.

Storage proofs associated with child tries are small in size, thereby allowing for storage on-chain if necessary. Short proofs of inclusion may also prove useful in the cross-chain context (i.e. when exporting data to other chains).

Constructing and using such proofs can be done via an outside call to the light client. The call would create an extrinsic that returns a proof of inclusion from the runtime. Note that there is no *proper* way of doing this in the runtime at this time.
