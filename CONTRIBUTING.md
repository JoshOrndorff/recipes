# Contributing Guidelines

The **purpose** of [Substrate Recipes](https://substrate.dev/recipes/) is to identify best practices and extract useful patterns for developing on Substrate, and to present those patterns in an approachable and fun format.

The recipes onboards aspiring blockchain developers by focusing first on **FRAME pallet development** patterns before reusing these patterns in the context of **runtime configurations**, and finally installing those runtimes in full substrate-based nodes.

The recipes supplements existing resources by providing usage examples, demonstrating best practices in context, and extending simple samples/tutorials. Likewise, it is necessary to **frequently link to/from [reference docs](https://substrate.dev/rustdocs/master/), [tutorials](https://substrate.dev/tutorials/), and [high-level docs](https://substrate.dev/docs)**.

## Chef's Workflow

1. Isolate useful pattern(s) from the main [Substrate](https://github.com/paritytech/substrate) repository and other code that builds on Substrate.

2. Open an [issue](https://github.com/substrate-developer-hub/recipes/issues/new) describing how the pattern can be condensed into a recipe.

3. Fork the [recipes](https://github.com/substrate-developer-hub/recipes). Before making any changes, checkout another branch.

4. Write complete working code in one of the `pallets`, `runtimes`, or `nodes` directories, and add it the the workspace in `Cargo.toml`.

5. Write the companion text in the `text` directory, referencing the code you wrote.

6. Stage your changes locally (see below).

7. Open a new pull request. Thanks for you contributions <3.

### Testing and Staging Locally
```bash
# Test code
cargo test --all

# Install mdbook
cargo install mdbook

#Build and open rendered book in default browser
mdbook build --open
```

## Style

### Rust Code
There is not yet strict enforcement of the [Rust in Substrate coding style](https://wiki.parity.io/Substrate-Style-Guide), but it is highly encouraged to wrap lines at 120 characters a line (or less) for improving reviewer experience on github.

Graciously invoke `cargo fmt` on any Rust code -- this should soon be enforced by CI!

### Cargo.toml
Prefer listing dependencies under a single `[dependencies]` header in lieu of using a `[dependencies.some_import]` for every `some_import` module imported. This preference is not enforced. See [`adding-machine/Cargo.toml`](https://github.com/substrate-developer-hub/recipes/blob/master/pallets/adding-machine/Cargo.toml) for an example of the recommended less verbose `Cargo.toml` syntax.

### English
No standards for language style are enforced aside from the common English spelling/grammar rules. @4meta5 has a few *preferences*:
* Avoid using "we", "our", "you" because it often is conducive to unnecessary language
* Prefer active voice (instead of "you may want to use active voice" `=>` "use active voice")
* Link as often as possible to outside content and useful resources including other recipes, docs,
tutorials, and code. It is not necessary to re-link the same external resource on subsequent
mentions in a single document.

## Test Coverage
Not all code is covered, and not all covered code is covered well, but we have a good start and will continue to improve. Your help is warmly welcomed.
Run the tests with `cargo test -- all`.
