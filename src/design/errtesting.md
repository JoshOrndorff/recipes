# Runtime Errors and Testing

Runtime tests allow you to verify the logic in your runtime module by mocking a Substrate runtime environment. This requires an explicit implementation of all the traits declared in the module(s) for the mock runtime. So, if our module trait looked like,

```rust
pub trait Trait: system::Trait {
  type Reward = SomeRewardType<AssociatedType>;

  type ConstThing = Get<u32>;
}
```

Then, the implementation would require an explicit implementation of this trait in our mock runtime (similar in structure to the [substrate](https://github.com/paritytech/substrate/tree/master/node/runtime) and [polkadot](https://github.com/paritytech/polkadot/blob/master/runtime/src/lib.rs) runtime configurations). For example,

```rust
pub type SpecificType = u32; // could be some other type

parameter_types!{
  pub const ConstThing = 255;
}

impl module::Trait for Runtime {
  type Reward = SpecificType;
  type ConstThing = ConstThing;
}
```

Within the context of testing, there are a few ways of building a mock runtime that offer varying levels of customizations. The easiest way is to mock runtime storage with


```rust

```

*To learn more about [test](), see the official docs*

What are runtime errors?

Panic vs Error Handling

Reminder: never panic

Reminder: verify first, write last

Custom Error Messages

[`decl_error`](https://crates.parity.io/srml_support/macro.decl_error.html)

* testing from `balances`