# Common Tests

To verify that our pallet code behaves as expected, it is necessary to check a few conditions with unit tests. Intuitively, the order of the testing may resemble the structure of runtime method development.
1. Within each runtime method, declarative checks are made prior to any state change. These checks ensure that any required conditions are met before all changes occur; need to ensure that [panics panic](#panicspanic).
2. Next, verify that the [expected storage changes occurred](#storage).
3. Finally, check that the [expected events were emitted](#events) with correct values.

### Checks before Changes are Enforced (i.e. Panics Panic) <a name = "panicspanic"></a>

The `Verify First, Write Last` paradigm encourages verifying certain conditions before changing storage values. In tests, it might be desirable to verify that invalid inputs return the expected error message.

In [`pallets/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/adding-machine), the runtime method `add` checks for overflow

```rust, ignore
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn add(origin, val1: u32, val2: u32) -> Result {
            let _ = ensure_signed(origin)?;
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
```

The test below verifies that the expected error is thrown for a specific case of overflow.

```rust, ignore
#[test]
fn overflow_fails() {
	ExtBuilder::build().execute_with(|| {
		assert_err!(
			AddingMachine::add(Origin::signed(3), u32::max_value(), 1),
			"Addition overflowed"
		);
	})
}
```

This requires importing the `assert_err` macro from `support`. With all the previous imported objects,

```rust, ignore
#[cfg(test)]
mod tests {
	use support::{assert_err, impl_outer_event, impl_outer_origin, parameter_types};
	// more imports and tests
}
```

For more examples, see [Substrate's own pallets](https://github.com/paritytech/substrate/tree/master/frame) -- `mock.rs` for mock runtime scaffolding and `test.rs` for unit tests.

### Expected Changes to Storage are Triggered <a name = "storage"></a>

*[pallets/single-value](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/single-value)*

Changes to storage can be checked by direct calls to the storage values. The syntax is the same as it would be in the pallet's runtime methods.

```rust, ignore
use crate::*;

#[test]
fn set_value_works() {
  ExtBuilder::build().execute_with(|| {
    assert_ok!(SingleValue::set_value(Origin::signed(1), 10));
    assert_eq!(SingleValue::stored_value(), 10);
    // Another way of accessing the storage. This pattern is needed if it is a more complexed data
    //   type, e.g. StorageMap, StorageLinkedMap
    assert_eq!(<StoredValue>::get(), 10);
  })
}
```

For context, the tested pallets's `decl_storage` block looks like

```rust, ignore
decl_storage! {
  trait Store for Module<T: Trait> as SingleValue {
    StoredValue get(fn stored_value): u32;
    StoredAccount get(fn stored_account): T::AccountId;
  }
}
```

### Expected Events are Emitted <a name = "events"></a>

The common way of testing expected event emission behavior requires importing `support`'s [`impl_outer_event!`](https://substrate.dev/rustdocs/v2.0.0-alpha.8/frame_support/macro.impl_outer_event.html) macro

```rust, ignore
use support::impl_outer_event;
```

The `TestEvent` enum imports and uses the pallet's `Event` enum. The new local pallet, `hello_substrate`, re-exports the contents of the root to give a name for the current crate to `impl_outer_event!`.

```rust, ignore
mod hello_substrate {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		hello_substrate<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}
```

Testing the correct emission of events compares constructions of expected events with the entries in the [`System::events`](https://substrate.dev/rustdocs/v2.0.0-alpha.8/frame_system/struct.Module.html#method.events) vector of `EventRecord`s. In [`pallets/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master//pallets/adding-machine),

```rust, ignore
#[test]
fn add_emits_correct_event() {
	ExtBuilder::build().execute_with(|| {
		AddingMachine::add(Origin::signed(1), 6, 9);

		assert_eq!(
			System::events(),
			vec![
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::added(crate::Event::Added(6, 9, 15)),
					topics: vec![],
				},
			]
		);
	})
}
```

This check requires importing from `system`

```rust, ignore
use system::{EventRecord, Phase};
```

A more ergonomic way of testing whether a specific event was emitted might use the `System::events().iter()`. This pattern doesn't require the previous imports, but it does require importing `RawEvent` (or `Event`) from the pallet and `ensure_signed` from `system` to convert signed extrinsics to the underlying `AccountId`,

```rust, ignore
#[cfg(test)]
mod tests {
	// other imports
	use system::ensure_signed;
	use super::RawEvent; // if no RawEvent, then `use super::Event;`
	// tests
}
```

In [`pallets/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/hello-substrate),

```rust, ignore
#[test]
fn last_value_updates() {
	ExtBuilder::build().execute_with(|| {
		HelloSubstrate::set_value(Origin::signed(1), 10u64);
		// some assert checks

		let id_1 = ensure_signed(Origin::signed(1)).unwrap();
		let expected_event1 = TestEvent::hello_substrate(
			RawEvent::ValueSet(id_1, 10),
		);
		assert!(System::events().iter().any(|a| a.event == expected_event1));
	})
}
```

This test constructs an `expected_event1` based on the event that the developer expects will be emitted upon the successful execution of logic in `HelloSubstrate::set_value`. The `assert!()` statement checks if the `expected_event1` matches the `.event` field for any `EventRecord` in the `System::events()` vector.
