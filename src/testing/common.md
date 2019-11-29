# Common Tests

To verify that our module code behaves as expected, it is necessary to check a few conditions with unit tests. Intuitively, the order of the testing may resemble the structure of runtime method development.
1. Within each runtime method, declarative checks are made prior to any state change. These checks ensure that any required conditions are met before all changes occur; need to ensure that [panics panic](#panicspanic).
2. Next, verify that the [expected storage changes occurred](#storage).
3. Finally, check that the [expected events were emitted](#events) with correct values.

### Checks before Changes are Enforced (i.e. Panics Panic) <a name = "panicspanic"></a>

The [`Verify First, Write Last`](https://substrate.dev/recipes/declarative/ensure.html) recipe encourages verifying certain conditions before changing storage values. In tests, it might be desirable to verify that invalid inputs return the expected error message.

In [`kitchen/module/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/adding-machine), the runtime method `add` checks for overflow

```rust
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

```rust
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

```rust
#[cfg(test)]
mod tests {
	use support::{assert_err, impl_outer_event, impl_outer_origin, parameter_types};
	// more imports and tests
}
```

For more examples, see the substrate runtime modules -- `mock.rs` for mock runtime scaffolding and `test.rs` for unit tests.

### Expected Changes to Storage are Triggered <a name = "storage"></a>

Changes to storage can be checked by direct calls to the storage values. The syntax is the same as it would be in the module's runtime methods

```rust
#[test]
fn last_value_updates() {
	ExtBuilder::build().execute_with(|| {
		let expected = 10u64;
		HelloSubstrate::set_value(Origin::signed(1), expected);
		assert_eq!(HelloSubstrate::last_value(), expected);
		// .. more assert statements
	})
}
```

For context, the tested module's `decl_storage` block looks like

```rust
decl_storage! {
	trait Store for Module<T: Trait> as HelloSubstrate{
		pub LastValue get(fn last_value): u64;
		pub UserValue get(fn user_value): map T::AccountId => u64;
	}
}
```

Updates to `UserValue` are tested in `last_value_updates` in [`kitchen/module/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/hello-substrate).

### Expected Events are Emitted <a name = "events"></a>

The common way of testing expected event emission behavior requires importing `support`'s [`impl_outer_event!`](https://crates.parity.io/srml_support/macro.impl_outer_event.html) macro

```rust
use support::impl_outer_event;
```

The `TestEvent` enum imports and uses the module's `Event` enum. The new local module `hello_substrate` is re-exports the contents of the root to give a name for the current crate to [`impl_outer_event!`](https://crates.parity.io/srml_support/macro.impl_outer_event.html).

```rust
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

Testing the correct emission of events compares constructions of expected events with the entries in the [`System::events`](https://crates.parity.io/srml_system/struct.Module.html#method.events) vector of `EventRecord`s. In [`kitchen/module/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/adding-machine),

```rust
#[test]
fn add_emits_correct_event() {
	ExtBuilder::build().execute_with(|| {
		AddingMachine::add(Origin::signed(1), 6, 9);

		assert_eq!(
			System::events(),
			vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: TestEvent::added(crate::Event::Added(6, 9, 15)),
					topics: vec![],
				},
			]
		);
	})
```

This check requires importing from `system`

```rust
use system::{EventRecord, Phase};
```

A more ergonomic way of testing whether a specific event was emitted might use the `System::events().iter()`. This pattern doesn't require the previous imports, but it does require importing `RawEvent` (or `Event`) from the module and `ensure_signed` from `system` to convert signed extrinsics to the underlying `AccountId`,

```rust
#[cfg(test)]
mod tests {
	// other imports
	use system::ensure_signed;
	use super::RawEvent; // if no RawEvent, then `use super::Event;`
	// tests
}
```

In [`kitchen/module/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/hello-substrate),

```rust
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
