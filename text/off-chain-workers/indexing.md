# Off-chain Indexing

There are times when on-chain extrinsics need to pass data to the off-chain worker context, such as pass the extrinsic caller account to off-chain worker for further computation.

We can surely pass this piece of data via on-chain storage, but this is costly and it will make the data propagate among the blockchain network. If this is not a piece of critical information that need to have consensus upon the whole network, another way is to pass this data via off-chain indexing.

We will demonstrate this in [`ocw-demo` pallet](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs).
Knowledge discussed in this chapter built upon [using local storage in off-chain worker context](./storage.md).

> **Notes**
>
> In order to see the off-chain indexing feature in effect, please run the kitchen node with
> off-chain indexing flag on, as `./target/debug/kitchen-node --dev --tmp --enable-offchain-indexing true`

## Writing to Off-chain Storage From On-chain Context

src: [`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
const ONCHAIN_TX_KEY: &[u8] = b"ocw-demo::storage::tx";

// -- snip --

pub fn submit_number_signed(origin, number: u64) -> DispatchResult {
  // -- snip --
  offchain_index::set(ONCHAIN_TX_KEY, &number.encode());
}
```

We first define a key used in the local off-chain storage. Then we can write to the storage with
`offchain_index::set(key, value)` function. Here `offchain_index::set()` accepts values in byte
format (`&[u8]`) so we encode the number first. If you refer back to
[`offchain_index` API rustdoc](https://substrate.dev/rustdocs/v3.0.0/sp_io/offchain_index/index.html),
you will see there are only `set()` and `clear()` functions. This means from the on-chain context,
we only expect to write to this local off-chain storage location but not reading from it, and we
cannot pass data within on-chain context using this method.

## Reading the Data in Off-chain Context

src: [`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_worker(block_number: T::BlockNumber) {
  // -- snip --

  // Reading the number written in the last on-chain transaction.
  let mem_onchain_num = StorageValueRef::persistent(ONCHAIN_TX_KEY);
  if let Some(Some(onchain_num)) = mem_onchain_num.get::<u64>() {
    debug::info!("Number written on last on-chain transaction: {:?}", onchain_num);
  }
}
```

We read the data back in the `offchain_worker()` function by how we would normally read from the
local off-chain storage. We first specify the memory space with `StorageValueRef::persistent()` with
its key, and then read back the data with `get` and decode it to `u64`.

## Reference

- [`offchain_index` API rustdoc](https://substrate.dev/rustdocs/v3.0.0/sp_io/offchain_index/index.html)
