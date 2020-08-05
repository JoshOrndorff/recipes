# Hybrid Consensus

_[`nodes/hybrid-consensus`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/hybrid-consensus)_

This recipe demonstrates a Substrate-based node that employs hybrid consensus. Specifically, it uses
[Sha3 Proof of Work](./sha3-pow-consensus.md) to dictate block authoring, and the
[Grandpa](https://substrate.dev/rustdocs/v2.0.0-rc5/sc_finality_grandpa/index.html) finality gadget to provide
[deterministic finality](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#finality). The minimal proof
of work consensus lives entirely outside of the runtime while the grandpa finality obtains its
authorities from the runtime via the
[GrandpaAPI](https://substrate.dev/rustdocs/v2.0.0-rc5/sp_finality_grandpa/trait.GrandpaApi.html). Understanding this
recipe requires familiarity with Substrate's
[block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import).

## The Block Import Pipeline

Substrate's block import pipeline is structured like an onion in the sense that it is layered. A
Substrate node can compose pieces of block import logic by wrapping block imports in other block
imports. In this node we need to ensure that blocks are valid according to both Pow _and_ grandpa.
So we will construct a block import for each of them and wrap one with the other. The end of the
block import pipeline is always the client, which contains the underlying datbase of imported
blocks.

We begin by creating the block import for grandpa. In addition to the block import itself, we get
back a `grandpa_link`. This link is a channel over which the block import can communicate with the
background task that actually casts grandpa votes. The
[details of the grandpa protocol](https://research.web3.foundation/en/latest/polkadot/GRANDPA.html)
are beyond the scope of this recipe.

```rust, ignore
let (grandpa_block_import, grandpa_link) =
	sc_finality_grandpa::block_import(
		client.clone(), &(client.clone() as std::sync::Arc<_>), select_chain
	)?;
```

This same block import will be used as a justification import, so we clone it right after
constructing it.

```rust, ignore
let justification_import = grandpa_block_import.clone();
```

With the grandpa block import created, we can now create the PoW block import. The Pow block import
is the outer-most layer of the block import onion and it wraps the grandpa block import.

```rust, ignore
let pow_block_import = sc_consensus_pow::PowBlockImport::new(
	grandpa_block_import,
	client.clone(),
	sha3pow::MinimalSha3Algorithm,
	0, // check inherents starting at block 0
	Some(select_chain),
	inherent_data_providers.clone(),
);
```

## The Import Queue

With the block imports setup, we can proceed to creating the import queue. We make it using PoW's
`import_queue` helper function. Notice that it requires the entire block import pipeline which we
refer to as `pow_block_import` because PoW is the outermost layer.

```rust, ignore
let import_queue = sc_consensus_pow::import_queue(
	Box::new(pow_block_import),
	Some(Box::new(justification_import)),
	None,
	sha3pow::MinimalSha3Algorithm,
	inherent_data_providers.clone(),
	spawn_task_handle,
)?;
```

## The Finality Proof Provider

Occasionally in the operation of a blockchain, other nodes will contact our node asking for proof
that a particular block is finalized. To respond to these requests, we include a finality proof
provider.

```rust, ignore
.with_finality_proof_provider(|client, backend| {
	let provider = client as Arc<dyn StorageAndProofProvider<_, _>>;
	Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, provider)) as _)
})?
```

## Spawning the PoW Authorship Task

Any node that is acting as an authority, typically called "miners" in the PoW context, must run a
mining task in another thread.

```rust, ignore
sc_consensus_pow::start_mine(
	Box::new(block_import),
	client,
	MinimalSha3Algorithm,
	proposer,
	None, // TODO Do I need some grandpa preruntime digests?
	500, // Rounds
	service.network(),
	std::time::Duration::new(2, 0),
	Some(select_chain),
	inherent_data_providers.clone(),
	can_author_with,
);
```

The use of a separate thread for block authorship is unlike other Substrate-based authorship tasks
which are typically run as `async` futures. Because mining is a CPU intensive process, it is
necessary to provide a separate thread or else the mining task would run continually and other tasks
such as transaction processing, gossiping, and peer discovery would be starved for CPU.

## Spawning the Grandpa Task

Grandpa is _not_ CPU intensive, so we will use a standard `async` worker to listen to and cast
grandpa votes. We begin by creating a grandpa
[`Config`](https://substrate.dev/rustdocs/v2.0.0-rc5/sc_finality_grandpa/struct.Config.html).

```rust, ignore
let grandpa_config = sc_finality_grandpa::Config {
	gossip_duration: Duration::from_millis(333),
	justification_period: 512,
	name: Some(name),
	observer_enabled: false,
	keystore,
	is_authority: role.is_network_authority(),
};
```

We can then use this config to create an instance of
[`GrandpaParams`](https://substrate.dev/rustdocs/v2.0.0-rc5/sc_finality_grandpa/struct.GrandpaParams.html).

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

Proof of Authority networks generally contain many full nodes that are not authorities. When Grandpa
is present in the network, we still need to tell the node how to interpret grandpa-related messages
it may receive (just ignore them).

```rust, ignore
sc_finality_grandpa::setup_disabled_grandpa(
	service.client(),
	&inherent_data_providers,
	service.network(),
)?;
```

## Constraints on the Runtime

### Runtime APIs

Grandpa relies on getting its authority sets from the runtime via the
[GrandpaAPI](https://substrate.dev/rustdocs/v2.0.0-rc5/sp_finality_grandpa/trait.GrandpaApi.html). So trying to build
this node with a runtime that does not provide this API will fail to compile. For that reason, we
have included the dedicated `minimal-grandpa-runtime`.

The opposite is not true, however. A node that does _not_ require grandpa may use the
`minimal-grandpa-runtime` successfully. The unused `GrandpaAPI` will remain as a harmless vestige in
the runtime.
