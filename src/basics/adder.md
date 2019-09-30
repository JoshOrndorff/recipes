# Adding Machine
*[`kitchen/modules/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/adding-machine)*

A simple adding machine checks for overflow and emits an event with the result, without using storage. In the module file,

```rust
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

If the addition overflows, the method will return the `"Addition overflowed"` without emitting the event. Likewise, events are generally emitted at the bottom of method bodies as an indication of correct execution of all logic therein.

*NOTE*: The event described above only wraps `u32` values. If we want/need the `Event` type to contain multiple types from our runtime, then the `decl_event` would use the following syntax

```rust
decl_event!(
    pub enum Event<T> {
        ...
    }
)
```

In some cases, the `where` clause can be used to specify type aliasing for more readable code

```rust
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