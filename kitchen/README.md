# Kitchen

The kitchen is for *cooking* recipes. It is structured like the main recipes build as specified in [src/SUMMARY.md](../src/SUMMARY.md), except every code sample is stored as a library via the [substrate-module-template](https://github.com/shawntabrizi/substrate-module-template).

## Directions

Within a recipe's folder, run the following command to build

```rust
cargo build
```

I haven't written unit tests *yet*, but you can write tests in the [usual way](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html), except with a bit more [scaffolding](https://www.shawntabrizi.com/substrate-collectables-workshop/#/5/setting-up-tests).

## Using Recipes in External Projects

Follow the [substrate-module-template directions](https://github.com/shawntabrizi/substrate-module-template) to use recipes in your personal projects. 

**I recommend extracting patterns and applying them in the context of your application rather than directly importing the recipes** :)