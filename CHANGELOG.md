# Version 0.1

**content**
* relative *completion*, but still much room for improvement `=>` the project's intended scope is covered with many basic and a few advanced patterns
* the `kitchen` contains examples of a node configuration, runtimes, and modules
* recent new recipes:
    * child-trie storage
    * instanceable modules
    * execution schedule
    * testing section
* in theory, this project should experience diminishing returns, but there is still a high frequency of significant changes and new features

**devops**
* the [book](https://substrate.dev/recipes/) is built and deployed with travis
* the `kitchen` is run and checked with `CircleCI`
* github actions is used to test the modules in the `kitchen/modules`