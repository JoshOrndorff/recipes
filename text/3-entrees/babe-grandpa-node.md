# Babe and Grandpa Node
*[`nodes/babe-grandpa-node`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/babe-grandpa-node)*

The `babe-grandpa-node` uses the [Babe](https://substrate.dev/rustdocs/master/sc_consensus_babe/) Proof of Authority consensus engine to determine who may author blocks, and the [Grandpa](https://substrate.dev/rustdocs/master/sc_finality_grandpa/index.html) finality gadget to provide [deterministic finality](https://www.substrate.io/kb/advanced/consensus#finality) to past blocks. Unlike other consensus engines demonstrated in the recipes, both of these engines are proof of authority algorithms which means that they need to know a finite list of authorities who are participating, are designed to get the authority set information from the Substrate runtime through specific runtime APIs.

In this recipe we will learn about:
* Proof of Authority in Substrate generally, and Babe and Grandpa in particular
* The [GrandpaAPI](https://substrate.dev/rustdocs/master/sp_finality_grandpa/trait.GrandpaApi.html) runtime API
* The [BabeApi](https://substrate.dev/rustdocs/master/sc_consensus_babe/trait.BabeApi.html) runtime API
* Passing information from consensus engines to the runtime with [PreRuntime Digests](https://substrate.dev/rustdocs/master/sp_runtime/enum.DigestItem.html)

## The Block Import Pipeline

Substrate's block import pipeline is structured like an onion in the sense that it is layered. A Substrate node can compose pieces of block import logic by wrapping block imports in other block imports. In this node wee need to ensure that blocks are valid according to both Babe _and_ grandpa. So we will construct a block import for each of them and wrap one with the other. The end of the block import pipeline is always the client.

We begin by creating the block import for grandpa. In addition to the block import itself, we get back a `grandpa_link`. This link is a channel over which the block import can communicate with the background task that actually casts grandpa votes. The [details of the grandpa protocol](https://research.web3.foundation/en/latest/polkadot/GRANDPA.html) are beyond the scope of this recipe.

```rust, ignore
let (grandpa_block_import, grandpa_link) =
	sc_finality_grandpa::block_import(
		client.clone(), &(client.clone() as std::sync::Arc<_>), select_chain
	)?;
```

This same block import will be used as a justification import, so we clone it right after constructing it.
```rust, ignore
let justification_import = grandpa_block_import.clone();
```

With the grandpa block import created, we can now create the Babe block import. The babe block import is the outer-most layer of the block import onion and it wraps the grandpa block import.

```rust, ignore
let (babe_block_import, babe_link) = sc_consensus_babe::block_import(
	sc_consensus_babe::Config::get_or_compute(&*client)?,
	grandpa_block_import,
	client.clone(),
)?;
```

Again we are given back a babe link which will be used to communicate with the import queue and background worker.

## The Import Queue

With the block imports setup, we can proceed to creating the import queue. We make it using Babe's `import_queue` helper function. Notice that it requires the babe link, and the entire block import pipeline which we refer to as `babe_block_import` because Babe is the outermost layer.

```rust, ignore
let import_queue = sc_consensus_babe::import_queue(
	babe_link.clone(),
	babe_block_import.clone(),
	Some(Box::new(justification_import)),
	None,
	client,
	inherent_data_providers.clone(),
)?;
```

## The Finality Proof Provider

Occasionally in the operation of a blockchain, other nodes will contact our node asking for proof that a particular block is finalized. To respond to these requests, we include a finality proof provider.

```rust, ignore
.with_finality_proof_provider(|client, backend| {
	let provider = client as Arc<dyn StorageAndProofProvider<_, _>>;
	Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, provider)) as _)
})?
```

## Spawning the Babe Authorship Task

Any node that is acting as an authority and participating in Babe consensus, must run an `async` authorship task. We begin by creating an instance of [`BabeParams`](https://substrate.dev/rustdocs/master/sc_consensus_babe/struct.BabeParams.html).

```rust, ignore
let babe_config = sc_consensus_babe::BabeParams {
	keystore: service.keystore(),
	client,
	select_chain,
	env: proposer,
	block_import,
	sync_oracle: service.network(),
	inherent_data_providers: inherent_data_providers.clone(),
	force_authoring,
	babe_link,
	can_author_with,
};
```

With the parameters established, we can now create and spawn the authorship future.

```rust, ignore
let babe = sc_consensus_babe::start_babe(babe_config)?;
service.spawn_essential_task("babe", babe);
```

## Spawning the Grandpa Task

Just as we needed an `async` worker to author block with Babe, we need an `async` worker to listen to and cast grandpa votes. Again, we begin by creating an instance of [`GrandpaParams`](https://substrate.dev/rustdocs/master/sc_finality_grandpa/struct.GrandpaParams.html)

```rust, ignore
let grandpa_config = sc_finality_grandpa::GrandpaParams {
	config: grandpa_config,
	link: grandpa_link,
	network: service.network(),
	inherent_data_providers: inherent_data_providers.clone(),
	telemetry_on_connect: Some(service.telemetry_on_connect_stream()),
	voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
	prometheus_registry: service.prometheus_registry(),
};
```

With the parameters established, we can now create and spawn the authorship future.

```rust, ignore
service.spawn_essential_task(
	"grandpa-voter",
	sc_finality_grandpa::run_grandpa_voter(grandpa_config)?
);
```

### Disabled Grandpa

Proof of Authority networks generally contain many full nodes that are not authorities. When Grandpa is present in the network, we still need to tell the node how to interpret grandpa-related messages it may receive (just ignore them) and ensure that the correct inherents are still included in blocks in the case that the node _is_ an authority in Babe but not Grandpa.

```rust, ignore
sc_finality_grandpa::setup_disabled_grandpa(
	service.client(),
	&inherent_data_providers,
	service.network(),
)?;
```

## Constraints on the Runtime

### Runtime APIs

Both Babe and Grandpa rely on getting their authority sets from the runtime via the [BabeAPI](https://substrate.dev/rustdocs/master/sc_consensus_babe/trait.BabeApi.html) and the [GrandpaAPI](https://substrate.dev/rustdocs/master/sp_finality_grandpa/trait.GrandpaApi.html). So trying to build this node with a runtime that does not provide these APIs will fail to compile.

### Pre Runtime Digests

The Babe consensus algorithm also needs to provide information to the runtime. Because BABE is a slot-based consensus engine, it must inform the runtime which slot each block was intended for. To do this, it uses a technique known as a pre-runtime digest. It has two kinds, [`RawPrimaryPredigest`](https://substrate.dev/rustdocs/master/sp_consensus_babe/digests/struct.RawPrimaryPreDigest.html) and [`SecondaryPreDigest`](https://substrate.dev/rustdocs/master/sp_consensus_babe/digests/struct.SecondaryPreDigest.html). The Babe authorship task automatically inserts these digest items in each block it authors.

Because the runtime needs to interpret these pre-runtime digests, they are not optional. That means runtimes that expect the pre-digests _require_ the pre-digests, and cannot be used in nodes that don't provide them. This limits the reusability of runtimes. Unlike the rest of the recipes where runtimes can be freely swapped between nodes, the `babe-grandpa-runtime` can _only_ be used in a node that provides the proper pre-runtime digests.
