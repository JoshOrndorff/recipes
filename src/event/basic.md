# Dummy Event Declaration

To declare an event, use the `decl_event` macro

```rust
decl_event!(
	pub enum Event<T> where B = <T as balances::Trait>::Balance {
		Dummy(B),
	}
);
```

The `Dummy` event uses the generic type `B = <T as balances::Trait>::Balance`. 

The `decl_event` macro generates an `Event` type which needs to be exposed in the module. This type inherits some traits

```rust
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> to Into<<Self as system::Trait>::Event>;
}
```

To use events in the runtime, it is necessary to add a function to deposit the declared events. Within the `decl_module!` block, add a new function

```rust
fn deposit_event<T>() = default();
```

**Note**: If your event uses only Rust primitive types, then the generic `<T>` is unncesssary and can be omitted.

After checking for the successful state transition in the body of a function, the requisite event should be deposited.

```rust
fn accumulate_dummy(origin, increase_by: T::Balance) -> Result {
    // This is a public call, so we ensure that the origin is some signed account.
    let _sender = ensure_signed(origin)?;

    // use the `::get` on the storage item type itself
    let dummy = <Dummy<T>>::get();

    // Calculate the new value.
    let new_dummy = dummy.map_or(increase_by, |dummy| dummy + increase_by);

    // Put the new value into storage.
    <Dummy<T>>::put(new_dummy);

    // Deposit an event to let the outside world know this happened.
    Self::deposit_event(RawEvent::Dummy(increase_by));

    // All good.
    Ok(())
}
```

Update the `lib.rs` file to include the new `Event<T>` type under the module's `Trait` implementation

```rust
// `lib.rs`
...
impl mymodule::Trait for Runtime {
    type Event = Event<T>;
}
```

Include the `Event<T>` type in the module's definition in the `construct_runtime!` macro.

```rust
// `lib.rs`
...
construct_runtime!(
    pub enum Runtime for Log(InteralLog: DigestItem<Hash, Ed25519AuthorityId) where
        Block = Block,
        NodeBlock = opaque::Block,
        InherentData = BasicInherentData
    {
        ...
        MyModule: mymodule::{Module, Call, Storage, Event<T>},
    }
);
```