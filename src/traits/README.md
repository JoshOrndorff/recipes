# Substrate Types and Traits

To access **substrate specific types**, the module's `Trait` may inherit from the [Substrate Runtime Module Library](https://github.com/paritytech/substrate/tree/master/srml). For example, to access the Substrate types `Hash`, `AccountId`, and `BlockNumber`, it is sufficient to inherit the [`system`](https://github.com/paritytech/substrate/tree/master/srml/system) module:

```rust, ignore
pub trait Trait: system::Trait {}
```

This provides access to `Hash`, `AccountId`, and `BlockNumber` anywhere that specifies the generic `<T: Trait>` using `T::<Type>`. It also provides access to other useful types, declared in the `pub Trait {}` block in [`systems/src/lib.rs`](https://github.com/paritytech/substrate/blob/v1.0/srml/system/src/lib.rs).

> basically add a note here on why traits are important for runtime development `=>` we are in the business of building libraries to support the configuration and modular and extensible digital infrastructure...

- [Currency Types](./currency.md)
- [Transaction Fees](./fees.md)
- [Mock Runtime for Unit Testing](./mock.md)

## support::traits

Unlike in smart contract development, the way to inherit shared behavior is not to directly import other modules. Instead, it is common to either implement the same logic in the new context or utilize a trait from [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/traits.rs) to guide the new implementation. By abstracting shared behavior from the runtime modules into [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/traits.rs), Substrate makes it easy to extract and enforce best practices in the runtime. You can find the trait documentation [here](https://crates.parity.io/srml_support/traits/index.html).