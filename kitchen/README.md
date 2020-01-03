# Kitchen
[![](https://tokei.rs/b1/github/substrate-developer-hub/recipes)](https://github.com/substrate-developer-hub/recipes)

The kitchen contains full working code examples, ready for *cooking*! Each topic discussed in the book has a recipe and there are further examples on some topics.

There are three sections:
* [Pallets](./pallets/README.md): individual pallets, for use in FRAME runtimes.
* [Runtimes](./runtimes/README.md): complete Substrate runtimes composed from FRAME pallets.
* [Node](./node/README.md): a BABE and GRANDPA node that supports either runtime.

## Directions

Build the kitchen node with `cargo build --release -p kitchen-node`
Then run it with `./target/release/kitchen-node --dev`

## Test Coverage
Not all code is covered, and not all covered code is covered well, but we have a good start and will continue to improve. Run the tests with `cargo test -- all`.

## Using Recipes in External Projects

The pallets and runtimes provided here are ready to be used in other Substrate-based blockchains. The big caveat is that you must use the same upstream Substrate version throughout the project. Check which version the recipes uses in any of the `Cargo.toml` files. This situation should improve when Substrate begins releasing on crates.io
