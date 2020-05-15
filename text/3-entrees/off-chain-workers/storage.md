# Local Storage in Off-chain Workers

*[`pallets/offchain-demo`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo)*

Remember we mentioned that off-chain workers (short for **ocw** below) cannot write directly to the on-chain storage, that is why they have to submit transactions back on-chain to modify the state.

Fortunately, there is also a local storage that persist across runs in off-chain workers. Storage is local within off-chain workers and not passed within network. Storage of off-chain workers is persisted across runs of off-chain workers and blockchain re-organizations.

Off-chain workers are asynchronously run during block import. Since ocws are not limited by how long they run, at any single instance there could be multiple ocws running, being initiated by previous block imports. See diagram below.

![More than one off-chain workers at a single instance](/img/multiple-ocws.png)

The storage has a similar API usage as on-chain [`StorageValue`](/2-appetizers/2-storage-values.html) with `get`, `set`, and `mutate`. `mutate` is using a [`compare-and-set`](https://en.wikipedia.org/wiki/Compare-and-swap) pattern. It compares the contents of a memory location with a given value and, only if they are the same, modifies the contents of that memory location to a new given value. This is done as a single atomic operation. The atomicity guarantees that the new value is calculated based on up-to-date information; if the value had been updated by another thread in the meantime, the write would fail.

In this recipe, we will add a cache and lock over our previous [http fetching example](./http-json.html). If the cached value existed, we will return using the cached value. Otherwise we acquire the lock and then fetch from github public API and save it to the cache.

## Setup

First, include the relevant module.

src: `offchain-demo/src/lib.rs`

```rust
use sp_runtime::{
	// ...
	offchain::{storage::StorageValueRef},
	// ...
}
```

Then, in the `fetch_if_needed()` function, we first define a storage reference used by the off-chain worker.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {

	// Start off by creating a reference to Local Storage value.
	// Since the local storage is common for all offchain workers, it's a good practice
	// to prepend our entry with the pallet name.
	let storage = StorageValueRef::persistent(b"offchain-demo::gh-info");
	let s_lock = StorageValueRef::persistent(b"offchain-demo::lock");
	// ...
}
```

Looking at the [API doc](https://substrate.dev/rustdocs/v2.0.0-alpha.6/sp_runtime/offchain/storage/struct.StorageValueRef.html), we see there are two type of StorageValueRef, created via `::persistent()` and `::local()`. `::local()` is not fully implemented yet and `::persistent()` is enough for this use cases. We passed in a key as our storage key. As storage keys are namespaced globally, a good practice would be to prepend our pallet name in front of our storage key.

## Access

Once we have the storage reference, we can access the storage via `get`, `set`, and `mutate`. Let's demonstrate the `mutate` function as the usage of the remaining two functions are pretty self-explanatory.

First we fetch to see if github info has been fetched and cached. If yes, we return early.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {
	// ...
	if let Some(Some(gh_info)) = s_info.get::<GithubInfo>() {
		// gh-info has already been fetched. Return early.
		debug::info!("cached gh-info: {:?}", gh_info);
		return Ok(());
	}
	// ...
}
```

As with general on-chain storage, if we have a storage access pattern of **get-check-set**, it is a good indicator we should use `mutate`. This makes sure that multiple off-chain workers running concurrently does not modify the same storage entry.

We then try to acquire the lock in order to fetch github info.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {
	//...
	// We are implementing a mutex lock here with `s_lock`
	let res: Result<Result<bool, bool>, Error<T>> = s_lock.mutate(|s: Option<Option<bool>>| {
		match s {
			// `s` can be one of the following:
			//   `None`: the lock has never been set. Treated as the lock is free
			//   `Some(None)`: unexpected case, treated it as AlreadyFetch
			//   `Some(Some(false))`: the lock is free
			//   `Some(Some(true))`: the lock is held

			// If the lock has never been set or is free (false), return true to execute `fetch_n_parse`
			None | Some(Some(false)) => Ok(true),

			// Otherwise, someone already hold the lock (true), we want to skip `fetch_n_parse`.
			// Covering cases: `Some(None)` and `Some(Some(true))`
			_ => Err(<Error<T>>::AlreadyFetched),
		}
	});
	//...
}
```

We use the `mutate` function to get and set the lock value, taking advantages of its compare-and-set access pattern. If the lock is being held by another ocw (with `s` equals value of `Some(Some(true))`), we return an error indicating the fetching is done by another ocw.

The return value of the `mutate` has a type of `Result<Result<T, T>, E>`, to indicate one of the following cases:

* `Ok(Ok(T))` - the value has been successfully set in the `mutate` closure and saved to the storage.
* `Ok(Err(T))` - the value has been successfully set in the `mutate` closure, but failed to save to the storage.
* `Err(_)` - the value has **NOT** been set successfully in the `mutate` closure.

Now we check the returned value of the `mutate` function. If fetching is done by another ocw (returning `Err(<Error<T>>)`), or cannot acquire the lock (returning `Ok(Err(true))`), we skip the fetching.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {
	// ...
	// Cases of `res` returned result:
	//   `Err(<Error<T>>)` - lock is held, so we want to skip `fetch_n_parse` function.
	//   `Ok(Err(true))` - Another ocw is writing to the storage while we set it,
	//                     we also skip `fetch_n_parse` in this case.
	//   `Ok(Ok(true))` - successfully acquire the lock, so we run `fetch_n_parse`
	if let Ok(Ok(true)) = res {
		match Self::fetch_n_parse() {
			Ok(gh_info) => {
				// set gh-info into the storage and release the lock
				s_info.set(&gh_info);
				s_lock.set(&false);

				debug::info!("fetched gh-info: {:?}", gh_info);
			},
			Err(err) => {
				// release the lock
				s_lock.set(&false);
				return Err(err);
			}
		}
	}
	Ok(())
}
```

Finally, whether the `fetch_n_parse()` function success or not, we release the lock by setting it to `false`.

## Reference

* [`StorageValueRef` API doc](https://substrate.dev/rustdocs/master/sp_runtime/offchain/storage/struct.StorageValueRef.html)
* [`example-offchain-worker` pallet in Substrate repo](https://github.com/paritytech/substrate/tree/master/frame/example-offchain-worker)
