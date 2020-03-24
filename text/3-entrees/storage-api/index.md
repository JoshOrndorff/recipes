# Storage API

We've already encountered the [`decl_storage!` macro](https://substrate.dev/rustdocs/master/frame_support/macro.decl_storage.html) in the appetizer on [storage items](../../2-appetizers/2-storage-values.md). There is a rich storage API in Substrate which we will explore in this section.

For crypto*currencies*, storage might consist of a mapping between account keys and corresponding balances.

More generally, blockchains provide an interface to store and interact with data in a verifiable and globally irreversible way. In this context, data is stored in a series of snapshots, each of which may be accessed at a later point in time, but, once created, snapshots are considered irreversible.

Arbitrary data may be stored, as long as its data type is serializable in Substrate i.e. implements [`Encode`](https://docs.rs/parity-scale-codec/1.0.6/parity_scale_codec/#encode) and [`Decode`](https://docs.rs/parity-scale-codec/1.0.6/parity_scale_codec/#decode) traits.

The previous *[single-value storage recipe](../../2-appetizers/2-storage-values.md)* showed how a single value can be stored in runtime storage. In this section, we cover
- [caching values rather than calling to storage multiple times](./cache.md)
- [storing sets, checking membership, and iteration](./iterate.md)
- [ordered lists with basic maps and linked maps](./enumerated.md)
- [efficient subgroup removal by key prefix with double maps](./double.md)
- [storing custom structs](./structs.md)

*in-progress*
- [cheap inclusion proofs with child tries](./childtries.md)
- [transient storage adapters by example of a ringbuffer queue](./ringbuffer.md)
