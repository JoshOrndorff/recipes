# Single Value

Substrate supports all primitive [Rust types](https://cheats.rs/) (`bool`, `u8`, `u32`, etc) as well as some [custom types specific to Substrate](https://github.com/paritytech/oo7/blob/master/packages/oo7-substrate/src/types.js) (`Hash`, `Balance`, `BlockNumber`, etc).

* [Basic Storage](#basic)
* [Storage Interaction](#interact)
* [Getter Syntax](#get)
* [Setter Syntax](#set)
* [Substrate Specific Types](#sub)

## Basic Storage <a name = "basic"></a>

Within a specific module, a single value (`u32` type) is stored in the runtime with this syntax:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyValue: u32;
    }
}
```

## Storage Interaction <a name = "interact"></a>

To interact with single storage values, it is necessary to import the `support::StorageValue` type. Functions used to access a `StorageValue` are defined in [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs):

```rust
/// Get the storage key.
fn key() -> &'static [u8];

/// true if the value is defined in storage.
fn exists<S: Storage>(storage: &S) -> bool {
    storage.exists(Self::key())
}

/// Load the value from the provided storage instance.
fn get<S: Storage>(storage: &S) -> Self::Query;

/// Take a value from storage, removing it afterwards.
fn take<S: Storage>(storage: &S) -> Self::Query;

/// Store a value under this key into the provided storage instance.
fn put<S: Storage>(val: &T, storage: &S) {
    storage.put(Self::key(), val)
}

/// Mutate this value
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(f: F, storage: &S) -> R;

/// Clear the storage value.
fn kill<S: Storage>(storage: &S) {
    storage.kill(Self::key())
}
```

Therefore, the syntax to "put" `Value`:

```rust
<MyValue<T>>::put(1738);
```

and to "get" `Value`:

```rust
let my_val = <MyValue<T>>::get();
```


## Getter Syntax <a name = "get"></a>

Storage values can also be declared with a `get` function to provide cleaner syntax for getting values.

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyValue get(value_getter): u32;
    }
}
```

The `get` parameter is optional, but, by including it, the module exposes a getter function (`fn value_getter() -> u32`). 

To "get" the `Value` with the getter function

```rust
let my_val = Self::value_getter();
```

## Setter Syntax <a name = "set"></a>

Here is an example of a module that stores a `u32` value in runtime storage and provides a function `set_value` to set the given `u32`. This code follows [convention](https://deterministic.space/elegant-apis-in-rust.html#consistent-names) for naming setters in Rust.

```rust
use srml_support::{StorageValue, dispatch::Result};

pub trait Trait: system::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin, value: u32) -> Result {
            // check sender signature to verify permissions
            let sender = ensure_signed(origin)?; 
            <MyValue<T>>::put(value);
            Ok(())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyValue: u32;
    }
}
```

# Substrate Specific Types <a name = "sub"></a>

To access Substrate specific types, the module's `Trait` must inherit from the [SRML](https://github.com/paritytech/substrate/tree/master/srml). For example, to access the Substrate types `Hash`, `AccountId`, and `Balance`, it is sufficient to inherit the [`balances`](https://github.com/paritytech/substrate/tree/master/srml/balances) module:

```
pub trait Trait: balances::Trait {}
```

This provides access to the types `Hash`, `AccountId`, and `Balance` anywhere that specifies the generic `<T: Trait>` using `T::<Type>`.

For a practical example of this syntax, *see the [Incrementing Balance](../event/balance.md) event recipe.*