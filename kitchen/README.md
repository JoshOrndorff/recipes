# Kitchen
[![](https://tokei.rs/b1/github/substrate-developer-hub/recipes)](https://github.com/substrate-developer-hub/recipes)

The kitchen is for *cooking* recipes. It is structured similarly to the main recipes build as specified in [src/SUMMARY.md](../src/SUMMARY.md), except every code sample is stored as a library via the [substrate-module-template](https://github.com/substrate-developer-hub/substrate-module-template).

There are three sections:
* [Modules](./modules/README.md): individual modules, formatted as libraries
* [Runtimes](./runtimes/README.md): module configurations for executable runtimes
* [Node](./node/README.md): node configuration using the runtimes

## Directions

Within each folder, run the following command to build

```rust, ignore
cargo build
```

I haven't written unit tests *yet*, but you can write tests in the [usual way](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html), except with a bit more [scaffolding](https://www.shawntabrizi.com/substrate-collectables-workshop/#/5/setting-up-tests).

## Using Recipes in External Projects

Follow the [substrate-module-template](https://github.com/substrate-developer-hub/substrate-module-template) directions to use recipes in your personal projects. 

**I recommend extracting patterns and applying them in the context of your application rather than directly importing the recipes** :)