# Contributing Guidelines

The Substrate Recipes strive to identify and extract useful patterns and examples of developing on
Substrate, and to present those patterns in an approachable and fun format. **Your help is
Welcome!**

The Recipes are part of the Substrate Developer Hub, so inter-linking between the Recipes and other
DevHub facets is common. In particular, the Recipes frequently links to:

-   The [Reference Docs](https://substrate.dev/rustdocs/v2.0.0-rc2/)
-   The [Tutorials](https://substrate.io/tutorials/)
-   The [Knowledge Base](https://substrate.io/kb/learn-substrate)

## Reporting Bugs

One way to contribute to the Recipes, is to report issues you find when using the Recipes. You may
report new issues at https://github.com/substrate-developer-hub/recipes/issues. When reporting an
issue, please inlcude the following information.

-   **Short summary of the issue encountered** - In a sentence or two explain what the issue is at a
    high level.
-   **What recipe has the issue** - You may specify the recipe by name (e.g. "Basic PoW Node"),
    directory (eg. `/nodes/basic-pow`), or GitHub link (eg.
    https://github.com/substrate-developer-hub/recipes/tree/master/nodes/basic-pow). Other
    unambiguous ways of specifying the particular recipe are also acceptable (e.g. crates.io link, or
    rendered text link).
-   **Steps to reproduce** - What actions did you take to notice the issue? Did you submit a
    particular extrinsic? Did you compile the code a particular way? What command did you run the
    node with?
-   **Expected Behavior** - What were you expecting to happen when you encountered the bug?
-   **Observed Behavior** - What actually happened when you encountered the bug?
-   **Additional Relevant Information** - What error message did you receive? What OS and rust
    compiler are you using?

## Git Workflow

The recipes adhere closely to
[gitflow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow). That means
there are two main branches in the repository, `master`, and `develop` as well as any number of
topic branches. If you aren't familiar with gitflow, it is worth reading that article and studying
the diagrams. One of the diagrams is included below.

![gitflow](<https://wac-cdn.atlassian.com/dam/jcr:a9cea7b7-23c3-41a7-a4e0-affa053d9ea7/04%20(1).svg?cdnVersion=1017>)

### Master Branch

The `master` branch contains stable, published code and is where release versions are tagged.
Released versions will always be in the history of the `master` branch. The code on `master` uses
published dependencies from `crates.io` and uses git dependencies only where absolutely necessary
likely because relevant crates.io does not yet host the relevant crates.

### Develop Branch

The `develop` branch is where active development happens. It is where new recipes,
revisions, CI updates, and most other changes are merged. In order to keep up with the latest
Substrate development, the `develop` branch allows dependencies from git.

### Cutting a Release

It is time to tag a new release when either enough new features have been contributed that
releasing makes sense, or, more likely, Substrate itself has tagged a new release. The release
process also follows gitflow. Creating a new release is usually initiated by the
project maintainer, but the steps are outlined here nonetheless.

1. Create a release branch off of `develop`.
1. Update dependencies to crates.io.
1. Update the version of each Recipes crate.
1. Update rustdocs links from crates.parity.io to substrate.dev/rustdocs/_appropriate-version_
1. Make a pull request against `master`.
1. Tag the new release version in the history of `master` only after the release branch is merged.

## Proposing Changes and Additions

If you would like to make a change or addition to the recipes, you do not need anyone's permission
to get started. You simply open a Pull Request against the `develop` branch. Of course, not all
changes will be accepted, and changes should either be in line with the existing Recipes structure
or refactor that structure for a good reason. If you want preliminary input from the
Recipes' maintainers before beginning, please
[open an issue](https://github.com/substrate-developer-hub/recipes/issues) discussing your idea
first. Either approach (PR or issue) is welcome.

### What to Contribute

Anything you think will make the Recipes better, is worth proposing. Here are some ideas to get you
started. All of these ideas and more are listed in our
[issue queue](https://github.com/substrate-developer-hub/recipes/issues)

-   **Test Coverage** - Not all code is covered, and not all covered code is covered well, but we
    would like more and better coverage.
-   **New recipes** - If you know how to do something useful in Substrate that is not yet covered in
    the Recipes, please contribute.
-   **UX improvements** - Any way to make it easier and less confusing to get new users onboarded
    is welcome.
-   **CI Improvements** - The more tests we have automated, the higher quality the Recipes will be.

## Style

### Rust Code

There is not yet strict enforcement of the
[Rust in Substrate coding style](https://wiki.parity.io/Substrate-Style-Guide), but it is highly
encouraged to wrap lines at 120 characters a line (or less) for improving reviewer experience on
github.

Graciously invoke `cargo fmt` and `cargo clippy` on any Rust code; This should soon be enforced by
CI.

### Cargo.toml

Prefer listing dependencies under a single `[dependencies]` header in lieu of using a
`[dependencies.some_import]` for every `some_import` module imported.

### English

No standards for language style are enforced aside from the common English spelling/grammar rules.
@4meta5 has a few _preferences_:

-   Avoid using "we", "our", "you" because it often is conducive to unnecessary language
-   Prefer active voice ("you may want to use active voice" `=>` "use active voice")
-   Link as often as possible to outside content and useful resources including other recipes,
    knowledge base, tutorials, Wikipedia, and 3rd party content. It is not necessary to re-link the
    same external resource on subsequent mentions in a single document.
