# Substrate Runtime Recipes Cookbook
> mirror of [Substrate Runtime Recipes](https://substrate.readme.io/docs/substrate-runtime-recipes)

* [Storage](./storage)
* [Events](./event)
* [Security](./security)
* [Advanced Patterns](./advanced)

I am considering expanding this list to include the following:
* [Cryptography](#crypto) (hashing data, encryption, zero knowledge (layerx))
* Balance/Currency (transfers, staking, token standards)
* Governance (`democracy`, consensus modules)
* Common Rust Patterns (state machine -> `futures`, heap allocation, etc)

## Contribution Guidelines
* LINK TO OUTSIDE STUFF A LOT!
    * link to other sections often to illustrate how things work together
    * whenever you reference anything from an external source, link to it
    * **whenever you add a reference to code, link to documentation**
        * (I find it extremely annoying when I read about a macro or trait and have to look it up in the documentation directly)
    
* Basic Pattern to Follow:
    * isolate specific pattern
    * show the full file (w/ necessary imports)
    * any necessary imports/inclusions in other files (final version)

## Plan

* use mdbook for presentation
    * use the async book or another book as an example
        * syntax highlighting
        * launch from github pages
    
* use **SourceGraph** to search the codebase for code examples
    * great practice with using **SourceGraph** effectively
    * should get the other devs hip if they aren't

* add a CONTRIBUTING.md file in the head of the directory with *guidelines for contribution*
    * (have a list of things to do with clear instructions as well) -- organize as `Issues` and fix with `Pull Requests`