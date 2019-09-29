# Storage

The [`decl_storage`](https://wiki.parity.io/decl_storage) documentation specifies how to define define type-safe, persistent data that needs to be stored on-chain.

For crypto*currencies*, storage might consist of a mapping between account keys and corresponding balances. 

More generally, blockchains provide an interface to store and interact with data in a verifiable and globally irreversible way. In this context, data is stored in a series of snapshots, each of which may be accessed at a later point in time, but, once created, snapshots are considered irreversible.

Arbitrary data may be stored, as long as its data type is serializable in Substrate i.e. implements [`Encode`](https://docs.rs/parity-scale-codec/1.0.6/parity_scale_codec/#encode) and [`Decode`](https://docs.rs/parity-scale-codec/1.0.6/parity_scale_codec/#decode) traits.

The previous *[single-value storage](../basics/value.md)* showed how a single value can be stored in runtime storage. In this section, we cover
- [caching values rather than calling to storage multiple times](./cache.md)
- [storing sets, checking membership, and iteration](./iterate.md)
- [ordered lists with basic maps and linked maps](./enumerated.md)
- [efficient subgroup removal by key prefix with double maps](./storage/double.md)
- [configurable module constants](./constants.md)

*in-progress*
* - [cheap inclusion proofs with child tries](./childtries.md)