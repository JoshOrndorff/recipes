# Substrate Specific Types

To access Substrate specific types, the given module's `Trait` must inherit from the [SRML](https://github.com/paritytech/substrate/tree/master/srml). For example, if we would like to access the Substrate types `Hash`, `AccountId`, and `Balance`, it is sufficient to inherit the [`balances`](https://github.com/paritytech/substrate/tree/master/srml/balances) module:

```
pub trait Trait: balances::Trait {}
```

This provides access to the types `Hash`, `AccountId`, and `Balance` anywhere that we have specified the generic `<T: Trait>` using `T::<Type>`.