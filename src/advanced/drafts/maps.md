# Hash Patterns

When deciding how to store proposals in `decl_storage {}`, we can either
1. hash each proposal and use the hash as a unique identifier for relevant maps
2. use a unique proposalIndex for each

## Initial Implementation
> [code](./old.rs)

The first iteration of Molochameleon actually used a `type ProposalIndex = u32` to track proposals.

> pick example code patterns and show why this is hard

* technically it constrains the number of possible key-value pairs
* moreso, there is head-of-line blocking and really awkard clean up
    * it's a very synchronous, rigid pattern for managing proposals

### Treasury (Frame Example)

So why does Treasury use indexes? Well sometimes we want to closely limit proposal throughput instead of optimzing protocol flexibility.

Also, using indexes may yield a more readable implementation. Readability and maintainability should always be regarded as key criteria when designing your pallet. This isn't some undergraduate CS project that you're hacking together to just run successfully; mistakes are expensive and ergonomic code is a necessity for careful audits.

## Migrating to Using Hashes as Unique Identifiers

* sentence on the magic of hash functions

Parity's hash functions operate on a bit digest so we need to first Encode our struct, then call encode on the local version and feed this to our hash function invocation.

* mention `derive` in this context as well...

* much more ergonomic and idiomatic implementation; one in which the progress of each proposal is not contingent on the ones that come before it but instead focus on the unique state conditions (like ensuring that applicants don't have multple applications at once)

* relieve fears of bad indexing causing errors where a late call references a different proposal than the one previously there...

**List Variants**
* applicants can only have one application at a time

When deciding on which fields of the Proposal to hash as the unique identifer, we should add just enough information to ensure uniqueness. In our case, we will choose

### Council (Frame Example)
* higher proposal throughput than `Treasury` but this can also be improved


### OPEN QUESTION

* can we increase proposal throughput even more by using futures to provide a proxy to an eventual response (being the lookup for a map)?
    * how much would that really increase throughput
