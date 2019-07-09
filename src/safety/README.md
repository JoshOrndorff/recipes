# Safety and Optimization

Unlike conventional software development kits that abstract away low-level decisions, Substrate grants developers fine-grain control over the underlying implementation. This approach fosters high-performance, modular applications. At the same time, it also demands increased attention from developers. To quote the [late Uncle Ben](https://knowyourmeme.com/memes/with-great-power-comes-great-responsibility), **with great power comes great responsibility**.

Indeed, Substrate developers have to exercise incredible caution. The bare-metal control that they maintain over the runtime logic introduces new attack vectors. In the context of blockchains, the cost of bugs scale with the amount of capital secured by the application. Likewise, developers should *generally* abide by a few [rules](#criteria) when building with Substrate. These rules may not hold in every situation; Substrate offers optimization in context.

* [Module Development Criteria](#criteria)
* [Declarative Programming](./cop.md)
* [Robust Path Handling](./paths.md)
* [Safe Arithmetic](./safemath.md)
* [Optimizations](./optimizations.md)

> *Testing is also important and relevant* `=>` see this section of the Substrate Collectables tutorial for more information on testing...

## Module Development Criteria <a name = "criteria"></a>

1. Modules should be independent pieces of code; if your module is tied to many other modules, it should be a smart contract. See the [substrate-contracts-workshop](https://github.com/shawntabrizi/substrate-contracts-workshop) for more details with respect to smart contract programming on Substrate.

2. It should not be possible for your code to panic after storage changes. Poor error handling in Substrate can *brick* the blockchain, rendering it useless thereafter. With this in mind, it is very important to structure code according to declarative, condition-oriented design patterns. *See more in the [declarative programming](./cop.md) section.*