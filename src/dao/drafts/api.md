# Ergonomic API

* api design
* traitify

*optimize for readability and auditability before focusing only on increasing performance of critical sections (reference `safety/optimizations.md`*

*reference `testing/unit.md` for test-based design which encourages ergonomicity*

* can someone explain this put trait bounds on structs or don't put trait bounds on structs discussion? Some people just say to prefer `derive` attribute...

## Type Aliasing and Organization

Looking through the `srml`, it is clear that type aliasing is used frequently to separate and clearly display key parameters in governance processes.

In `srml/council`, this is used for the `MemberCount = u32` and `ProposalIndex = u32`.

* idea for project with Kian `=>` organize a `fees` module that includes the types referenced at the top of `Balances` `=>` refactor existing code by moving relevant logic from `node/runtime` to `srml/fees` and then importing `fees` in the usual way

## Future-Proofing and Backwards Compatibility

Substrate provides novel metagovernance capabilities which can be invoked, but it is much easier to swap parts of the runtime vs rewriting the entire thing.

This sort of conveys the difference btween the soft fork vs hard fork, except neither change results in a new chain in this context. 

Anyway, there are a few ways to make your implementation more backwards compatibke:

* `__non_exhaustive_ ();` at the end of match statements (and using the `#[derive()]` feature
* https://rust-lang-nursery.github.io/api-guidelines/future-proofing.html