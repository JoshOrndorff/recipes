# Collective Notes

I'm down to document `collective`, but only after I write some high-level docs on it that get some review.

## Hmmm

* I like the idea of using `council` voting to delegate to a special committee `=>` (1) council election (2) delegate stake to members (using Phragmen-based delegation)?

### Questions

* why does instancing introduce an `rstd::marker::PhantomData` type? This is used in the `EnsureMember` wrapper struct later as well...

* `Into` and `From` traits

* is stable sort better if you receive a sorted list? Otherwise, why is stable sort used in `collective`? I was under the impression, that unstable sorting was [better in terms of memory usage](https://stackoverflow.com/a/1517930/11637659)

* is line 152 comparing `AccountId` when it should be comparing the index?

* shouldn't we be using `put_ref` instead of `push` for vectors now? See `line 229` which uses `push` and has a clone...

* `execute` allows any member to dispatch an origin `=>` shouldn't there be some permissions or a verification of the `proposal` state?

* the `ProposalCount` iteration doesn't check for overflows? **find the correct pattern** for this...it might be a `if let Some(number) = blank.checked_add(new_numebr) {}`...Joe answered it, but I lost the question? https://github.com/paritytech/substrate/pull/3071