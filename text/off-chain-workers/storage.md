# Local Storage in Off-chain Workers

`pallets/ocw-demo`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%ocw-demo%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

Remember we mentioned that off-chain workers (or **ocw** for short) cannot write directly to the
blockchain state? This is why they have to submit transactions back on-chain. Fortunately, there is
also local storage that persist across runs in off-chain workers. Storage is only local to off-chain
workers and is not passed within the blockchain network.

Off-chain workers are asynchronously run at the end of block import. Since ocws are not limited by how
long they run, at any single instance there could be multiple ocw instances running, being initiated by previous
block imports. See diagram below.

![More than one off-chain workers at a single instance](../img/multiple-ocws.png)

The storage has a similar API as their on-chain counterpart with `get`, `set`, and `mutate`. `mutate` is
using a [`compare-and-set`](https://en.wikipedia.org/wiki/Compare-and-swap) pattern. It compares the
contents of a memory location with a given value and, only if they are the same, modifies the
contents of that memory location to a new given value. This is done as a single atomic operation.
The atomicity guarantees that the new value is calculated based on up-to-date information; if the
value had been updated by another thread in the meantime, the write would fail.

In this recipe, we will add a cache and lock over our previous
[http fetching example](./http-json.md). If the cached value existed, we will return using the
cached value. Otherwise we acquire the lock, fetch from github public API, and save it to the cache.

## Setup

In the `fetch_github_info()` function, we first define a storage reference used by the off-chain
worker.

```rust
fn fetch_github_info() -> Result<(), Error<T>> {
	// Create a reference to Local Storage value.
	// Since the local storage is common for all offchain workers, it's a good practice
	// to prepend our entry with the pallet name.
	let s_info = StorageValueRef::persistent(b"offchain-demo::gh-info");
	// ...
}
```

We pass in a key as our storage key. As all storage keys share a single global namespace, a good practice
would be to prepend the pallet name in front of our storage key, as we have done above.

## Access

Once we have the storage reference, we can access the storage via `get`, `set`, and `mutate`. Let's
demonstrate the `mutate` function as the usage of the remaining two functions are pretty
self-explanatory.

We first check if the github info has been fetched and cached.

```rust
fn fetch_github_info() -> Result<(), Error<T>> {
	// -- snip --
	if let Some(Some(gh_info)) = s_info.get::<GithubInfo>() {
		// gh-info has already been fetched. Return early.
		debug::info!("cached gh-info: {:?}", gh_info);
		return Ok(());
	}
	// -- snip --
}
```

We then define a lock and try to acquire it before fetching github info.

```rust
fn fetch_github_info() -> Result<(), Error<T>> {
	// -- snip --

	let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
		b"offchain-demo::lock",
		LOCK_BLOCK_EXPIRATION,
		rt_offchain::Duration::from_millis(LOCK_TIMEOUT_EXPIRATION)
	);

	// -- snip --
}
```

In the above code, we first define a lock by giving it a name and set the time limit.
The time limit can be specified by providing number of blocks to wait, amount of
time to wait, or both (whichever is shorter).

We then perform the fetch after the lock is acquired.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {
	// ...
	if let Ok(_guard) = lock.try_lock() {
		match Self::fetch_n_parse() {
			Ok(gh_info) => { s_info.set(&gh_info); }
			Err(err) => { return Err(err); }
		}
	}

	Ok(())
}
```

Finally when the `_guard` variable goes out of scope, the lock is released.

## Conclusion

In this chapter, we demonstrate how to define a persistent storage value and a
storage lock that set the locking time limit by either number of block passed or
time passed, or both. Finally we demonstrate how to acquire the lock, perform
a relatively long process (fetching data externally) and writing the data back to
the storage.

## Reference

- [`StorageValueRef` API doc](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/offchain/storage/struct.StorageValueRef.html)
- [`example-offchain-worker` pallet in Substrate repo](https://github.com/paritytech/substrate/tree/master/frame/example-offchain-worker)
