# Module Testing

Although the Rust compiler ensures safe memory management, it cannot formally verify the correctness of a program's logic. Fortunately, Rust also comes with a convenient suite for writing unit and integration tests. When you initiate code with Cargo, test scaffolding is automatically generated to simplify the developer experience. Testing concepts and syntax are covered in depth in [Chapter 11 of the Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).

* [Mock Runtime for Unit Testing](./mock.md)

There's also more rigorous testing systems ranging from mocking and fuzzing to formal verification. See [quickcheck](https://docs.rs/quickcheck/0.9.0/quickcheck/) for an example of a property-based testing framework ported from Haskell to Rust.

## Kitchen Unit Test Coverage

Unit tests have been written for the following kitchen modules:
- [`adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/adding-machine)
- [`simple-event`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/simple-event)
- [`generic-event`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/generic-event)
- [`single-value`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/single-value)
- [`simple-map`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/simple-map)
- [`double-map`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/double-map)
- [`storage-cache`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/storage-cache)
- [`vec-set`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/vec-set)
- [`module-constant-config`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/module-constant-config)

To run the tests, clone the repo 

```bash
$ git clone https://github.com/substrate-developer-hub/recipes
```

Enter the path to the module to be tested

```bash
recipes git:(some-branch) ✗ cd kitchen/modules/<some-module>
```

For example, to test `module-constant-config`, used in [Configurable Constants](https://substrate.dev/recipes/storage/constants.html), 

```bash
recipes git:(some-branch) ✗ cd kitchen/modules/module-constant-config/
module-constant-config git:(some-branch) ✗ cargo test
```

Writing unit tests is one of the best ways to understand the code. Although unit tests are not comprehensive, they provide a first check to verify that the programmer's basic invariants are not violated in the presence of obvious, expected state changes.