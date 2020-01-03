# Substrate Types and Traits

Pallets may access their own associated types as well as the associated types of other pallets in the runtime. This is seen in most pallets when they access `frame_system`'s associated `AccountId` type. All pallets are tightly coupled to `frame_system` through this syntax, and thus have access to its types. You may optionally couple to additional pallets in this way.

> Another option to loosely couple to other pallets is discussed further later.

```rust, ignore
pub trait Trait: system::Trait {}
```

This provides access to `Hash`, `AccountId`, and `BlockNumber` anywhere that specifies the generic `<T: Trait>` using `T::<Type>`. It also provides access to other useful types, declared in [`frame_system`'s configuration trait](https://substrate.dev/rustdocs/master/frame_system/trait.Trait.html).

> basically add a note here on why traits are important for runtime development `=>` we are in the business of building libraries to support the configuration and modular and extensible digital infrastructure...

- [Currency Types](./currency.md)
- [Transaction Fees](./fees.md)
- [Mock Runtime for Unit Testing](./mock.md)

## support::traits

Unlike in smart contract development, the way to inherit shared behavior is not to directly import other pallets. Instead, it is common to either implement the same logic in the new context or utilize a trait from [`frame_support`](https://substrate.dev/rustdocs/master/frame_support/index.html) to guide the new implementation. By abstracting common behavior from pallets into `frame_support`, Substrate makes it easy to extract and enforce best practices in the runtime.
