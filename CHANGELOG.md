# Version 0.1

**content**
* the `kitchen` contains examples of a node, runtimes, and modules
* recent new recipes:
    * child-trie storage
    * instanceable modules
    * execution schedule
    * testing section (+3)
* in theory, this project should experience diminishing returns, but there is still a high frequency of significant changes and new features

**devops**
* the [book](https://substrate.dev/recipes/) is built and deployed with travis; analytics are used in the `book.toml` config for `mdbook` and stored in the `analytics/` folder
* #95 consolidates `cargo check` and `cargo test` into one CI workflow handled by Github Actions (mdbook deploy script to be added soon)