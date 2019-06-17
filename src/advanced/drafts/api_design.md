# API Design

* api design
* traitify

## Future-Proofing and Backwards Compatibility

Substrate provides novel metagovernance capabilities which can be invoked, but it is much easier to swap parts of the runtime vs rewriting the entire thing.

This sort of conveys the difference btween the soft fork vs hard fork, except neither change results in a new chain in this context. 

Anyway, there are a few ways to make your implementation more backwards compatibke:

* `__non_exhaustive_ ();` at the end of match statements (and using the `#[derive()]` feature
* https://rust-lang-nursery.github.io/api-guidelines/future-proofing.html