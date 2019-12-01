# Contributing Guidelines

The **purpose** of [Substrate Recipes](https://substrate.dev/recipes/) is to identify best practices in Substrate runtime code and extract useful patterns.

* [Template](./src/TEMPLATE.md)
* [Scope and Structure](#scope)
* [Getting Involved](#involve)
* [Mdbook Local Build Instructions](#instructions)
* [Etiquette](#etiquette)

## Scope and Structure <a name = "scope"></a>

At the moment, the recipes onboards developers by focusing primarily on **module development** patterns before reusing these patterns in the context of **runtime configuration** (runtime section is *in-progress*).

The **[kitchen](./kitchen)** contains code from the recipes in the context of modules/runtimes, while **[src](./src)** houses the commentary for related explanations and references. The structure of the `src` can be found in [src/SUMMARY.md](./src/SUMMARY.md).

In practice, the recipes supplements existing resources by providing usage examples, demonstrating best practices in context, and extending simple samples/tutorials. Likewise, it is necessary to **frequently link to/from [reference docs](https://crates.parity.io/substrate_service/index.html?), [tutorials](https://github.com/substrate-developer-hub/), and [high-level docs](https://substrate.dev/)**.

The recipes do NOT cover:
* module testing
* rpc, cli, and other [`node/`](https://github.com/paritytech/substrate/tree/master/node) stuff outside the runtime
* frontend UI
* protocol engineering (consensus, networking, etc.)

If you're interested in adding new chapters for any of these sections, [create an issue](https://github.com/substrate-developer-hub/recipes/issues/new) and convince us :)

## Getting Involved <a name = "involve"></a>

1. isolate useful pattern (in [`issues`](https://github.com/substrate-developer-hub/recipes/issues))
2. build a module/runtime example with context (in [`kitchen`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen))
3. walk through logic of useful pattern in piecewise steps (in [`src/`](https://github.com/substrate-developer-hub/recipes/tree/master/src))
4. link `src` and `kitchen` (in [`src/`](https://github.com/substrate-developer-hub/recipes/tree/master/src)) 

## Local Build Instructions <a name = "instructions"></a>

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

## Etiquette <a name = "etiquette"></a>

* don't use "we" or "our" because it often is conducive to unnecessary language
* use active voice (instead of passive voice like "you may want to use active voice", prefer "use active voice")
* link as often as possible to outside content and useful resources including other recipes, [documentation](https://substrate.dev/docs/en/getting-started/), [tutorials](https://substrate.dev/en/tutorials), and [code](https://github.com/substrate)
* **be nice, abide by the [Rust CoC](https://www.rust-lang.org/policies/code-of-conduct)**

For blocks of rust, prefer annotating code blocks with `rust, ignore` because the current compilation environment doesn't maintain the substrate dependencies.

```rust, ignore
pub fn fake_method() {
    nothing()
}
```