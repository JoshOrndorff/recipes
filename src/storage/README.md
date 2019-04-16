# Storage

Use the [`decl_storage`](https://wiki.parity.io/decl_storage) macro to define type-safe, persistent data that needs to be stored on-chain.

For cryptocurrencies such storage might consist of a mapping between account keys and corresponding balances. 

More generally, blockchains provide an interface to store and interact with data in a verifiable and globally irreversible way. In this context, data is stored in a series of snapshots, each of them may be accessed at a later point in time, but, once created, snapshots are considered irreversible.

Generally speaking, you may store arbitrary data, as long as its data type is serializable in substrate terms, i.e. implements [`Encode`](https://docs.rs/parity-codec/3.1.0/parity_codec/trait.Encode.html) and [`Decode`](https://docs.rs/parity-codec/3.1.0/parity_codec/trait.Decode.html#foreign-impls) traits.

## Recipes

*Common Patterns*
* [Single Value](./value.md)
* [Mapping](./mapping.md)
* [List](./list.md)
* [Generic Structs](./structs.md)
* [Higher Order Arrays](./arrays.md)

*Unidiomatic Patterns*
* [`String`](./string.md)

<!-- *Off-Chain Storage Patterms*
* `TODO`: caching (even for compilation -- sscache), database interaction, etc. -->

## More Docs

* [Wiki Documentation for the SRML Source Code](https://wiki.parity.io/decl_storage)