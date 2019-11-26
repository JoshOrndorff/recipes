# Version 0.1

**content**
* relative *completion*, but still much room for improvement `=>` the project's intended scope is covered with many basic and a few advanced patterns
* the `kitchen` contains examples of a node configuration, runtimes, and modules
* recent new recipe on `execution-schedule` for `on_initialize`, `on_finalize`, and space for an offchain-worker example
* recently introduced a section for unit testing with substrate
    * need better visibility of test coverage for the `kitchen/modules`
* in theory, this project should experience diminishing returns, but there is still a high frequency of drastic changes and new features

**devops**
* the [book](https://substrate.dev/recipes/) is built and deployed with travis
* the `kitchen` is run and checked with `CircleCI`
* looking into using github actions to test modules; next I want to display some test coverage button