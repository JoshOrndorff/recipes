# Testing Substrate

Although the Rust compiler ensures safe memory management, it cannot formally verify the correctness of a program's logic. Fortunately, Rust also comes with a convenient suite for writing unit and integration tests. When you initiate code with Cargo, generic test scaffolding is automatically generated to simplify the developer experience. Testing concepts and syntax are covered in depth in [Chapter 11 of the Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).

<!-- *Jump Ahead to...*
* [Unit Testing](./unit.md)
* [Fuzzing](./fuzzing.md) -->
<!-- * [Benchmarking](./benching.md) -->

## Scaffolding

To test a module in the context of Substrate, we have to do a little bit more work to set up our testing environment. Here, we'll go over the basic scaffolding necessary to test a module. If you just want the code, you can just use the [Substrate Node Template](https://github.com/shawntabrizi/substrate-package/blob/master/substrate-node-template/runtime/src/template.rs#L68). However, because most modules require some custom configuration, it is useful to understand the components that comprise the scaffolding.

Before we dive into the weeds, create a `mock.rs` and `test.rs` file in your runtime directory ([here](https://github.com/shawntabrizi/substrate-package/blob/master/substrate-node-template/runtime/src/)). At the top of `mock.rs` and `test.rs`, include the following compilation flag:

```rust
#![cfg(test)]
```

This basically tells the compiler to only run the tests if the `cargo test` command is invoked. For more information on this syntax, check out the [Rust reference guide](https://doc.rust-lang.org/reference/attributes.html#conditional-compilation) as well as [this tutorial by Philipp Oppermann](https://os.phil-opp.com/unit-testing/).

```rust
use primitives::{BuildStorage, traits::IdentityLookup, testing::{Digest, DigestItem, Header, UintAuthorityId}};
use srml_support::impl_outer_origin;
use runtime_io;
use substrate_primitives::{H256, Blake2Hasher};
```

We also need to import the module configuration traits. For our module, we're going to import `Module` and `Trait`. We may also import `GenesisConfig` if some storage items are set to be configured in the genesis block (marked with `config()` in the `decl_storage` block).

```rust
use crate::{GenesisConfig, Module, Trait};
```

### Constructing a Mock Runtime

To test our module, we need to construct a mock runtime. To do so, we must create a configuration type (`Test`) which implements each of the configuration traits of modules we want to use.

```rust
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;
```

The [derive attribute](https://doc.rust-lang.org/edition-guide/rust-2018/macros/custom-derive.html) ensures that you don't have to manually implement the `Clone, PartialEq, Eq, Debug` traits; the compiler does this for you thereby ensuring that the `Test` type conforms to the behavior of these traits.

Even so, this doesn't work for all traits. Indeed, there are a few traits that require manual implementation to effectively set up our testing environment in `test.rs`. In most case, these *implementations* are limited to specifying the type in your module that corresponds to the type in the imported module. For example, the [Staking module](https://github.com/paritytech/substrate/blob/master/srml/staking/src/mock.rs) implements the `balances` trait in its `mock.rs` file like so:

```rust
impl balances::Trait for Test {
	type Balance = u64;
	type OnFreeBalanceZero = Staking;
	type OnNewAccount = ();
	type Event = ();
	type TransactionPayment = ();
	type TransferPayment = ();
	type DustRemoval = ();
}
```

> If the configuration logic is not overly complicated, the pattern that follows below can be forgone and replaced with something like [the test scaffolding in `srml/aura`](https://github.com/paritytech/substrate/blob/master/srml/aura/src/mock.rs).

Next, define the an `ExtBuilder` struct that contains the configuration items from your module. In [`srml/staking`](https://github.com/paritytech/substrate/blob/master/srml/staking/src/mock.rs), this looks like

```rust
pub struct ExtBuilder {
	existential_deposit: u64,
	session_length: u64,
	sessions_per_era: u64,
	current_era: u64,
	reward: u64,
	validator_pool: bool,
	nominate: bool,
	validator_count: u32,
	minimum_validator_count: u32,
	fair: bool,
}
```

It is useful for testing purposes to define default configuration values for each of the struct's fields. There is [a derive macro](https://doc.rust-lang.org/std/default/trait.Default.html) which could be invoked instead as an annotation on the `ExtBuilder` struct, but it assumes certain default values. From [`srml/staking`](https://github.com/paritytech/substrate/blob/master/srml/staking/src/mock.rs),

```rust
impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			existential_deposit: 0,
			session_length: 1,
			sessions_per_era: 1,
			current_era: 0,
			reward: 10,
			validator_pool: false,
			nominate: true,
			validator_count: 2,
			minimum_validator_count: 0,
			fair: true
		}
	}
}
```

To implement the relevant methods for `ExtBuilder`, convention dictates defining a function to set each configuration value like so

```rust
impl ExtBuilder {
    	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	pub fn session_length(mut self, session_length: u64) -> Self {
		self.session_length = session_length;
		self
	}
	pub fn sessions_per_era(mut self, sessions_per_era: u64) -> Self {
		self.sessions_per_era = sessions_per_era;
		self
	}
	pub fn _current_era(mut self, current_era: u64) -> Self {
		self.current_era = current_era;
		self
	}
	pub fn validator_pool(mut self, validator_pool: bool) -> Self {
		self.validator_pool = validator_pool;
		self
	}
	pub fn nominate(mut self, nominate: bool) -> Self {
		self.nominate = nominate;
		self
	}
	pub fn validator_count(mut self, count: u32) -> Self {
		self.validator_count = count;
		self
	}
	pub fn minimum_validator_count(mut self, count: u32) -> Self {
		self.minimum_validator_count = count;
		self
	}
	pub fn fair(mut self, is_fair: bool) -> Self {
		self.fair = is_fair;
		self
	}
    // more code...
}
```

In addition, we define a `build` method for `ExtBuilder` to properly set all the configuration values in our runtime storage. If we are just using our default values, it is not more complicated than defining the following method:

```rust
fn build() -> runtime_io::TestExternalities<Blake2Hasher> {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
}
```

However, the logic for the `build` method in [`srml/staking`](https://github.com/paritytech/substrate/blob/master/srml/staking/src/mock.rs) is a tad bit more complicated in order to allow for a diversity of testing scenarios.

At the bottom of the `mock.rs` file, we publicly declare all of the modules that we're using in correspondence to the traits that were *implemented* for the `Test` struct. For [`srml/staking`](https://github.com/paritytech/substrate/blob/master/srml/staking/src/mock.rs),

```rust
pub type System = system::Module<Test>;
pub type Balances = balances::Module<Test>;
pub type Session = session::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;
pub type Staking = Module<Test>;
```

### Setting up the Testing Environment

All of the types publicly declared at the bottom of `mock.rs` are imported in `test.rs` along with any other traits that will be used in the unit testing and any necessary comparison operators. We'll continue with using [`srml/staking/src/mock.rs`] as an example:

```rust
// don't forget this at the top of the file to indicate 
// compilation only with the `cargo test` command
#![cfg(test)]

use super::*;
use runtime_io::with_externalities;
use phragmen;
use primitives::PerU128;
use srml_support::{assert_ok, assert_noop, assert_eq_uvec, EnumerableStorageMap}; // comparison operators
use mock::{Balances, Session, Staking, System, Timestamp, Test, ExtBuilder, Origin}; // publicly declared types
use srml_support::traits::{Currency, ReservableCurrency};
```

<!-- Next, we'll demonstrate the proper syntax for [unit testing](./unit.md) in the `test.rs` file. -->