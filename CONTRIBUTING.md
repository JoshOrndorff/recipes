# Contributing Guidelines

The **purpose** of [Substrate Recipes](https://substrate.dev/recipes/) is to identify best practices in Substrate runtime code and extract useful patterns. If you're eager to PR, jump to the [Chef's Workflow](#workflow) section for clear instructions to guide contributions.

## Scope: Substrate Module and Runtime Development <a name = "scope"></a>

At the moment, the recipes onboards developers by focusing primarily on **module development** patterns before reusing these patterns in the context of **runtime configuration** (runtime section is *in-progress*).

The **[kitchen](./kitchen)** contains code from the recipes in the context of modules/runtimes, while **[src](./src)** houses the commentary for related explanations and references (displayed at https://substrate.dev/recipes ). The structure of the `src` can be found in [src/SUMMARY.md](./src/SUMMARY.md).

In practice, the recipes supplements existing resources by providing usage examples, demonstrating best practices in context, and extending simple samples/tutorials. Likewise, it is necessary to **frequently link to/from [reference docs](https://crates.parity.io/substrate_service/index.html?), [tutorials](https://github.com/substrate-developer-hub/), and [high-level docs](https://substrate.dev/)**.

The recipes do NOT cover:
* module testing
* rpc, cli, and other [`node/`](https://github.com/paritytech/substrate/tree/master/node) stuff outside the runtime
* frontend UI
* protocol engineering (consensus, networking, etc.)

If you're interested in adding new chapters for any of these sections, [create an issue](https://github.com/substrate-developer-hub/recipes/issues/new) and convince us :)

## Chef's Workflow <a name = "workflow"></a>

1. Isolate useful pattern(s) from [Substrate](https://github.com/paritytech/substrate) and code written with Substrate.

2. (optional) Write an issue to discuss organizing the pattern into a written recipe in `recipes/src` or a code example in `recipes/kitchen`

3. Fork the [recipes](https://github.com/substrate-developer-hub/recipes). Before making any changes, checkout another branch. 

4. (optional) To contribute a code example, place the code under the appropriate directory in [`recipes/kitchen`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen).  

5. (optional) To write a recipe, discuss where it should go in `src` in an issue in step (2). [`SUMMARY.md`](./src/SUMMARY.md) describes the structure of the [book](https://substrate.dev/recipes). [`TEMPLATE.md`](./src/TEMPLATE.md) provides a recommended structure.

There should be high coverage of the `kitchen` code in the written content (in `src`), but this is not necessarily enforced.

### Cooking Modules

We do not enforce the [Rust in Substrate coding style](https://wiki.parity.io/Substrate-Style-Guide), but we prefer line wrapping at 120 characters a line (or less) for ease of review on github. 

Graciously invoke `cargo fmt` on module and runtime code -- this should soon be enforced by a script!

For `Cargo.toml` files, prefer listing dependencies under a single `[dependencies]` header in lieu of using a `[dependencies.some_import]` for every `some_import` module imported. This preference is not enforced. See [`adding-machine/Cargo.toml`](https://github.com/substrate-developer-hub/recipes/blob/master/kitchen/modules/adding-machine/Cargo.toml) for an example of the recommended less verbose `Cargo.toml` syntax.

### Editing and Writing Recipes

See the [Template](./src/TEMPLATE.md) for recommendations on structure. **No structure is enforced** so feel free to do things however you feel comfortable doing them :)

For showing Rust in the Recipes writing, prefer annotating code blocks with `rust, ignore` because the current compilation environment doesn't maintain the substrate dependencies.

```rust, ignore
pub fn fake_method() {
    nothing()
}
```

**Don't forget to stage locally before making a PR.**

1. install [`mdbook`](https://github.com/rust-lang-nursery/mdBook)

```bash
$ cargo install mdbook
```

2. build and open rendered book in default browser

```bash
$ mdbook build --open
```

3. If everything looks good, open a [Pull Request](https://github.com/substrate-developer-hub/recipes/compare)

No standards for language style are enforced aside from the common english spelling/grammar rules. @4meta5 has a few *preferences*:
* avoid using "we", "our", "you" because it often is conducive to unnecessary language
* prefer active voice (instead of "you may want to use active voice" `=>` "use active voice")
* link as often as possible to outside content and useful resources including other recipes, docs, tutorials, and code