# Storage API

We've already encountered the
[`decl_storage!` macro](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/macro.decl_storage.html) in the
appetizer on [storage items](../../2-appetizers/2-storage-values.md). There is a rich storage API in
Substrate which we will explore in this section.

For crypto*currencies*, storage might consist of a mapping between account keys and corresponding
balances.

More generally, blockchains provide an interface to store and interact with data in a verifiable and
globally irreversible way. In this context, data is stored in a series of snapshots, each of which
may be accessed at a later point in time, but, once created, snapshots are considered irreversible.

Arbitrary data may be stored, as long as its data type is serializable in Substrate i.e. implements
[`Encode`](https://docs.rs/parity-scale-codec/1.3.0/parity_scale_codec/#encode) and
[`Decode`](https://docs.rs/parity-scale-codec/1.3.0/parity_scale_codec/#decode) traits.

The previous _[single-value storage recipe](../../2-appetizers/2-storage-values.md)_ showed how a
single value can be stored in runtime storage. In this section, we cover

-   [caching values rather than calling to storage multiple times](./cache.md)
-   [Vec sets](./vec-set.md)
-   [Map sets](./map-set.md)
-   [efficient subgroup removal by key prefix with double maps](./double.md)
-   [storing custom structs](./structs.md)
-   [transient storage adapters by example of a ringbuffer queue](./ringbuffer.md)
