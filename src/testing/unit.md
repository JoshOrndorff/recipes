# Unit Testing
* writing unit tests for test-driven development
    * conditional paths and testing panics...avoid when possible, but otherwise, do it

* which modules provide great examples of unit testing best practices
* test all branches of conditional paths with something like the `weight_limit` test I wrote in `srml/executive` (closure with paths as condition)
* call individual tests by their name with `cargo test __name__`

Unit tests are nowhere as rigorous or comprehensive as fuzzing or formal verification, but they help developers identify simple logic errors.

## References

* [Design for Testability](https://blog.nelhage.com/2016/03/design-for-testability/)
* [How I Test](https://blog.nelhage.com/2016/12/how-i-test/)
* [cfg pattern and more](https://os.phil-opp.com/unit-testing/)

* [Simple Testing Can Prevent Most Critical Failures](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-yuan.pdf)