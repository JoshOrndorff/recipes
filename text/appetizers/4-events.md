# Using Events
*[`pallets/simple-event`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/simple-event)*, *[`pallets/generic-event`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/generic-event)*

In Substrate, [transaction](https://docs.substrate.dev/docs/glossary#section-transaction) finality does not guarantee the execution of functions dependent on the given transaction. To verify that functions have executed successfully, emit an [event](https://docs.substrate.dev/docs/glossary#section-events) at the bottom of the function body.

> **Events** notify the off-chain world of successful state transitions

To declare an event, use the [`decl_event`](https://substrate.dev/rustdocs/master/frame_support/macro.decl_event.html) macro.

## Simple Event

The simplest example of an event uses the following syntax

```rust, ignore
decl_event!(
    pub enum Event {
        EmitInput(u32),
    }
);
```

The event is emitted at the bottom of the `do_something` function body:

```rust, ignore
Self::deposit_event(Event::EmitInput(new_number));
```

## Events with Generic Types

[Sometimes](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/generic-event) events might contain types from the pallet's Configuration Trait. In this case, it is necessary to specify additional syntax

```rust, ignore
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        EmitInput(AccountId, u32),
    }
);
```

The syntax for `deposit_event` now takes the `RawEvent` type because it is generic over the pallet's configuration trait

```rust, ignore
Self::deposit_event(RawEvent::EmitInput(user, new_number));
```

*NOTE*: The event described above only wraps `u32` values. If we want/need the `Event` type to contain multiple types from our runtime, then the `decl_event` would use the following syntax

```rust, ignore
decl_event!(
    pub enum Event<T> {
        ...
    }
)
```

In some cases, the `where` clause can be used to specify type aliasing for more readable code

```rust, ignore
decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        <T as system::Trait>::AccountId,
        <T as system::Trait>::BlockNumber,
        <T as system::Trait>::Hash,
    {
        FakeEvent1(AccountId, Hash, BlockNumber),
        FakeEvent2(AccountId, Balance, BlockNumber),
    }
)
```


*See the next example to use the simple event syntax in the context of verifying successful execution of an [adding machine](./adder.md)*
