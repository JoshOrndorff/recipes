# Hybrid Consensus

`nodes/hybrid-consensus`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fnodes%2Fhybrid-consensus%2Fsrc%2Fservice.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/nodes/hybrid-consensus/src/service.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

This recipe demonstrates a Substrate-based node that employs hybrid consensus. Specifically, it uses
[Sha3 Proof of Work](./sha3-pow-consensus.md) to dictate block authoring, and the
[Grandpa](https://substrate.dev/rustdocs/v3.0.0/sc_finality_grandpa/index.html) finality gadget to provide
[deterministic finality](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#finality). The minimal proof
of work consensus lives entirely outside of the runtime while the grandpa finality obtains its
authorities from the runtime via the
[GrandpaAPI](https://substrate.dev/rustdocs/v3.0.0/sp_finality_grandpa/trait.GrandpaApi.html). Understanding this
recipe requires familiarity with Substrate's
[block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import).

## The Block Import Pipeline

Substrate's block import pipeline is structured like an onion in the sense that it is layered. A
Substrate node can compose pieces of block import logic by wrapping block imports in other block
imports. In this node we need to ensure that blocks are valid according to both Pow _and_ grandpa.
So we will construct a block import for each of them and wrap one with the other. The end of the
block import pipeline is always the client, which contains the underlying database of imported
blocks. Learn more about the [block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import) in the Substrate knowledgebase.

We begin by creating the block import for grandpa. In addition to the block import itself, we get
back a `grandpa_link`. This link is a channel over which the block import can communicate with the
background task that actually casts grandpa votes. The
[details of the grandpa protocol](https://research.web3.foundation/en/latest/polkadot/finality.html)
are beyond the scope of this recipe.

```rust, ignore
let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
	client.clone(),
	&(client.clone() as std::sync::Arc<_>),
	select_chain.clone(),
)?;
```

With the grandpa block import created, we can now create the PoW block import. The Pow block import
is the outer-most layer of the block import onion and it wraps the grandpa block import.

```rust, ignore
let pow_block_import = sc_consensus_pow::PowBlockImport::new(
	grandpa_block_import,
	client.clone(),
	sha3pow::MinimalSha3Algorithm,
	0, // check inherents starting at block 0
	select_chain.clone(),
	inherent_data_providers.clone(),
	can_author_with,
);
```

## The Import Queue

With the block imports setup, we can proceed to creating the import queue. We make it using PoW's
[`import_queue` helper function](https://substrate.dev/rustdocs/v3.0.0/sc_consensus_pow/fn.import_queue.html). Notice that it requires the entire block import pipeline which we
refer to as `pow_block_import` because PoW is the outermost layer.

```rust, ignore
let import_queue = sc_consensus_pow::import_queue(
	Box::new(pow_block_import.clone()),
	None,
	sha3pow::MinimalSha3Algorithm,
	inherent_data_providers.clone(),
	&task_manager.spawn_handle(),
	config.prometheus_registry(),
)?;
```

## Spawning the PoW Authorship Task

Any node that is acting as an authority, typically called "miners" in the PoW context, must run a
mining worker that is spawned by the task manager.

```rust, ignore
let (_worker, worker_task) = sc_consensus_pow::start_mining_worker(
	Box::new(pow_block_import),
	client,
	select_chain,
	MinimalSha3Algorithm,
	proposer,
	network.clone(),
	None,
	inherent_data_providers,
	// time to wait for a new block before starting to mine a new one
	Duration::from_secs(10),
	// how long to take to actually build the block (i.e. executing extrinsics)
	Duration::from_secs(10),
	can_author_with,
);

task_manager
	.spawn_essential_handle()
	.spawn_blocking("pow", worker_task);
```

## Spawning the Grandpa Task

Grandpa is _not_ CPU intensive, so we will use a standard `async` worker to listen to and cast
grandpa votes. We begin by creating a grandpa
[`Config`](https://substrate.dev/rustdocs/v3.0.0/sc_finality_grandpa/struct.Config.html).

```rust, ignore
let grandpa_config = sc_finality_grandpa::Config {
	gossip_duration: Duration::from_millis(333),
	justification_period: 512,
	name: None,
	observer_enabled: false,
	keystore: Some(keystore_container.sync_keystore()),
	is_authority,
};
```

We can then use this config to create an instance of
[`GrandpaParams`](https://substrate.dev/rustdocs/v3.0.0/sc_finality_grandpa/struct.GrandpaParams.html).

```rust, ignore
let grandpa_config = sc_finality_grandpa::GrandpaParams {
	config: grandpa_config,
	link: grandpa_link,
	network,
	telemetry_on_connect: telemetry_connection_notifier.map(|x| x.on_connect_stream()),
	voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
	prometheus_registry,
	shared_voter_state: sc_finality_grandpa::SharedVoterState::empty(),
};
```

With the parameters established, we can now create and spawn the authorship future.

```rust, ignore
task_manager.spawn_essential_handle().spawn_blocking(
	"grandpa-voter",
	sc_finality_grandpa::run_grandpa_voter(grandpa_config)?
);
```

## Constraints on the Runtime

### Runtime APIs

Grandpa relies on getting its authority sets from the runtime via the
[GrandpaAPI](https://substrate.dev/rustdocs/v3.0.0/sp_finality_grandpa/trait.GrandpaApi.html). So trying to build
this node with a runtime that does not provide this API will fail to compile. For that reason, we
have included the dedicated `minimal-grandpa-runtime`.

The opposite is not true, however. A node that does _not_ require grandpa may use the
`minimal-grandpa-runtime` successfully. The unused `GrandpaAPI` will remain as a harmless vestige in
the runtime.
