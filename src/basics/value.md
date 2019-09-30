# Single Value
*[`kitchen/modules/single-value`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/single-value)*

Within a specific module, a single value (`u32` type) is stored in the runtime using the [`decl_storage`](https://wiki.parity.io/decl_storage) macro

```rust
decl_storage! {
    trait Store for Module<T: Trait> as SingleValue {
        MyValue: u32;
    }
}
```

To interact with single storage values, it is necessary to import the `support::StorageValue` type. Functions used to access a `StorageValue` are defined in [`srml/support`](https://crates.parity.io/srml_support/storage/trait.StorageValue.html#required-methods):

```rust
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
n mutate<R, F: FnOnce(&mut G::Query) -> R>(f: F) -> R;

/// Takes the value at the key
fn take() -> G::Query;

/// Clear the storage value
fn kill();
```

Therefore, the syntax to "put" `Value`:

```rust
<MyValue>::put(1738);
```

and to "get" `Value`:

```rust
let my_val = <MyValue>::get();
```

Note that we do not need the type `T` because the value is only of one type `u32`. If the `T` was polymorphic over more than one type, the syntax would include `T` in call

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyValue: u32;
        MyAccount: T::AccountId;
    }
}

```

The requirements for setting the `AccountId` stored in `MyAccount` can be specified in the runtime and exposed via

```rust
<MyAccount<T>>::put(some_account_id);
```

*The full example in [`kitchen/modules/single-value`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/single-value) emits events to also notify off-chain processes of when values were set and got.*
