# Configurable Pallet Constants
*[`kitchen/pallets/constant-config`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/constant-config)*

To declare constant values within a runtime, it is necessary to import the [`Get`](https://substrate.dev/rustdocs/master/frame_support/traits/trait.Get.html) trait from `frame_support`

```rust, ignore
use support::traits::Get;
```

Configurable constants are declared as associated types in the pallet's `pub trait Trait` block using the `Get<T>` syntax for any type `T`.

```rust, ignore
pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;

    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    type MaxAddend: Get<u32>;

    // frequency with which the this value is deleted
    type ClearFrequency: Get<Self::BlockNumber>;
}
```

In order to make these constants accessible within the pallet, it is necessary to declare them with the `const` syntax in the `decl_module` block. Usually constants are declared at the top of this block, under `fn deposit_event`.

```rust, ignore
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        const MaxAddend: u32 = T::MaxAddend::get();

        const ClearFrequency: T::BlockNumber = T::ClearFrequency::get();
    }
}
```

This example manipulates a single value in storage declared as `SingleValue`.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as Example {
        SingleValue get(fn single_value): u32;
	}
}
```

`SingleValue` is set to `0` every `ClearFrequency` number of blocks in the `on_finalize` function that runs at the end of blocks execution.

```rust, ignore
fn on_finalize(n: T::BlockNumber) {
    if (n % T::ClearFrequency::get()).is_zero() {
        let c_val = <SingleValue>::get();
        <SingleValue>::put(0u32); // is this cheaper than killing?
        Self::deposit_event(Event::Cleared(c_val));
    }
}
```

Signed transactions may invoke the `add_value` runtime method to increase `SingleValue` as long as each call adds less than `MaxAddend`. *There is no anti-sybil mechanism so a user could just split a larger request into multiple smaller requests to overcome the `MaxAddend`*, but overflow is still handled appropriately.

```rust, ignore
fn add_value(origin, val_to_add: u32) -> Result {
    let _ = ensure_signed(origin)?;
    ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= maximum add amount constant");

    // previous single value
    let c_val = <SingleValue>::get();

    // checks for overflow
    let result = match c_val.checked_add(val_to_add) {
        Some(r) => r,
        None => return Err("Addition overflowed"),
    };
    <SingleValue>::put(result);
    Self::deposit_event(Event::Added(c_val, val_to_add, result));
    Ok(())
}
```

In more complex patterns, the constant value may be used as a static, base value that is scaled by a multiplier to incorporate stateful context for calculating some dynamic fee (ie floating transaction fees).

To test the range of pallet configurations introduced by configurable constants, see *[custom configuration of externalities](../testing/externalities.md)*
