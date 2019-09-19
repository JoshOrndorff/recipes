# Kitchen

The kitchen is for *cooking* recipes. It is structured like the main recipes build as specified in [src/SUMMARY.md](../src/SUMMARY.md), except every code sample is stored as a library via the [substrate-module-template](https://github.com/shawntabrizi/substrate-module-template).

**NEW STRUCTURE**
* divided up into `modules` and `runtimes`...

**Event**: effectively logging, scheduling, and reacting to events defined in the `decl_event` block of the runtime.
* [Adding Machine](./adder/)

**Storage**: managing interactions with the on-chain storage via the `decl_storage` block in the runtime.
* [Single Value Storage](./value/)
* [Configurable Module Constants](./constants/)
* [Lists as Maps](./list/)

**MISC**
- [Nested Structs](./nstructs)
- Currency Types and Locking Techniques::{[lockable](./lockable), [reservable](./reservable), [imbalances](./imbalances)}
- [Token Transfer](./token)
- [Permissioned Methods](./permissioned)
- [Blockchain Event Loop](./loop)
- [Social Network](./social)

<!-- **Support**: using traits from [`srml/support`](https://github.com/paritytech/substrate/tree/master/srml/support) to inherit shared behavior from existing runtime modules
* [Using Balances](./support/balances/)

ADD BACK WHEN THERE ARE MORE RECIPES THAN JUST THIS BALANCES STRUCT
-->

## Directions

Within each folder, run the following command to build

```rust
cargo build
```

I haven't written unit tests *yet*, but you can write tests in the [usual way](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html), except with a bit more [scaffolding](https://www.shawntabrizi.com/substrate-collectables-workshop/#/5/setting-up-tests).

## Using Recipes in External Projects

Follow the [substrate-module-template](https://github.com/shawntabrizi/substrate-module-template) directions to use recipes in your personal projects. 

**I recommend extracting patterns and applying them in the context of your application rather than directly importing the recipes** :)