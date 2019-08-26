# Storage

Use the [`decl_storage`](https://wiki.parity.io/decl_storage) macro to define type-safe, persistent data that needs to be stored on-chain.

For crypto*currencies*, storage might consist of a mapping between account keys and corresponding balances. 

More generally, blockchains provide an interface to store and interact with data in a verifiable and globally irreversible way. In this context, data is stored in a series of snapshots, each of which may be accessed at a later point in time, but, once created, snapshots are considered irreversible.

Arbitrary data may be stored, as long as its data type is serializable in Substrate i.e. implements [`Encode`](https://docs.rs/parity-codec/3.1.0/parity_codec/trait.Encode.html) and [`Decode`](https://docs.rs/parity-codec/3.1.0/parity_codec/trait.Decode.html#foreign-impls) traits.

## Recipes
- [Single Value Storage](./value.md)
- [Configurable Module Constants](./constants.md)
- [Simple Token Transfer (Maps)](./mapping.md)
- [Lists as Maps](./list.md)
- [Nested Structs](./structs.md)
- [Social Network (Higher Order Arrays)](./arrays.md)

### More Resources

* [`decl_storage` wiki docs](https://wiki.parity.io/decl_storage)