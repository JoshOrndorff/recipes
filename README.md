# <a href="https://substrate.dev/recipes">Substrate Recipes</a> 🍴😋🍴
![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fsubstrate-developer-hub%2Frecipes%2Fbadge%3Fref%3Dmaster&style=flat)
![Lines of Code](https://tokei.rs/b1/github/substrate-developer-hub/recipes)

_A Hands-On Cookbook for Aspiring Blockchain Chefs_

## Get Started
Ready to roll up your sleeves and cook some blockchain? Read the book online at [substrate.dev/recipes](https://substrate.dev/recipes)

## Repository Structure
There are five primary directories in this repository:

* **Text**: Source of [the book](https://substrate.dev/recipes) written in markdown. This text describes the code in the other three directories.
* **Pallets**: Pallets for use in FRAME-based runtimes.
* **Runtimes**: Runtimes for use in Substrate nodes.
* **Consensus**: Consensus engines for use in Substrate nodes.
* **Nodes**: Complete Substrate nodes ready to run.

The book is built with [mdbook](https://rust-lang-nursery.github.io/mdBook/) and deployed via [github pages](https://pages.github.com/).

## License
The Substrate Recipes are [GPL 3.0 Licensed](LICENSE) It is open source and [open for contributions](./CONTRIBUTING.md).

## Using Recipes in External Projects

The pallets and runtimes provided here are tested and ready to be used in other Substrate-based blockchains. The big caveat is that you must use the same upstream Substrate version throughout the project.
