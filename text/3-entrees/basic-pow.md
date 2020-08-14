# Basic Proof of Work

`nodes/basic-pow`
[
	![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)
](https://playground-staging.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fnodes%2Fbasic-pow%2Fsrc%2Flib.rs)
[
	![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)
](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/basic-pow/src/lib.rs)

The `basic-pow` node demonstrates how to wire up a custom consensus engine into the Substrate
Service. It uses a minimal proof of work consensus engine to reach agreement over the blockchain. It
will teach us many useful aspects of dealing with consensus and prepare us to understand more
advanced consensus engines in the future. In particular we will learn about:

-   Substrate's
    [`BlockImport` trait](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_consensus/block_import/trait.BlockImport.html)
-   Substrate's [import pipeline](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_consensus/import_queue/index.html)
-   Structure of a typical [Substrate Service](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_service/index.html)
-   Configuration of
    [`InherentDataProvider`](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_authorship/struct.InherentDataProvider.html)s

## The Structure of a Node

You may remember from the [hello-substrate recipe](../2-appetizers/1-hello-substrate.md) that a
Substrate node has two parts. An outer part that is responsible for gossiping transactions and
blocks, handling [rpc requests](./custom-rpc.md), and reaching consensus. And a runtime that is
responsible for the business logic of the chain. This architecture diagram illustrates the
distinction.

![Substrate Architecture Diagram](../img/substrate-architecture.png)

In principle, the consensus engine (part of the outer node) is agnostic to the runtime that is used
with it. But in practice, most consensus engines will require the runtime to provide certain
[runtime APIs](./runtime-api.md) that affect the engine. For example, Aura and Babe query the
runtime for the set of validators. A more real-world PoW consensus would query the runtime for the
block difficulty. Additionally, some runtimes rely on the consensus engine to provide
[pre-runtime digests](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_runtime/generic/enum.DigestItem.html#variant.PreRuntime).
For example, runtimes that include the Babe pallet expect a pre-runtime digest containing
information about the current babe slot.

In this recipe we will avoid those practical complexities by using the
[Minimal Sha3 Proof of Work](./sha3-pow-consensus.md) consensus engine, and a dedicated
`pow-runtime` which are truly isolated from each other. The contents of the runtime should be
familiar, and will not be discussed here.

## The Service Builder

The [Substrate Service](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_service/trait.AbstractService.html) is the main
coordinator of the various parts of a Substrate node, including consensus. The service is large and
takes many parameters, so it is built with a
[ServiceBuilder](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_service/struct.ServiceBuilder.html) following
[Rust's builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html). This code
is demonstrated in the nodes `src/service.rs` file.

The particular builder method that is relevant here is
[`with_import_queue`](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_service/struct.ServiceBuilder.html#method.with_import_queue).
Here we construct an instance of the
[`PowBlockImport` struct](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_consensus_pow/struct.PowBlockImport.html),
providing it with references to our client, our `MinimalSha3Algorithm`, and some other necessary
data.

```rust, ignore
builder
	.with_import_queue(|_config, client, select_chain, _transaction_pool| {

		let pow_block_import = sc_consensus_pow::PowBlockImport::new(
			client.clone(),
			client.clone(),
			sha3pow::Sha3Algorithm,
			0, // check inherents starting at block 0
			select_chain,
			inherent_data_providers.clone(),
		);

		let import_queue = sc_consensus_pow::import_queue(
			Box::new(pow_block_import.clone()),
			sha3pow::Sha3Algorithm,
			inherent_data_providers.clone(),
		)?;

		import_setup = Some(pow_block_import);

		Ok(import_queue)
	})?;
```

Once the `PowBlockImport` is constructed, we can use it to create an actual import queue that the
service will use for importing blocks into the client.

### The Block Import Pipeline

You may have noticed that when we created the `PowBlockImport` we gave it two separate references to
the client. The second reference will always be to a client. But the first is interesting. The
[rustdocs tell us](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_consensus_pow/struct.PowBlockImport.html#method.new)
that the first parameter is `inner: BlockImport<B, Transaction = TransactionFor<C, B>>`. Why would a
block import have a reference to another block import? Because the "block import pipeline" is
constructed in an onion-like fashion, where one layer of block import wraps the next. Learn more
about this pattern in the knowledgebase article on the
[block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import).

### Inherent Data Providers

Both the BlockImport and the `import_queue` are given an instance called `inherent_data_providers`.
This object is created in a helper function defined at the beginning of `service.rs`

```rust, ignore
pub fn build_inherent_data_providers() -> Result<InherentDataProviders, ServiceError> {
	let providers = InherentDataProviders::new();

	providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	Ok(providers)
}
```

Anything that implements the
[`ProvideInherentData` trait](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_inherents/trait.ProvideInherentData.html)
may be used here. The block authoring logic must supply all inherents that the runtime expects. In
the case of this basic-pow chain, that is just the
[`TimestampInherentData`](https://substrate.dev/rustdocs/v2.0.0-rc4/sp_timestamp/trait.TimestampInherentData.html)
expected by the [timestamp pallet](https://substrate.dev/rustdocs/v2.0.0-rc4/pallet_timestamp/index.html). In order
to register other inherents, you would call `register_provider` multiple times, and map errors
accordingly.

## Mining

We've already implemented a mining algorithm as part of our
[`MinimalSha3Algorithm`](./sha3-pow-consensus.md), but we haven't yet told our service to actually
mine with that algorithm. This is our last task in the `new_full` function.

```rust, ignore
if participates_in_consensus {
	let proposer = sc_basic_authorship::ProposerFactory::new(
		service.client(),
		service.transaction_pool()
	);

	// The number of rounds of mining to try in a single call
	let rounds = 500;

	let client = service.client();
	let select_chain = service.select_chain()
		.ok_or(ServiceError::SelectChainRequired)?;

	let can_author_with =
		sp_consensus::CanAuthorWithNativeVersion::new(service.client().executor().clone());

	sc_consensus_pow::start_mine(
		Box::new(block_import),
		client,
		MinimalSha3Algorithm,
		proposer,
		None, // No preruntime digests
		rounds,
		service.network(),
		std::time::Duration::new(2, 0),
		Some(select_chain),
		inherent_data_providers.clone(),
		can_author_with,
	);
}
```

We begin by testing whether this node participates in consensus, which is to say we check whether
the user wants the node to act as a miner. If this node is to be a miner, we gather references to
various parts of the node that the
[`start_mine` function](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_consensus_pow/fn.start_mine.html) requires, and
define that we will attempt 500 rounds of mining for each block before pausing. Finally we call
`start_mine`.

## The Light Client

The last thing in the `service.rs` file is constructing the
[light client](https://www.parity.io/what-is-a-light-client/)'s service. This code is quite similar
to the construction of the full service.

Instead of using the `with_import_queue` function we used previously, we use the
`with_import_queue_and_fprb` function. FPRB stand for
[`FinalityProofRequestBuilder`](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_network/config/trait.FinalityProofRequestBuilder.html).
In chains with deterministic finality, light clients must request proofs of finality from full
nodes. But in our chain, we do not have deterministic finality, so we can use the
[`DummyFinalityProofRequestBuilder`](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_network/config/struct.DummyFinalityProofRequestBuilder.html)
which does nothing except satisfying Rust's type checker.

Once the dummy request builder is configured, the `BlockImport` and import queue are configured
exactly as they were in the full node.

## Note of Finality

If we run the `basic-pow` node now, we see in console logs, that the finalized block always remains
at 0.

```
...
2020-03-22 12:50:09 Starting consensus session on top of parent 0x85811577d1033e918b425380222fd8c5aef980f81fa843d064d80fe027c79f5a
2020-03-22 12:50:09 Imported #189 (0x8581…9f5a)
2020-03-22 12:50:09 Prepared block for proposing at 190 [hash: 0xdd83ba96582acbed59aacd5304a9258962d1d4c2180acb8b77f725bd81461c4f; parent_hash: 0x8581…9f5a; extrinsics (1): [0x77a5…f7ad]]
2020-03-22 12:50:10 Idle (1 peers), best: #189 (0x8581…9f5a), finalized #0 (0xff0d…5cb9), ⬇ 0.2kiB/s ⬆ 0.4kiB/s
2020-03-22 12:50:15 Idle (1 peers), best: #189 (0x8581…9f5a), finalized #0 (0xff0d…5cb9), ⬇ 0 ⬆ 0
```

This is expected because Proof of Work is a consensus mechanism with probabilistic finality. This
means a block is never truly finalized and can always be reverted. The further behind the blockchain
head a block is, the less likely it is going to be reverted.
