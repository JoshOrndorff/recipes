# Mock Runtime for Unit Testing
*see [root](./index.md) for list of kitchen modules with unit test coverage*

At the bottom of the runtime module, place unit tests in a separate rust module with a special compilation flag

```rust, ignore
#[cfg(test)]
mod tests {
	...
}
```

To use the logic from the runtime module to be tested, bring `Module` and `Trait` into scope.

```rust, ignore
use crate::{Module, Trait};
```

Now, declare the mock runtime as a unit structure

```rust, ignore
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
```

The `derive` macro attribute provides implementations of the `Clone + PartialEq + Eq + Debug` traits for the `TestRuntime` struct. 

The mock runtime also needs to implement the tested module's `Trait`. If it is unnecessary to test the module's `Event` type, the type can be set to `()`. See further below to test the module's `Event` enum.

```rust, ignore
impl Trait for TestRuntime {
	type Event = ();
}
```

Next, we create a new type that wraps the mock `TestRuntime` in the module's `Module`.

```rust, ignore
pub type HelloSubstrate = Module<Runtime>;
```

It may be helpful to read this as type aliasing our configured mock runtime to work with the module's `Module`, which is what is ultimately being tested.

## `impl system::Trait`

In many cases, the module's `Trait` inherits `system::Trait` like

```rust, ignore
pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

The mock runtime must inherit and define the `system::Trait` associated types (*[remember](https://substrate.dev/recipes/traits/index.html)*). To do so, `impl` the `system::Trait` for `TestRuntime` with a few types imported from other crates,

```rust, ignore
use support::{impl_outer_event, impl_outer_origin, parameter_types};
use runtime_primitives::{Perbill, traits::{IdentityLookup, BlakeTwo256}, testing::Header};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for TestRuntime {
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

pub type System = system::Module<TestRuntime>;
```

With this, it is possible to use this type in the unit tests. For example, the block number can be set with [`set_block_number`](https://crates.parity.io/srml_system/struct.Module.html#method.set_block_number)

```rust, ignore
#[test]
fn add_emits_correct_event() {
	// ExtBuilder syntax is explained further below
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(2);
		// some assert statements and HelloSubstrate calls
	}
}
```

## Basic Test Environments

To build the test runtime environment, import `runtime_io`

```rust, ignore
use runtime_io;
```

In the `Cargo.toml`, this only needs to be imported under `dev-dependencies` since it is only used in the `tests` module,

```
[dev-dependencies.runtime-io]
default_features = false
git = 'https://github.com/paritytech/substrate.git'
package = 'sr-io'
rev = '6ae3b6c4ddc03d4cdb10bd1d417b95d20f4c1b6e'
```

There is more than one pattern for building a mock runtime environment for testing module logic. Two patterns are presented below. The latter is generally favored for reasons discussed in [custom test environment](./externalities.md)
* [`new_test_ext`](#newext) -  consolidates all the logic for building the environment to a single public method, but isn't relatively configurable (i.e. uses one set of module constants)
* [`ExtBuilder`](#extbuilder) - define methods on the unit struct `ExtBuilder` to facilitate a flexible environment for tests (i.e. can reconfigure module constants in every test if necessary)

## new_test_ext <a name = "newext"><a/>
*[`kitchen/modules/smpl-treasury`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/smpl-treasury)*

In [`smpl-treasury`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/smpl-treasury), use the `balances::GenesisConfig` and the module's `Genesis::<TestRuntime>` to set the balances of the test accounts and establish council membership in the returned test environment.

```rust, ignore
pub fn new_test_ext() -> runtime_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
	balances::GenesisConfig::<TestRuntime> {
		balances: vec![
			// members of council (can also be users)
			(1, 13),
			(2, 11),
			(3, 1),
			(4, 3),
			(5, 19),
			(6, 23),
			(7, 17),
			// users, not members of council
			(8, 1),
			(9, 22),
			(10, 46),
		],
		vesting: vec![],
	}.assimilate_storage(&mut t).unwrap();
	GenesisConfig::<TestRuntime>{
		council: vec![
			1,
			2,
			3,
			4,
			5,
			6,
			7,
		]
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}
```

More specifically, this sets the `AccountId`s in the range of `[1, 7]` inclusive as the members of the `council`. This is expressed in the `decl_module` block with the addition of an `add_extra_genesis` block,

```rust, ignore
add_extra_genesis {
	build(|config| {
		// ..other stuff..
		<Council<T>>::put(&config.council);
	});
}
```

To use `new_test_ext` in a runtime test, we call the method and call [`execute_with()`](https://crates.parity.io/substrate_state_machine/struct.TestExternalities.html#method.execute_with) on the returned `runtime_io::TestExternalities` 

```rust, ignore
#[test]
fn fake_test() {
	new_test_ext().execute_with(|| {
		// test logic
	})
}
```

[`execute_with()`](https://crates.parity.io/substrate_state_machine/struct.TestExternalities.html#method.execute_with)
executes all logic expressed in the closure within the configured runtime test environment specified in `new_test_ext`

## ExtBuilder <a name = "extbuilder"></a>
*[`kitchen/modules/struct-storage`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/struct-storage)*

Another approach for a more flexible runtime test environment instantiates a unit struct `ExtBuilder`,

```rust, ignore
pub struct ExtBuilder;
```

The behavior for constructing the test environment is contained the methods on the `ExtBuilder` unit structure. This fosters multiple levels of configuration depending on if the test requires a common default instance of the environment or a more specific edge case configuration. The latter is explored in more detail in [Custom Test Environment](./externalities.md).

Like `new_test_ext`, the `build()` method on the `ExtBuilder` object returns an instance of [`TestExternalities`](https://crates.parity.io/sr_io/type.TestExternalities.html). [Externalities](https://crates.parity.io/substrate_externalities/index.html) are an abstraction that allows the runtime to access features of the outer node such as storage or offchain workers. 

In this case, create a mock storage from the default genesis configuration.

```rust, ignore
impl ExtBuilder {
	pub fn build() -> runtime_io::TestExternalities {
		let mut storage = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
		runtime_io::TestExternalities::from(storage)
	}
}
```

which calls some methods to create a test environment,

```rust, ignore
#[test]
fn fake_test_example() {
	ExtBuilder::build().execute_with(|| {
		// ...test conditions...
	}) 
}
```

While testing in this environment, runtimes that require signed extrinsics (aka take `origin` as a parameter) will require transactions coming from an `Origin`. This requires importing the [`impl_outer_origin`](https://crates.parity.io/srml_support/macro.impl_outer_origin.html) macro from `support`

```rust, ignore
use support::{impl_outer_origin};

impl_outer_origin!{
	pub enum Origin for TestRuntime {}
}
```

It is possible to placed signed transactions as parameters in runtime methods that require the `origin` input. See the [full code in the kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/hello-substrate), but this looks like

```rust, ignore
#[test]
fn last_value_updates() {
	ExtBuilder::build().execute_with(|| {
		HelloSubstrate::set_value(Origin::signed(1), 10u64);
		// some assert statements
	})
}
```

Run these tests with `cargo test`, an optional parameter is the test's name to only run that test and not all tests.

NOTE: the input to `Origin::signed` is the `system::Trait`'s `AccountId` type which was set to `u64` for the `TestRuntime` implementation. In theory, this could be set to some other type as long as it conforms to the [trait bound](https://crates.parity.io/srml_system/trait.Trait.html),

```rust, ignore
pub trait Trait: 'static + Eq + Clone {
    //...
    type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + Default;
	//...
}
```
<!-- add link to testing in devhub docs after it is added -->