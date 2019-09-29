# Event
*[`kitchen/modules/simple-event`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/simple-event)*, *[`kitchen/modules/generic-event`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/simple-event)*

In Substrate, [transaction](https://docs.substrate.dev/docs/glossary#section-transaction) finality does not guarantee the execution of functions dependent on the given transaction. To verify that functions have executed successfully, emit an [event](https://docs.substrate.dev/docs/glossary#section-events) at the bottom of the function body.

> **Events** notify the off-chain world of successful state transitions

To declare an event, use the [`decl_event`](https://crates.parity.io/srml_support/macro.decl_event.html) macro.

## Simple Event

The [simplest example of an event](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/simple-event) uses the following syntax

```rust
decl_event!(
    pub enum Event {
        EmitInput(u32),
    }
);
```

The event is emitted at the bottom of the `do_something` function body:

```rust
Self::deposit_event(Event::EmitInput(new_number));
```

## Events with Module Types

[Sometimes](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/generic-event) events might emit types from the module Trait. When the event uses types from the module, it is necessary to specify additional syntax

```rust
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        EmitInput(AccountId, u32),
    }
);
```

The syntax for `deposit_event` now takes the `RawEvent` type because it is generic over the module Trait 

```rust
Self::deposit_event(RawEvent::EmitInput(user, new_number));
```

*See the next example to use the simple event syntax in the context of verifying successful execution of an [adding machine](./adder.md)*

<!-- ## More Resources
> OUTDATED SO DO NOT INCLUDE AT THE MOMENT

* [`decl_event` wiki docs](https://wiki.parity.io/decl_event)
* [Substrate Collectables Tutorial: Creating Events](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/creating-an-event) -->