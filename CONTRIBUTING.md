# Contributing Guidelines

**Recipes are not entire tutorials -- they are small patterns that may be extracted from tutorials**. The purpose of Substrate Cookbook is to identify best practies in Substrate runtime code and extract patterns that are useful outside of the context of the specific use case.
    
1. isolate specific pattern
2. walk through logic in piecewise steps
3. show/link to the full file in the used codebase

If you have want to get involved, feel free to open an [issue](https://github.com/substrate-developer-hub/recipes/issues/new) with any ideas/comments/questions.**The markdown for each recipe can be found by following the paths set in [SUMMARY.md](./src/SUMMARY.md)**.

I'm going to spend more time working on samples over the next few weeks, but I'm in the process of improving this project as well.

## Common Etiquette

* try to not use "we" or "our" because it often is conducive to unncessary language
* frequently link to outside content (home of the original code, blog/tutorial references, documentation for a specific method/trait/etc)

## Local Build Instructions

I recommend staging locally before making a PR. Don't forget to switch to a new branch before you make edits.

1. install [`mdbook`](https://github.com/rust-lang-nursery/mdBook)

```bash
$ cargo install mdbook
```

2. build and open rendered book in default browser

```bash
$ mdbook build --open
```

3. If everything looks good, open a [Pull Request](https://github.com/substrate-developer-hub/recipes/compare)