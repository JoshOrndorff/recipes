# Event

On Substrate, [transaction](https://docs.substrate.dev/docs/glossary#section-transaction) finalization does not necessarily ensure the execution of functions dependent on the transaction. To check that dependent functions have been executed successfully, we need to emit an [event]() at the end of the function.

> *Events* serve to notify the off-chain world of completed state transitions

To declare an event, we utilize the `decl_event!` macro. Here's an example of the event declaration:

```rust
decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Balance
    {
        MyEvent(u32, Balance),
        MyOtherEvent(Balance, AccountId),
    }
);
```

This `decl_event!` macro generates an `Event` type which needs to be exposed in the module. This type needs to inherit some traits:

```rust
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> to Into<<Self as system::Trait>::Event>;
}
```

To use events in the runtime, it is necessary to add a function to deposit the declared events. Within the `decl_module!` block, add a new function

```rust
fn deposit_event<T>() = default();
```

*Note*: If your event uses only Rust primitive types, then the generic `<T>` is unncesssary and can be omitted.

After checking for the successful state transition in the body of your function, the event should be deposited.

```rust
let some_value = 1738;
let some_balance = <T::Balance as As<u64>>::sa(1738);

Self::deposit_event(RawEvent::MyEvent(some_value, some_balance));
```

Next, update the `lib.rs` file to include the new `Event<T>` type under the module's `Trait` implementation

```rust
impl mymodule::Trait for Runtime {
    type Event = Event<T>;
}
```

Now include the `Event<T>` type in the module's definition in the `construct_runtime!` macro.

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

## Recipes

* [Adding Machine](./adder.md)
* [Permissioned Function with Generic Event](./permissioned.md)

## Examples in the <a href="">SRML Source Code</a>

* [SRML EXAMPLES HERE](https://wiki.parity.io/decl_event)

> [Creating an Event](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/creating-an-event)

> TCR relevant code

### TODO

* clean up existing examples and format in a coherent way
* off-chain patterns (Substrate event listener)

*Use SourceGraph*
* include page on the srml examples of `decl_event!`
* annotate a few and link a bunch of others

* add more event patterns