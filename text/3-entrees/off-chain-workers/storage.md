# Local Storage in Off-chain Workers

*[`pallets/offchain-demo`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo)*

Remember we mentioned that off-chain workers cannot write directly to the on-chain storage, that is why they have to submit transactions back on-chain to modify the state.

Fortunately, there is also a local storage that persist across runs in off-chain workers. It has a similar API usage as [`StorageValue`](/2-appetizers/2-storage-values.html) with `get`, `set`, and `mutate`.

Storage of off-chain workers is persisted across runs of off-chain workers and across nodes with off-chain wokers enabled.

In this recipe, we will add a simple cache over our previous [http fetching example](./http-json.html). If the cached value existed, we will return using the cached value. Otherwise we fetch from github public API and save it to the cache.

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
	// ...
}
```

Looking at the [API doc](https://substrate.dev/rustdocs/v2.0.0-alpha.6/sp_runtime/offchain/storage/struct.StorageValueRef.html), we see there are two type of StorageValueRef, created via `::persistent()` and `::local()`. `::local()` is not fully implemented yet and `::persistent()` is enough for this use cases. We passed in a key as our storage key. As storage keys are namespaced globally, a good practice would be to prepend our pallet name in front of our storage key.

## Access

Once we have the storage reference, we can access the storage via `get`, `set`, and `mutate`. Let's demonstrate the `mutate` function as the usage of the remaining two functions are pretty self-explanatory.

As with general on-chain storage, if we have a storage access pattern of **get-check-set**, it is a good indicator we should use `mutate`. This makes our storage access atomically in one go rather than two. This is good as storage access is expensive.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {
	// ...

	// The local storage is persisted and shared between runs of the offchain workers,
	// and offchain workers may run concurrently. We can use the `mutate` function, to
	// write a storage entry in an atomic fashion.
	//
	// It has a similar API as `StorageValue` that offer `get`, `set`, `mutate`.
	// If we are using a get-check-set access pattern, we likely want to use `mutate` to access
	// the storage in one go.
	//
	// Ref: https://substrate.dev/rustdocs/v2.0.0-alpha.6/sp_runtime/offchain/storage/struct.StorageValueRef.html
	let res = storage.mutate(|store: Option<Option<GithubInfo>>| {
		match store {
			// info existed, returning the value
			Some(Some(info)) => {
				debug::info!("Using cached gh-info.");
				Ok(info)
			},
			// info not existed, so we remote fetch (and parse the JSON)
			_ => Self::fetch_n_parse(),
		}
	});
}
```

Here inside the closure function, we are expecting the store to be in type of `Option<Option<GithubInfo>>`. If the value existed, we return the value. If not, we call `fetch_n_parse()` function which do the heavy-lifting of sending http request and parse the returned JSON response.

Finally we get the returned value, print it out and return if we get the value successfully. Otherwise we return a custom error back.

```rust
fn fetch_if_needed() -> Result<(), Error<T>> {
	// ...
	match res {
		Ok(Ok(gh_info)) => {
			// Print out our github info, whether it is newly-fetched or cached.
			debug::info!("gh-info: {:?}", gh_info);
			Ok(())
		},
		_ => Err(<Error<T>>::HttpFetchingError)
	}
}
```

`res` looks a bit funny, a type of `Result<Result<T, E>, E>`, to indicate the following cases:

* `Ok(Ok(T))` - the value has been successfully set in the previous `mutate` closure, and has been saved to the storage successfully.
* `Ok(Err(T))` - the value has been successfully set in the previous `mutate` closure, but cannot be saved to the storage successfully.
* `Err(_)` - the value has **NOT** been set successfully in the previous `mutate` closure.

## Reference

* [`StorageValueRef` API doc](https://substrate.dev/rustdocs/master/sp_runtime/offchain/storage/struct.StorageValueRef.html)
* [`example-offchain-worker` pallet in Substrate repo](https://github.com/paritytech/substrate/tree/master/frame/example-offchain-worker)
