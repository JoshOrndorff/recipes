# Declarative Syntax

Unlike conventional software development kits that abstract away low-level decisions, Substrate
grants developers fine-grain control over the underlying implementation. This approach fosters
high-performance, modular applications. At the same time, it also demands increased attention from
developers. **With great power comes great responsibility**.

Indeed, Substrate developers have to exercise incredible caution. The bare-metal control that they
maintain over the runtime logic introduces new attack vectors. In the context of blockchains, the
cost of bugs scale with the amount of capital secured by the application. Likewise, developers
should generally abide by a few _[rules](#criteria)_ when building with Substrate. These rules may
not hold in every situation; Substrate offers optimization in context.

Each of the recipes in this section are oriented around increasing

-   [Verify First, Write Last](./ensure.md)
-   [Safe Math](./safemath.md)
-   [Permissioned Methods](./permissioned.md)
<!-- * [checking for collisions](./collide.md) -->

## Pallet Development Criteria <a name = "criteria"></a>

1. Pallets should be relatively independent pieces of code; if your pallet is tied to many other
   pallets, it should be a smart contract. See the
   [substrate-contracts-workshop](https://github.com/shawntabrizi/substrate-contracts-workshop) for
   more details with respect to smart contract programming on Substrate. Also, _use traits for
   abstracting shared behavior_.

2. It should not be possible for your code to panic after storage changes. Poor error handling in
   Substrate can _brick_ the blockchain, rendering it useless thereafter. With this in mind, it is
   very important to structure code according to declarative, condition-oriented design patterns.
