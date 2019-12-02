# Contributing Guidelines

The **purpose** of [Substrate Recipes](https://substrate.dev/recipes/) is to identify best practices in Substrate runtime code and extract useful patterns. If you're eager to PR, skip to the [Chef's Workflow](#workflow) section.

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

https://substrate.dev/recipes/base/kitchenoverview.html

TODO: break the algorithm into 2-sentence paragraphs with more specificity per paragraph

*algorithm*
1. isolate useful pattern and make an [`issues`](https://github.com/substrate-developer-hub/recipes/issues)
2. clone and a fork a branch; build a module/runtime example with context in the [`kitchen`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen) directory
3. walk through logic of useful pattern in piecewise steps (in [`src/`](https://github.com/substrate-developer-hub/recipes/tree/master/src))
4. link `src` and `kitchen` (in [`src/`](https://github.com/substrate-developer-hub/recipes/tree/master/src)) 

### TODO: Cooking Modules

TODO:
* preferences on style?
* enforcement of `cargo fmt`
* maybe of the format of the `Cargo.toml`?

### Editing and Writing Recipes

See the [Template](./src/TEMPLATE.md) for recommendations on structure. **No structure is enforced** so feel free to do things however you feel comfortable doing them :)

For showing Rust in the Recipes writing, prefer annotating code blocks with `rust, ignore` because the current compilation environment doesn't maintain the substrate dependencies.

```rust, ignore
pub fn fake_method() {
    nothing()
}
```

**Stage locally before making a PR.** Don't forget to switch to a new branch before you make edits.

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