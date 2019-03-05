# Storage

Use the [`decl_storage`](https://wiki.parity.io/decl_storage) macro to define type-safe, persistent data that needs to be stored on-chain.

For cryptocurrencies such storage might consist of a mapping between account keys and corresponding balances. 

More generally, blockchains provide an interface to store and interact with data in a verifiable and globally irreversible way. In this context, data is stored in a series of snapshots, each of them may be accessed at a later point in time, but, once created, snapshots are considered irreversible.

Generally speaking, you may store arbitrary data, as long as its data type is serializable in substrate terms, i.e. implements [`Encode`]() and [`Decode`]() traits.

## Recipes

*Common Patterns*
* [Single Value](./value.md)
* [Mapping](./mapping.md)
* [Generic Structs](./structs.md)
* [Higher Order Arrays](./arrays.md)

*Unidiomatic Patterns*
* [`List`](./list.md)
* [`String`](./string.md)

*Off-Chain Storage Patterms*
* `TODO`: caching, database interaction, etc.

## Examples in the <a href="">SRML Source Code</a>

* [SRML EXAMPLES HERE](https://wiki.parity.io/decl_storage)

### TODO

* clean up existing examples and format in a coherent way
* off-chain patterns for storage and database interaction
* include page on the srml examples of `decl_storage` (reference relevant patterns)