# Incrementing Balances

This recipe demonstrates how we can store a `balance` type and increment it using a runtime method. The first step is to add `srml-balances` to the `Cargo.toml` file like so

```
[dependencies.balances]
default_features = false
git = 'https://github.com/paritytech/substrate.git'
package = 'srml-balances'
branch = 'v1.0'
```

Don't forget to add `balances/std` under the `[features]` section:

```
[features]
default = ['std']
std = [
    'parity-codec/std',
    'support/std',
    'system/std',
    'runtime-primitives/std',
    'balances/std',
]
```

The `decl_event` macro generates an `Event` type which needs to be exposed in the module. This type inherits the `balances` trait. *See [single value storage](../storage/value.md) recipe for more information on Substrate specific types*

```rust
/// in module file
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> to Into<<Self as system::Trait>::Event>;
}
```

Our stored balance type is kept in the [`decl_storage`](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html) block

```rust
decl_storage! {
	trait Store for Module<T: Trait> as IncBalance {
		BalanceVal get(balance_val): Option<T::Balance>;
	}
}
```

The `NewBalance` event associated with updating `BalanceVal` uses the generic type `B = <T as balances::Trait>::Balance`

```rust
/// in module file
decl_event!(
	pub enum Event<T> where B = <T as balances::Trait>::Balance {
		NewBalance(B),
	}
);
```

To use events in the runtime, it is necessary to add a function to deposit the declared events. Within the [`decl_module`](https://crates.parity.io/srml_support/macro.decl_module.html) block, add a new function

```rust
/// in module file, decl_module block
fn deposit_event<T>() = default();
```

**Note**: If your event uses only Rust primitive types, then the generic `<T>` is unncesssary and can be omitted. *See [adding machine](./adder.md) for an example of this*

After checking for the successful state transition in the body of a function, the corresponding event should be invoked.

```rust
/// in module file, decl_module block
pub fn accumulate_dummy(origin, increase_by: T::Balance) -> Result {
    // This is a public call, so we ensure that the origin is some signed account.
    let _sender = ensure_signed(origin)?;

    // use the `::get` on the storage item type itself
    let balance_val = <BalanceVal<T>>::get();

    // Calculate the new value.
    let new_balance = balance_val.map_or(increase_by, |val| val + increase_by);

    // Put the new value into storage.
    <BalanceVal<T>>::put(new_balance);

    // Deposit an event to let the outside world know this happened.
    Self::deposit_event(RawEvent::NewBalance(increase_by));

    // All good.
    Ok(())
}
```
