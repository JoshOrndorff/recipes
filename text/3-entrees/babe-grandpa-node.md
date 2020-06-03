# BABE and GRANDPA Node

_[`nodes/babe-grandpa-node`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/babe-grandpa-node)_

The `babe-grandpa-node` uses the [BABE](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/index.html) Proof
of Authority consensus engine to determine who may author blocks, and the
[GRANDPA](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_finality_grandpa/index.html) finality gadget to provide
[deterministic finality](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#finality) to past blocks.
This is the same design used in Polkadot. Understanding this recipe requires familiarity with
Substrate's [block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import).

In this recipe we will learn about:

-   The [GrandpaAPI](https://substrate.dev/rustdocs/v2.0.0-rc2/sp_finality_grandpa/trait.GrandpaApi.html) runtime API
-   The [BabeApi](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/trait.BabeApi.html) runtime API
-   The [block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import)

## The Block Import Pipeline

The babe-grandpa node's block import pipeline will have three layers. The inner-most layer is the
Substrate [`Client`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_service/client/struct.Client.html), as always. We
will wrap the client with a
[`GrandpaBlockImport`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_finality_grandpa/struct.GrandpaBlockImport.html),
and wrap that with a
[`BabeBlockImport`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/struct.BabeBlockImport.html).

We begin by creating the block import for GRANDPA. In addition to the block import itself, we get
back a `grandpa_link`. This link is a channel over which the block import can communicate with the
background task that actually casts GRANDPA votes. The
[details of the GRANDPA protocol](https://research.web3.foundation/en/latest/polkadot/GRANDPA.html)
are beyond the scope of this recipe.

```rust, ignore
let (grandpa_block_import, grandpa_link) =
	sc_finality_grandpa::block_import(
		client.clone(), &(client.clone() as std::sync::Arc<_>), select_chain
	)?;
```

In addition to actual blocks, this same block import will be used to import
[`Justifications`](https://substrate.dev/rustdocs/v2.0.0-rc2/sp_runtime/type.Justification.html), so we clone it
right after constructing it.

```rust, ignore
let justification_import = grandpa_block_import.clone();
```

With the GRANDPA block import created, we can now create the BABE block import. The BABE block
import is the outer-most layer of the block import onion and it wraps the GRANDPA block import.

```rust, ignore
let (babe_block_import, babe_link) = sc_consensus_babe::block_import(
	sc_consensus_babe::Config::get_or_compute(&*client)?,
	grandpa_block_import,
	client.clone(),
)?;
```

Again we are given back a BABE link which will be used to communicate with the import queue and
background authoring worker.

## The Import Queue

With the block import pipeline setup, we can proceed to creating the import queue which will feed
blocks from the network into the import pipeline. We make it using BABE's `import_queue` helper
function. Notice that it requires the BABE link, and the entire block import pipeline which we refer
to as `babe_block_import` because BABE is the outermost layer.

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

Occasionally in the operation of a blockchain, other nodes will contact our node asking for proof
that a particular block is finalized. To respond to these requests, we include a finality proof
provider.

```rust, ignore
.with_finality_proof_provider(|client, backend| {
	let provider = client as Arc<dyn StorageAndProofProvider<_, _>>;
	Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, provider)) as _)
})?
```

## Spawning the BABE Authorship Task

Any node that is acting as an authority and participating in BABE consensus, must run an `async`
authorship task. We begin by creating an instance of
[`BabeParams`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/struct.BabeParams.html).

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

## Spawning the GRANDPA Task

Just as we needed an `async` worker to author blocks with BABE, we need an `async` worker to listen
to and cast GRANDPA votes. Again, we begin by creating an instance of
[`GrandpaParams`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_finality_grandpa/struct.GrandpaParams.html)

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

### Disabled GRANDPA

Proof of Authority networks generally contain many full nodes that are not authorities. When GRANDPA
is present in the network, we still need to tell the node how to interpret GRANDPA-related messages
it may receive (just ignore them) and ensure that the correct inherents are still included in blocks
in the case that the node _is_ an authority in BABE but not GRANDPA.

```rust, ignore
sc_finality_grandpa::setup_disabled_grandpa(
	service.client(),
	&inherent_data_providers,
	service.network(),
)?;
```

## Constraints on the Runtime

### Runtime APIs

Both BABE and GRANDPA rely on getting their authority sets from the runtime via the
[BabeAPI](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/trait.BabeApi.html) and the
[GrandpaAPI](https://substrate.dev/rustdocs/v2.0.0-rc2/sp_finality_grandpa/trait.GrandpaApi.html). So trying to build
this node with a runtime that does not provide these APIs will fail to compile.

### Pre-Runtime Digests

Just as we cannot use this node with a runtime that does not provide the appropriate runtime APIs,
we also cannot use a runtime designed for this node with different consensus engines.

Because BABE is a slot-based consensus engine, it must inform the runtime which slot each block was
intended for. To do this, it uses a technique known as a pre-runtime digest. It has two kinds,
[`PrimaryPreDigest`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/struct.PrimaryPreDigest.html) and
[`SecondaryPlainPreDigest`](https://substrate.dev/rustdocs/v2.0.0-rc2/sc_consensus_babe/struct.SecondaryPlainPreDigest.html).
The BABE authorship task automatically inserts these digest items in each block it authors.

Because the runtime needs to interpret these pre-runtime digests, they are not optional. That means
runtimes that expect the pre-digests cannot be used, unmodified, in nodes
that don't provide the pre-digests. Unlike other runtimes in the Recipes where runtimes can be
freely swapped between nodes, the babe-grandpa-runtime can only be used in a node that is actually
running BABE
