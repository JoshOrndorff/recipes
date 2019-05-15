# Adding Machine

A simple adding machine which [checks for overflow](../advanced/safety.md#overunder) and emits an event with the result, without using storage.

First, modify `lib.rs`. Add `type Event = Event;` to the trait implementation and add `Event` to [`construct_runtime`](https://crates.parity.io/srml_support/macro.construct_runtime.html)

```rust
/// root `lib.rs`
impl runtime_example::Trait for Runtime {
	type Event = Event;
}

...
RuntimeExample: runtime_example::{Module, Call, Event},
...
```

In the module file

```rust
/// module file `runtime_example.rs`
use srml_support::dispatch::Result;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn add(_origin, val1: u32, val2: u32) -> Result {
            // checks for overflow
            let result = match val1.checked_add(val2) {
                Some(r) => r,
                None => return Err("Addition overflowed"),
            };
            Self::deposit_event(Event::Added(val1, val2, result));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event {
        Added(u32, u32, u32),
    }
);
```