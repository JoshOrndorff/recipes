# Mapping

In the context of blockchains, mappings are a very powerful primitive. A *stateful* cryptocurrency might store a mapping between accounts and balances (see [example](#ex)). In this way, mappings prove useful when representing "owned" data. By tracking ownership with maps, we can easily manage permissions for modifying values specific to individual users or groups.

Within a specific module, a key-value mapping (between `u32` types) can be stored with this syntax:

```rust
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		MyMap: map u32 => u32;
	}
}
```

## Basic Interaction

To interact with a storage map, it is necessary to import the `support::StorageMap` type (see example code with the necessary import statement [below](#ex)). Functions used to access a `StorageValue` are defined [in `srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L185):

```rust
/// Get the prefix key in storage.
fn prefix() -> &'static [u8];

/// Get the storage key used to fetch a value corresponding to a specific key.
fn key_for(x: &K) -> Vec<u8>;

/// true if the value is defined in storage.
fn exists<S: Storage>(key: &K, storage: &S) -> bool {
    storage.exists(&Self::key_for(key)[..])
}

/// Load the value associated with the given key from the map.
fn get<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Take the value under a key.
fn take<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Store a value to be associated with the given key from the map.
fn insert<S: Storage>(key: &K, val: &V, storage: &S) {
    storage.put(&Self::key_for(key)[..], val);
}

/// Remove the value under a key.
fn remove<S: Storage>(key: &K, storage: &S) {
    storage.kill(&Self::key_for(key)[..]);
}

/// Mutate the value under a key.
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(key: &K, f: F, storage: &S) -> R;
```

To "insert" a `(key, value)` pair into a `StorageMap` named `MyMap`:

```rust
<MyMap<T>>::insert(key, value);
```

To "query" the `MyMap` for the `value` corresponding to a `key`:

```rust
let value = <MyMap<T>>::get(key);
```

## Account => Value <a name = "ex"></a>

Here is an example of a module that stores a mapping between `AccountId` keys to `u32` values and provides a function `set_account_value` to enable an account owner to set their corresponding value.

```rust
use srml_support::{StorageMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn set_account_value(origin, value: u32) -> Result {
			let sender = ensure_signed(origin)?;
			<MyMap<T>>::insert(sender.clone(), value);
			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as RuntimeExampleStorage {
		MyMap: map T::AccountId => u32;
	}
}
```