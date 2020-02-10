# Event
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

[Sometimes](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/generic-event) events might contin types from the pallet's Configuration Trait. In this case, it is necessary to specify additional syntax

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

*See the next example to use the simple event syntax in the context of verifying successful execution of an [adding machine](./adder.md)*
