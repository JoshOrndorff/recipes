# Using Events

`pallets/simple-event`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fsimple-event%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/simple-event/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

`pallets/generic-event`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fgeneric-event%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/generic-event/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

Having a [transaction](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#transaction) included in a
block does not guarantee that the function executed successfully. As we saw in the previous recipe,
many calls can cause errors, but the transaction may still be included in a block. To verify that
functions have executed successfully, emit an
[event](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#events) at the bottom of the function body.

Events notify the off-chain world of successful state transitions.

## Some Prerequisites

When using events, we have to include the `Event` type in our configuration trait. Although the
syntax is a bit complex, it is the same every time. If you are a skilled Rust programmer you will
recognize this as a series of [trait bounds](https://doc.rust-lang.org/book/ch10-02-traits.html). If
you don't recognize this feature of Rust yet, don't worry; it is the same every time, so you can
just copy it and move on.

```rust, ignore
pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}
```

Next we have to add a line inside of the `decl_module!` macro which generates the `deposit_event`
function we'll use later when emitting our events. Even experienced Rust programmers will not
recognize this syntax because it is unique to this macro. Just copy it each time.

```rust, ignore
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// This line is new
		fn deposit_event() = default;

		// --snip--
	}
}
```

## Declaring Events

To declare an event, use the
[`decl_event!` macro](https://substrate.dev/rustdocs/v3.0.0/frame_support/macro.decl_event.html). Like any rust
enum, Events have names and can optionally carry data with them. The syntax is slightly different
depending on whether the events carry data of primitive types, or generic types from the pallet's
configuration trait. These two techniques are demonstrated in the `simple-event` and `generic-event`
pallets respectively.

### Simple Events

The simplest example of an event uses the following syntax

```rust, ignore
decl_event!(
	pub enum Event {
		EmitInput(u32),
	}
);
```

### Events with Generic Types

[Sometimes](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/generic-event)
events might contain types from the pallet's Configuration Trait. In this case, it is necessary to
specify additional syntax

```rust, ignore
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		EmitInput(AccountId, u32),
	}
);
```

This example also demonstrates how the `where` clause can be used to specify type aliasing for more
readable code.

## Emitting Events

Events are emitted from dispatchable calls using the `deposit_event` method.

> Events are not emitted on block 0. So any dispatchable calls made during genesis block formation
> will have no events emitted.

### Simple Events

The event is emitted at the bottom of the `do_something` function body.

```rust, ignore
Self::deposit_event(Event::EmitInput(new_number));
```

### Events with Generic Types

The syntax for `deposit_event` now takes the `RawEvent` type because it is generic over the pallet's
configuration trait.

```rust, ignore
Self::deposit_event(RawEvent::EmitInput(user, new_number));
```

### Constructing the Runtime

For the first time in the recipes, our pallet has an associated type in its configuration trait. We
must specify this type when implementing its trait. In the case of the `Event` type, this is
entirely straight forward, and looks the same for both simple events and generic events.

```rust, ignore
impl simple_event::Trait for Runtime {
	type Event = Event;
}
```

Events, like dispatchable calls and storage items, requires a slight change to the line in
`construct_runtime!`. Notice that the `<T>` is necessary for generic events.

```rust, ignore
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// --snip--
		GenericEvent: generic_event::{Module, Call, Event<T>},
		SimpleEvent: simple_event::{Module, Call, Event},
	}
);
```
