# A Runtime Without FRAME

I piece of code can be considered a Substrate Runtime if it implements a few key traits. The most fundamental of these traits is the [Core API](https://substrate.dev/rustdocs/master/sp_api/trait.Core.html). In practice, the runtime must also implement several other APIs which are described below. Typically Substrate runtimes are written using Parity's Framework for Runtime Aggregation from Modularized Entities, more commonly known as [FRAME](https://www.substrate.io/kb/runtime/frame). FRAME is an excellent way to write runtimes, and all the other runtimes in the Recipes use it. However runtimes may be sufficiently simple that FRAME is not necessary, and much can be learned by implementing a runtime that does not use FRAME. In this recipe we will do jsut that.

## Imports and Type Definitions

## The `opaque` Module

## Runtime Versioning

## The `Runtime` Struct

## The `GenesisConfig` Struct

## Defining Extrinsics

## Implementing the Runtime APIs
As mentioned previously, the definition of a runtime is something that implements the [Core API](https://substrate.dev/rustdocs/master/sp_api/trait.Core.html). In practice, a runtime must also implement:
* The [`BlockBuilder` API](https://substrate.dev/rustdocs/master/sp_block_builder/trait.BlockBuilder.html)
* The [TransactionPool API](https://substrate.dev/rustdocs/master/sp_transaction_pool/runtime_api/trait.TaggedTransactionQueue.html)
* The [Metadata API](https://substrate.dev/rustdocs/master/sp_api/trait.Metadata.html)
* The [`OffchainWorkerAPI`](https://substrate.dev/rustdocs/master/sp_offchain/trait.OffchainWorkerApi.html)
* The [`SessionKeys` API](https://substrate.dev/rustdocs/master/sp_session/trait.SessionKeys.html)

### The Core API

### The Blockcuilder API

### Stubbing some APIs

Several of the APIs that are required by Substrate will not be used in any meaningful way by our example runtime, and thus they are stubbed or implemented in trivial ways that make the compiler happy. I hope that the requirement for a runtime to implement these APIs will be loosened in the future.

As an example, let's look at the transaction pool API. This API is responsible for determining whether a transaction is valid according to some inexpensive checks, satisfying the transaction DAG dependencies, and prioritizing the transaction. We consider all transactions valid in any order, and we consider all transactions of equal priority. Thus we return a trivial [`ValidTransaction`](https://substrate.dev/rustdocs/master/sp_runtime/transaction_validity/struct.ValidTransaction.html).

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

We will not quote the rest of the trivial code here, but you can see that they do like transaction pool API did and return values like `None` and `Vec::new()`.
