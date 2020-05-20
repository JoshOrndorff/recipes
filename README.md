# Substrate Recipes 🍴😋🍴
![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fsubstrate-developer-hub%2Frecipes%2Fbadge%3Fref%3Dmaster&style=flat)
![Lines of Code](https://tokei.rs/b1/github/substrate-developer-hub/recipes)

_A Hands-On Cookbook for Aspiring Blockchain Chefs_

Substrate Recipes is a cookbook of working examples and best practices of building blockchains with **[Substrate](https://github.com/paritytech/substrate)**. Each recipe contains a complete working code example as well as a detailed writeup describing the code.

## How to Use This Book
Ready to roll up your sleeves and cook some blockchain? Read the book online at [substrate.io/recipes](https://substrate.io/recipes).

When you're ready to start hacking with the recipes, follow the instructions to [prepare your kitchen](prepare-your-kitchen/README.md). There you will set up your toolchain, compile a blockchain node, learn the structure of the recipes repository, and interact with a running blockchain.

There are recipes at targeting all levels of skill and complexity, so you should be right at home regardless of your experience.

Remember, you can't learn to cook by reading alone. As you work through the book, put on your apron, get out some pots and pans, and practice compiling, testing, and hacking on the recipes. Play with the code in the kitchen, extract patterns, and apply them to a problem that you want to solve!

## Getting Help

When learning any new skill, you will inevitably get stuck at some point. When you do get stuck you can seek help in several ways:

* Ask a question on [Stack Overflow](https://stackoverflow.com/questions/tagged/substrate)
* Ask a question in the [Substrate Technical Riot channel](https://riot.im/app/#/room/#substrate-technical:matrix.org)
* Open a [new issue](https://github.com/substrate-developer-hub/recipes/issues/new) against this repository

## Repository Structure
There are five primary directories in this repository:

* **Text**: Source of [the book](https://substrate.dev/recipes) written in markdown. This text describes the code in the other three directories.
* **Pallets**: Pallets for use in FRAME-based runtimes.
* **Runtimes**: Runtimes for use in Substrate nodes.
* **Consensus**: Consensus engines for use in Substrate nodes.
* **Nodes**: Complete Substrate nodes ready to run.

The book is built with [mdbook](https://rust-lang-nursery.github.io/mdBook/) and deployed via [github pages](https://pages.github.com/).

## Using Recipes in External Projects

The pallets and runtimes provided here are tested and ready to be used in other Substrate-based blockchains. The big caveat is that you must use the same upstream Substrate version throughout the project.

## Contributing
The Substrate Recipes are [GPL 3.0 Licensed](LICENSE) It is open source and [open for contributions](./CONTRIBUTING.md).
