# Testing Substrate

**TODO**

*This section is written in the context of writing tests for [SunshineDAO](https://github.com/AmarRSingh/MoloChameleon)*

Although the Rust compiler ensures safe memory management, it cannot formally verify the correctness of a program's logic. Fortunately, Rust also comes with a convenient suite for writing unit and integration tests. When you initiate code with Cargo, generic test scaffolding is automatically generated to simplify the developer experience. Related concepts and syntax are also covered in depth in [Chapter 11 of the Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).

*Jump Ahead to...*
* [Unit Testing](./unit.md)
* [Fuzzing](./fuzzing.md)
<!-- * [Benchmarking](./benching.md) -->

## Scaffolding

To test a module in the context of Substrate, we have to do a little bit more work to set up our testing environment. Here, we'll go over the basic scaffolding necessary to test a module. If you just want the code, you can just use the [Substrate Node Template](https://github.com/shawntabrizi/substrate-package/blob/master/substrate-node-template/runtime/src/template.rs#L68). However, because most modules require some custom configuration, it is useful to understand the components that comprise the scaffolding.

Before we dive into the weeds, create a `mock.rs` and `test.rs` file in your runtime directory ([here](https://github.com/shawntabrizi/substrate-package/blob/master/substrate-node-template/runtime/src/)). At the top of `mock.rs` and `test.rs`, include the following compilation flag:

```rust
#![cfg(test)]
```

This basically tells the compiler to only run the tests if the `cargo test` command is invoked. For more information on this syntax, check out the [Rust reference guide](https://doc.rust-lang.org/reference/attributes.html#conditional-compilation) and/or [this tutorial by Philipp Oppermann](https://os.phil-opp.com/unit-testing/).

```rust
use primitives::{BuildStorage, traits::IdentityLookup, testing::{Digest, DigestItem, Header, UintAuthorityId}};
use srml_support::impl_outer_origin;
use runtime_io;
use substrate_primitives::{H256, Blake2Hasher};
```

We also need to import the module configuration traits. For our module, we're going to import `Module` and `Tait` from our crate as well as `GenesisConfig` because a few of our storage items are set to be configured in the genesis block.

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

Even so, this doesn't work for all traits. Indeed, there are a few traits that require *manual* implementation to effectively set up our testing environment in `test.rs`.

* example of implementing the required traits, just with the config type declarations

### Building Test Environment from Config

For simpler configurations, we don't have to do much else...

```rust
// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
}
```

```rust
#[test]
fn it_works_for_default_value() {
    with_externalities(&mut new_test_ext(), || {
        // Just a dummy test for the dummy funtion `do_something`
        // calling the `do_something` function with a value 42
        assert_ok!(Dao::do_something(Origin::signed(1), 42));
        // asserting that the stored value is equal to what we stored
        assert_eq!(Dao::something(), Some(42));
    });
}
```

## ExtBuilder Pattern

https://github.com/paritytech/substrate/blob/master/srml/staking/src/mock.rs