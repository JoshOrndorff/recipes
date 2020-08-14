# Single Value

`pallets/single-value`
[
	![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)
](https://playground-staging.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fsingle-value%2Fsrc%2Flib.rs)
[
	![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)
](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/single-value/src/lib.rs)

Storage is used for data that should be kept between blocks and accessible to future transactions.
Most runtimes will have many storage values, and together the storage values make up the
blockchain's "state". The storage values themselves are _not_ stored in the blocks. Instead the
blocks contain extrinsics that represent _changes_ to the storage values. It is the job of each node
in a blockchain network to keep track of the current storage. The current state of storage can be
determined by executing all of the blocks in the chain.

## Declaring Storage

A pallet's storage items are declared with the
[`decl_storage!` macro](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_support/macro.decl_storage.html).

```rust, ignore
decl_storage! {
    trait Store for Module<T: Trait> as SingleValue {
        // --snip--
    }
}
```

The code above is boilerplate that does not change with the exception of the `SingleValue`. The
macro uses this as the name for a struct that it creates. As a pallet author you don't need to worry
about this value much, and it is fine to use the name of the pallet itself.

This pallet has two storage items, both of which are single storage values. Substrate's storage API
also supports more complex storage types which are
[covered in the entrees](../3-entrees/storage-api/index.md). The fundamentals for all types are
the same.

Our first storage item is a `u32` value which is declared with this syntax

```rust, ignore
StoredValue get(fn stored_value): u32;
```

The `StorageValue` is the name of the storage item, similar to a variable name. We will use this
name any time we write to the storage item. The `get(fn stored_value)` is optional. It tells the
`decl_storage!` macro to create a getter function for us. That means we get a function called
`stored_value` which returns the value in that storage item. Finally, the `: u32` declares the type
of the item.

The next storage item is an `AccountId`. This is not a primitive type, but rather comes from the
system pallet. Types like this need to be prefixed with a `T::` as we see here.

```rust, ignore
StoredAccount get(fn stored_account): T::AccountId;
```

## Reading and Writing to Storage

Functions used to access a single storage value are defined in the
[`StorageValue` trait](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_support/storage/trait.StorageValue.html). In
this pallet, we use the most common method, `put`, but it is worth skimming the other methods so you
know what is available.

The `set_value` method demonstrates writing to storage, as well as taking a parameter in our
dispatchable call.

```rust, ignore
fn set_value(origin, value: u32) -> DispatchResult {
	let _ = ensure_signed(origin)?;

	StoredValue::put(value);

	Ok(())
}
```

To read a value from storage, we could use the `get` method, or we could use the getter method we
declared in `decl_storage!`.

```rust, ignore
// The following lines are equivalent
let my_val = StoredValue::get();
let my_val = Self::stored_value();
```

## Storing the Callers Account

In terms of storage, the `set_account` method is quite similar to `set_value`, but it also
demonstrates how to retreive the `AccountId` of the caller using the
[`ensure_signed` function](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_system/fn.ensure_signed.html).

```rust, ignore
fn set_account(origin) -> DispatchResult {
	let who = ensure_signed(origin)?;

	<StoredAccount<T>>::put(&who);

	Ok(())
}
```

Because the `AccountId` type comes from the configuration trait, we must use slightly different
syntax. Notice the `<T>` attached to the name of the storage value this time. Notice also that
because `AccountId` is not primitive, we lend a reference to it rather than transferring ownership.

## Constructing the Runtime

We learned about the
[`construct_runtime!` macro](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_support/macro.construct_runtime.html) in
the previous section. Because this pallet uses storage items, we must add this to the line in
construct runtime. In the Super Runtime, we see the additional `Storage` feature.

```rust, ignore
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// --snip--
		SingleValue: single_value::{Module, Call, Storage},
	}
);
```
