# Safety and Optimization

Unlike conventional software development kits that abstract away low-level decisions, Substrate grants developers fine-grain control over the underlying implementation. This approach fosters high-performance, modular applications. At the same time, it also demands increased attention from developers. To quote the [late Uncle Ben](https://knowyourmeme.com/memes/with-great-power-comes-great-responsibility), **with great power comes great responsibility**.

Indeed, Substrate developers have to exercise incredible caution. The bare-metal control that they maintain over the runtime logic introduces new attack vectors. In the context of blockchains, the cost of bugs scale with the amount of capital secured by the application. Likewise, developers should *generally* abide by a few [rules](#criteria) when building with Substrate. These rules may not hold in every situation; Substrate offers optimization in context.

* [Module Development Criteria](#criteria)
* [Declarative Programming](./cop.md)
* [Optimizations](./optimizations.md)

## Testing

*Testing is not (yet) covered in the Substrate Recipes, but there is a great introduction to testing in the context of Substrate in the [Crypto Collectables Tutorial](https://www.shawntabrizi.com/substrate-collectables-workshop/#/5/setting-up-tests).* I also have enjoyed the following articles/papers on testing that apply to code organization more generally:
* [Conditional Compilation and Rust Unit Testing](https://os.phil-opp.com/unit-testing/)
* [Design for Testability](https://blog.nelhage.com/2016/03/design-for-testability/)
* [How I Test](https://blog.nelhage.com/2016/12/how-i-test/)
* [Simple Testing Can Prevent Most Critical Failures](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-yuan.pdf)

## Module Development Criteria <a name = "criteria"></a>

1. Modules should be independent pieces of code; if your module is tied to many other modules, it should be a smart contract. See the [substrate-contracts-workshop](https://github.com/shawntabrizi/substrate-contracts-workshop) for more details with respect to smart contract programming on Substrate.

2. It should not be possible for your code to panic after storage changes. Poor error handling in Substrate can *brick* the blockchain, rendering it useless thereafter. With this in mind, it is very important to structure code according to declarative, condition-oriented design patterns. *See more in the [declarative programming](./cop.md) section.*