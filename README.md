# <a href="https://substrate.dev/recipes">Substrate Recipes</a> 🍴😋🍴
![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fsubstrate-developer-hub%2Frecipes%2Fbadge%3Fref%3Dmaster&style=flat)
![Lines of Code](https://tokei.rs/b1/github/substrate-developer-hub/recipes)

_A Hands-On Cookbook for Aspiring Blockchain Chefs_

## Get Started
Ready to roll up your sleeves and cook some blockchain? Read the book online at https:/substrate.dev/recipes

## Repository Structure
There are four primary directories in this repository:

* **Text**: Source of [the book](https://substrate.dev/recipes) written in markdown. This text describes the code in the other three directories.
* **Pallets**: Complete pallets for use in FRAME-based runtimes.
* **Runtimes**: Complete runtimes for use in Substrate nodes.
* **Nodes**: Complete Substrate nodes ready to run.

The book is built with [mdbook](https://rust-lang-nursery.github.io/mdBook/) and deployed via [github pages](https://pages.github.com/).

## License
<a rel="license" href="http://creativecommons.org/licenses/by/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">Creative Commons Attribution 4.0 International License</a>. It is open source and [open for contributions](./CONTRIBUTING.md).

## Using Recipes in External Projects

The pallets and runtimes provided here are tested and ready to be used in other Substrate-based blockchains. The big caveat is that you must use the same upstream Substrate version throughout the project. The recipes currently use Substrate@`v2.0.0-alpha.3`. This situation should improve when Substrate begins releasing on crates.io
