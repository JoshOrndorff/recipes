# Testing

Although the Rust compiler ensures safe memory management, it cannot formally verify the correctness of a program's logic. Fortunately, Rust also comes with great libraries and documentation for writing unit and integration tests. When you initiate code with Cargo, test scaffolding is automatically generated to simplify the developer experience. Basic testing concepts and syntax are covered in depth in [Chapter 11 of the Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).

* [Basic Test Environments](./mock.md)
* [Common Tests](./common.md)
* [Custom Test Environment](./externalities.md)

There's also more rigorous testing systems ranging from mocking and fuzzing to formal verification. See [quickcheck](https://docs.rs/quickcheck/0.9.0/quickcheck/) for an example of a property-based testing framework ported from Haskell to Rust.

## Kitchen Pallets with Unit Tests

The following modules in the [`kitchen`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/) have partial unit test coverage
- [`struct-storage`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/struct-storage)
- [`adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/adding-machine)
- [`simple-event`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/simple-event)
- [`generic-event`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/generic-event)
- [`single-value`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/single-value)
- [`simple-map`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/simple-map)
- [`double-map`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/double-map)
- [`storage-cache`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/storage-cache)
- [`vec-set`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/vec-set)
- [`constant-config`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/constant-config)

### Cooking in the Kitchen (Running Tests)

To run the tests, clone the repo

```bash
$ git clone https://github.com/substrate-developer-hub/recipes
```

Enter the path to the pallet to be tested

```bash
$ cd kitchen/pallets/<some-module>
```

For example, to test `constant-config`, used in [Configurable Constants](https://substrate.dev/recipes/storage/constants.html),

```bash
$ cd kitchen/pallets/constant-config/
$ cargo test
```

Writing unit tests is one of the best ways to understand the code. Although unit tests are not comprehensive, they provide a first check to verify that the programmer's basic invariants are not violated in the presence of obvious, expected state changes.

## Sauce

Over the past few weeks, testing has driven a significant rewrite of the [kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/). This increased focus on testing and benchmarking will continue over the next few weeks in the *[sauce](https://github.com/substrate-developer-hub/recipes/tree/master/src/tour)*, starting with

- [`execution-schedule`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/execution-schedule)
- [`smpl-treasury`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/smpl-treasury)
