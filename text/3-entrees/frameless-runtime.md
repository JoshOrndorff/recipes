# A Runtime Without FRAME

A piece of code can be considered a Substrate Runtime if it implements a few key traits. The most
fundamental of these traits is the [Core API](https://crates.parity.io/sp_api/trait.Core.html). In
practice, the runtime must also implement several other APIs which are described below. Typically
Substrate runtimes are written using Parity's "Framework for Runtime Aggregation from Modularized
Entities", more commonly known as
[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame). FRAME is an excellent way to
write runtimes, and all the other runtimes in the Recipes use it. However runtimes may be
sufficiently simple that FRAME is not necessary, and much can be learned by implementing a runtime
that does not use FRAME. In this recipe we will do just that.

## Imports and Type Definitions

## The `opaque` Module

## Runtime Versioning

## Declaring Storage

The storage items we're operating on are never explicitly declared. Defining storage items is
accomplished in FRAME pallets by the `decl_storage!` macro. We could use that macro here as well. We
have chosen not to make this a truly FRAMEless runtime.

Instead we will rely on .... We just define storage keys....

## Defining Extrinsics

ususaly done by decl_module

## The `GenesisConfig` Struct

When starting a new blockchain, it is often useful to initialize some starting state. To do this we
make a type that implements the
[`BuildStorage` trait](https://crates.parity.io/sp_runtime/trait.BuildStorage.html). Following
FRAME's example, we call this unit struct `GenesisConfig`. Because building genesis storage is a job
that will be performed by the outer node, we only need to compile this type when building to `std`.

```rust, ignore
/// The type that provides the genesis storage values for a new chain
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Default))]
pub struct GenesisConfig;
```

We will initialize storage by setting our main Boolean storage value to false. We must also insert
the runtime's WASM blob into the proper storage location to facilitate Substrate's forkless runtime
upgrades.

```rust, ignore
#[cfg(feature = "std")]
impl BuildStorage for GenesisConfig {
	fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
		// Declare the storage items we need
		let storage_items = vec![
			(BOOLEAN_KEY.encode(), false.encode()),
			(well_known_keys::CODE.into(), WASM_BINARY.to_vec()),
		];

		// Put them into genesis storage
		storage.top.extend(
			storage_items.into_iter()
		);

		Ok(())
	}
}
```

## The `Runtime` Struct

The primary type that we'll be exporting is a unit struct called `Runtime`. This struct will
implement all the necessary APIs to make it a runtime. FRAME-based runtimes export a struct called
`Runtime` as well, but this is not obvious when writing a runtime becuase the struct is typically
defined by the `construct_runtime!` macro. In our FRAMEless runtime, this declaration is explicit.

```rust, ignore
pub struct Runtime;
```

## Implementing the Runtime APIs

As mentioned previously, the definition of a runtime is something that implements the
[Core API](https://crates.parity.io/sp_api/trait.Core.html). In practice, a runtime must also
implement:

-   The [`BlockBuilder` API](https://crates.parity.io/sp_block_builder/trait.BlockBuilder.html)
-   The
    [TransactionPool API](https://crates.parity.io/sp_transaction_pool/runtime_api/trait.TaggedTransactionQueue.html)
-   The [Metadata API](https://crates.parity.io/sp_api/trait.Metadata.html)
-   The [`OffchainWorkerAPI`](https://crates.parity.io/sp_offchain/trait.OffchainWorkerApi.html)
-   The [`SessionKeys` API](https://crates.parity.io/sp_session/trait.SessionKeys.html)

### The Core API

### The Blockcuilder API

### Stubbing some APIs

Several of the APIs that are required by Substrate will not be used in any meaningful way by our
example runtime, and thus they are stubbed or implemented in trivial ways that make the compiler
happy. I hope that the requirement for a runtime to implement these APIs will be loosened in the
future.

As an example, let's look at the transaction pool API. This API is responsible for determining
whether a transaction is valid according to some inexpensive checks, satisfying the transaction DAG
dependencies, and prioritizing the transaction. We consider all transactions valid in any order, and
we consider all transactions of equal priority. Thus we return a trivial
[`ValidTransaction`](https://crates.parity.io/sp_runtime/transaction_validity/struct.ValidTransaction.html).

```rust, ignore
impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
	fn validate_transaction(
		_source: TransactionSource,
		_tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
		// Any transaction of the correct type is valid
		Ok(ValidTransaction{
			priority: 1u64,
			requires: Vec::new(),
			provides: Vec::new(),
			longevity: TransactionLongevity::max_value(),
			propagate: true,
		})
	}
}
```

We will not quote the rest of the trivial code here, but you can see that they do like transaction
pool API did and return values like `None` and `Vec::new()`.
