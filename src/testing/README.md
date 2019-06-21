# Testing Substrate

Although the Rust compiler ensures safe memory management, it cannot formally verify the correctness of a program's logic. Fortunately, Rust also comes with a convenient suite for writing unit and integration tests. When you initiate code with Cargo, test scaffolding is automatically generated to simplify the developer experience. Testing concepts and syntax are covered in depth in [Chapter 11 of the Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).

* [Scaffolding](./scaffolding.md)
* [Unit Testing](./unit.md)

There's also more rigorous testing systems ranging from mocking and fuzzing to formal verification. Once best practices for these patterns starts to become clear in the context of Substrate, related recipes will be added.