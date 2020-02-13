# Single Value
*[`pallets/single-value`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/single-value)*

Storage is used for data that should be kept between blocks, and accessible to future transactions. Most blockchains will have many storage values, and together the storage values make up the blockchain's "state". The storage values themselves are _not_ stored in the blocks. Instead the blocks contains extrinsics which represent _changes_ to the storage values. It is the job of each node in a blockchain network to keep track of the current storage.


The blockchain's runtime storage is configured in [`decl_storage`](https://substrate.dev/rustdocs/master/frame_support/macro.decl_storage.html).

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as HelloWorld {
		pub LastValue get(fn last_value): u64;
		pub UserValue get(fn user_value): map T::AccountId => u64;
	}
}
```

Within a specific pallet, a single value (`u32` type) is stored in the runtime using the [`decl_storage`](https://wiki.parity.io/decl_storage) macro

```rust, ignore
decl_storage! {
    trait Store for Module<T: Trait> as SingleValue {
        MyValue: u32;
    }
}
```

To interact with single storage values, it is necessary to import the `frame_support::StorageValue` type. Functions used to access a `StorageValue` are defined in [`frame-support`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageValue.html):

```rust, ignore
/// Get the storage key.
fn hashed_key() -> [u8; 16];

/// true if the value exists in storage.
fn exists() -> bool;

/// Load the value from the provided storage instance.
fn get() -> Self::Query;

///Put the borrowed value at the key
fn put<Arg: Borrow<T>>(val: Arg);

/// Put an unsized and `Encode` value at the key
fn put_ref<Arg: ?Sized + Encode>(val: &Arg) where T: AsRef<Arg>;

/// Mutate the value at the key
fn mutate<R, F: FnOnce(&mut G::Query) -> R>(f: F) -> R;

/// Takes the value at the key
fn take() -> G::Query;

/// Clear the storage value
fn kill();
```

Therefore, the syntax to "put" `Value`:

```rust, ignore
<MyValue>::put(1738);
```

and to "get" `Value`:

```rust, ignore
let my_val = <MyValue>::get();
```

Note that we do not need the type `T` because the value is only of one type `u32`. If the `T` was polymorphic over more than one type, the syntax would include `T` in call

```rust, ignore
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyValue: u32;
        MyAccount: T::AccountId;
    }
}
```

Now that we're using `T::AccountId` in the `MyAccount` storage value, it is necessary to specify that the call is generic over the trait `Trait` by writing

```rust, ignore
// in a runtime method in `decl_module` block
<MyAccount<T>>::get()
```

The requirements for setting the `AccountId` stored in `MyAccount` can be specified in the runtime and exposed via

```rust, ignore
<MyAccount<T>>::put(some_account_id);
```

*The full example  emits events to also notify off-chain processes of when values were `set` and `got`.*
