# Version 0.1

**content**
* relative *completion*, but still much room for improvement `=>` the project's intended scope is covered with many basic and a few advanced patterns
* the `kitchen` contains examples of a node, runtimes, and modules
* recent new recipes:
    * child-trie storage
    * instanceable modules
    * execution schedule
    * testing section
* in theory, this project should experience diminishing returns, but there is still a high frequency of significant changes and new features

**devops**
* the [book](https://substrate.dev/recipes/) is built and deployed with travis
* #95 consolidates `cargo check` and `cargo test` into one CI workflow handled by Github Actions (mdbook deploy script to be added soon)