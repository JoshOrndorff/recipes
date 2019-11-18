# Mock Runtime for Unit Testing
*[`kitchen/module/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/hello-substrate), [`kitchen/module/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/adding-machine)*

At the bottom of the module, we can place unit tests in a separate module with a special compilation flag

```rust
#[cfg(test)]
mod tests {
	...
}
```

To use the logic from the module to be tested, it is necessary to bring `Module` and `Trait` into scope.

```rust
use crate::{Module, Trait};
```

Now, declare the mock runtime as a blanket structure

```rust
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;
```

The `derive` macro attribute provides implementations of the `Clone + PartialEq + Eq + Debug` traits for the `Runtime` struct. 

The mock runtime also needs to implement the tested module's `Trait`. 

```rust
impl Trait for Runtime {
	type Event = TestEvent;
}
```

The `TestEvent` enum is defined to emulate the module's `Event` enum. 

```rust
mod hello_substrate {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for Runtime {
		hello_substrate<T>,
	}
}

impl Trait for Runtime {
	type Event = TestEvent;
}
```

This requires using `support`'s [`impl_outer_event!`](https://crates.parity.io/srml_support/macro.impl_outer_event.html) macro

```rust
use support::impl_outer_event;
```

Next, we create a new type that wraps the mock `Runtime` in the module's `Module`.

```rust
pub type HelloSubstrate = Module<Runtime>;
```

It may be helpful to read this as type aliasing our configured mock runtime to work with the module's `Module`, which is what is ultimately being tested.

To build the runtime environment, import `runtime_io`

```rust
use runtime_io;
```

and define the `ExtBuilder` object

```rust
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> runtime_io::TestExternalities {
		let mut storage = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
		runtime_io::TestExternalities::from(storage)
	}
}
```

which calls some methods to create a test environment,

```rust
#[test]
fn fake_test_example() {
	ExtBuilder::build().execute_with(|| {
		// ...test conditions...
	}) 
}
```

While testing in this environment, runtimes that require signed extrinsics (aka take `origin` as a parameter) will require transactions coming from an `Origin`. This requires also importing [`impl_outer_origin`](https://crates.parity.io/srml_support/macro.impl_outer_origin.html) macro from `support`

```rust
use support::{impl_outer_event, impl_outer_origin};

impl_outer_origin!{
	pub enum Origin for Runtime {}
}
```

It is possible to placed signed transactions as parameters in runtime methods that require the `origin` input. See the [full code in the kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/hello-substrate)), but this looks like

```rust
#[test]
fn last_value_updates() {
	ExtBuilder::build().execute_with(|| {
		HelloSubstrate::set_value(Origin::signed(1), 10u64);
		// some assert statements
	})
}
```

Run `cargo test` to run the unit tests

## Storage Changes

Changes to storage can be checked by direct calls to the storage values. The syntax is the same as it would be in the module's runtime methods

```rust
#[test]
fn last_value_updates() {
	ExtBuilder::build().execute_with(|| {
	HelloSubstrate::set_value(Origin::signed(1), 10u64);
	assert_eq!(HelloSubstrate::last_value(), 10u64);
	HelloSubstrate::set_value(Origin::signed(2), 11u64);
	assert_eq!(HelloSubstrate::last_value(), 11u64);
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

Updates to `UserValue` are *covered* in the `last_value_updates` test in [`kitchen/module/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/hello-substrate).

## `impl system::Trait`

Substrate's design makes it convenient for the `trait Trait` to inherit `system::Trait` to inherit its types (*[remember](https://substrate.dev/recipes/traits/index.html)*). To inherit this behavior in the mock `Runtime`, it is necessary to `impl` the `system::Trait` (and import some types to do so),

```rust
use support::{impl_outer_event, impl_outer_origin, parameter_types};
use runtime_primitives::{Perbill, traits::{IdentityLookup, BlakeTwo256}, testing::Header};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for Runtime {
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
}

pub type System = system::Module<Runtime>;
```

With this, it is possible to use this type in the unit tests. For example, the block number can be set with [`set_block_number`](https://crates.parity.io/srml_system/struct.Module.html#method.set_block_number)

```rust
#[test]
fn add_emits_correct_event() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(2);
		// some assert statements and HelloSubstrate calls
	}
}
```

## Event Emission

Testing the correct emission of events should call [`System::events`](https://crates.parity.io/srml_system/struct.Module.html#method.events) to compare the events emitted in the test environment with the expected event emission. In [`kitchen/module/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/adding-machine),

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

A more ergonomic way of testing whether a specific event was emitted might use the `System::events().iter()`. This pattern would doesn't require the previous imports, but it does require importing `RawEvent` from the module and `ensure_signed` from `system` to convert signed extrinsics to the underlying `AccountId`,

```rust
#[cfg(test)]
mod tests {
	// other imports
	use system::ensure_signed;
	use super::RawEvent;
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

This test constructs an `expected_event1` based on the event that the developer expects will be emitted upon the successful execution of logic in `HelloSubstrate::set_value`. The `assert!()` statement checks if the `expected_event1` is contained in the `System::events` vector of `EventRecord`s.

## Panics Panic

The [`Verify First, Write Last`](https://substrate.dev/recipes/declarative/ensure.html) recipe encourages verifying certain conditions before changing storage values. In tests, it might be desirable to verify that invalid inputs cannot trigger runtime method logic and return the accurate error message.

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

A naive test to verify that the overflow throws the correct error message,

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

For more examples, see the [paint](https://github.com/paritytech/substrate/tree/master/paint) modules (specifically `mock.rs` for mock runtime scaffolding and `test.rs` for unit tests)