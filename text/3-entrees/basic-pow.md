# Basic Proof of Work
*[`nodes/basic-pow`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/basic-pow)*

The `basic-pow` node uses a minimal [Proof of Work](https://en.wikipedia.org/wiki/Proof_of_work) consensus engine to reach agreement over the blockchain. This node is kept intentionally simple. It omits some features that make Proof of Work practical for real-world use such as difficulty adjustment and block rewards. Nonetheless, it is a real usable consensus engine that will teach us many useful aspects of dealing with consensus and prepare us to understand more advanced consensus engines in the future. In particular we will learn about:
* Substrate's [`BlockImport` trait](https://substrate.dev/rustdocs/master/sp_consensus/block_import/trait.BlockImport.html)
* Substrate's [import pipeline](https://substrate.dev/rustdocs/master/sp_consensus/import_queue/index.html)
* Structure of a typical [Substrate Service](https://substrate.dev/rustdocs/master/sc_service/index.html)
* Configuration of [`InherentDataProvider`](https://substrate.dev/rustdocs/master/sp_authorship/struct.InherentDataProvider.html)s

## The Structure of a Node

You may remember from the [hello-substrate recipe](../2-appetizers/1-hello-substrate.md) that a Substrate node has two parts. An outer part that is responsible for gossiping transactions and blocks, handling [rpc requests](./custom-rpc.md), and reaching consensus. And a runtime that is responsible for the business logic of the chain. This architecture diagram illustrates the distinction.

![Substrate Architecture Diagram](../img/substrate-architecture.png)

In principle the consensus engine, part of the outer node, is agnostic over the runtime that is used with it. But in practice, most consensus engines will require the runtime to provide certain [runtime APIs](./runtime-api.md) that affect the engine. For example, Aura and Babe query the runtime for the set of validators. A more real-world PoW consensus would query the runtime for the block difficulty. Additionally, some runtimes rely on the consensus engine to provide [pre-runtime digests](https://substrate.dev/rustdocs/master/sp_runtime/generic/enum.DigestItem.html#variant.PreRuntime). For example, runtimes that include the Babe pallet expect a pre-runtime digest containing information about the current babe slot. Because of these requirements, this node will use a dedicated `pow-runtime`. The contents of that runtime should be familiar, and will not be discussed here.

## Proof of Work Algorithms

Proof of work is not a single consensus algorithm. Rather it is a class of algorithms represented by the [`PowAlgorithm` trait](https://substrate.dev/rustdocs/master/sc_consensus_pow/trait.PowAlgorithm.html). Before we can build a PoW node we must specify a concrete PoW algorithm by implementing this trait. We specify our algorithm in the `pow.rs` file.

```rust, ignore
/// A concrete PoW Algorithm that uses Sha3 hashing.
#[derive(Clone)]
pub struct Sha3Algorithm;
```

We will use the [sha3 hashing algorithm](https://en.wikipedia.org/wiki/SHA-3), which we have indicated in the name of our struct. Because this is a _minimal_ PoW algorithm, our struct can also be quite simple. In fact, it is a [unit struct](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html). A more complex PoW algorithm that interfaces with the runtime would need to hold a reference to the client. An example of this (on an older Substrate codebase) can be seen in [Kulupu](https://github.com/kulupu/kulupu/)'s [RandomXAlgorithm](https://github.com/kulupu/kulupu/blob/3500b7f62fdf90be7608b2d813735a063ad1c458/pow/src/lib.rs#L137-L145).

### Difficulty

The first fucntion we must provide returns the difficulty of the next block to be mined. In our basic PoW, this function is quite simple. The difficulty is fixed. This means that as more mining power joins the network, the block time will become faster.

```rust, ignore
impl<B: BlockT<Hash=H256>> PowAlgorithm<B> for Sha3Algorithm {
	type Difficulty = U256;

	fn difficulty(&self, _parent: &BlockId<B>) -> Result<Self::Difficulty, Error<B>> {
		// This basic PoW uses a fixed difficulty.
		// Raising this difficulty will make the block time slower.
		Ok(U256::from(1_000_000))
	}

	// --snip--
}
```

### Verification

Our PoW algorithm must also be able to verify blocks provided by other authors. We are first given the pre-hash, which is a hash of the block before the proof of work seal is attached. We are also given the seal, which testifies that the work has been done, and the difficulty that the block author needed to meet. This function first confirms that the provided seal actually meets the target difficulty, then it confirms that the seal is actually valid for the given pre-hash.

```rust, ignore
fn verify(
	&self,
	_parent: &BlockId<B>,
	pre_hash: &H256,
	seal: &RawSeal,
	difficulty: Self::Difficulty
) -> Result<bool, Error<B>> {
	// Try to construct a seal object by decoding the raw seal given
	let seal = match Seal::decode(&mut &seal[..]) {
		Ok(seal) => seal,
		Err(_) => return Ok(false),
	};

	// See whether the hash meets the difficulty requirement. If not, fail fast.
	if !hash_meets_difficulty(&seal.work, difficulty) {
		return Ok(false)
	}

	// Make sure the provided work actually comes from the correct pre_hash
	let compute = Compute {
		difficulty,
		pre_hash: *pre_hash,
		nonce: seal.nonce,
	};

	if compute.compute() != seal {
		return Ok(false)
	}

	Ok(true)
}
```

### Mining

Finally our proof of work algorithm needs to be able to mine blocks of our own.

```rust, ignore
fn mine(
	&self,
	_parent: &BlockId<B>,
	pre_hash: &H256,
	difficulty: Self::Difficulty,
	round: u32 // The number of nonces to try during this call
) -> Result<Option<RawSeal>, Error<B>> {
	// Get a randomness source from the environment; fail if one isn't available
	let mut rng = SmallRng::from_rng(&mut thread_rng())
		.map_err(|e| Error::Environment(format!("Initialize RNG failed for mining: {:?}", e)))?;

	// Loop the specified number of times
	for _ in 0..round {

		// Choose a new nonce
		let nonce = H256::random_using(&mut rng);

		// Calculate the seal
		let compute = Compute {
			difficulty,
			pre_hash: *pre_hash,
			nonce,
		};
		let seal = compute.compute();

		// If we solved the PoW then return, otherwise loop again
		if hash_meets_difficulty(&seal.work, difficulty) {
			return Ok(Some(seal.encode()))
		}
	}

	// Tried the specified number of rounds and never found a solution
	Ok(None)
}
```

Notice that this function takes a parameter for the number of rounds of mining it should attempt. If no block has been successfully mined in this time, the method will return. This gives the service a chance to check whether any new blocks have been received from other authors since the mining started. If a valid block has been received, then we will start mining on it. If no such block has been received, we will go in for another try at mining on the same block as before.

## The Service Builder

The [Substrate Service](https://substrate.dev/rustdocs/master/sc_service/trait.AbstractService.html) is the main coordinator of the various parts of a Substrate node, including consensus. The service is large and takes many parameters, so it is built with a [ServiceBuilder](https://substrate.dev/rustdocs/master/sc_service/struct.ServiceBuilder.html) following [Rust's builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).

The particular builder method that is relevant here is [`with_import_queue`](https://substrate.dev/rustdocs/master/sc_service/struct.ServiceBuilder.html#method.with_import_queue). Here we construct an instance of the [`PowBlockImport` struct](https://substrate.dev/rustdocs/master/sc_consensus_pow/struct.PowBlockImport.html), providing it with references to our client, our Sha3Algorithm, and some other necessary data.

```rust, ignore
builder
	.with_import_queue(|_config, client, select_chain, _transaction_pool| {

		let pow_block_import = sc_consensus_pow::PowBlockImport::new(
			client.clone(),
			client.clone(),
			crate::pow::Sha3Algorithm,
			0, // check inherents starting at block 0
			select_chain,
			inherent_data_providers.clone(),
		);

		let import_queue = sc_consensus_pow::import_queue(
			Box::new(pow_block_import.clone()),
			crate::pow::Sha3Algorithm,
			inherent_data_providers.clone(),
		)?;

		import_setup = Some(pow_block_import);

		Ok(import_queue)
	})?;
```

Once the `PowBlockImport` is constructed, we can use it to create an actual import queue that the service will use for importing blocks into the client.

### The Block Import Pipeline

You may have noticed that when we created the `PowBlockImport` we gave it two separate references to the client. The second reference will always be to a client. But the first is interesting. The rustdocs tell us that the first parameter is `inner: BlockImport<B, Transaction = TransactionFor<C, B>>`. Why would a block import have a reference to another block import? Because the "block import pipeline" is constructed in an onion-like fashion, where one layer of block import wraps the next. In this minimal PoW node, there is only one layer to the onion. But in other nodes, including our own kitchen node, there are two layers: one for babe and one for grandpa.

### Inherent Data Providers

Both the BlockImport and the `import_queue` are given an instance called `inherent_data_providers`. This object is created in a helper function defined at the beginning of `service.rs`

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

Anything that implements the [`ProvideInherentData` trait](https://substrate.dev/rustdocs/master/sp_inherents/trait.ProvideInherentData.html) may be used here. The block authoring logic must supply all inherents that the runtime expects. In this case of this basic-pow chain, that is just the [`TimestampInherentData`](https://substrate.dev/rustdocs/master/sp_timestamp/trait.TimestampInherentData.html) expected by the [timestamp pallet](https://substrate.dev/rustdocs/master/pallet_timestamp/index.html). In order to register other inherents, you would call `register_provider` multiple times, and map errors accordingly.

## Mining

We've already implemented a mining algorithm as part of our `Sha3Algorithm`, but we haven't yet told our service to actually mine with that algorithm. This is our last task in the `new_full` function.

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
		Sha3Algorithm,
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

We begin by testing whether this node participates in consensus, which is to say we check whether the user wants the node to act as a miner. If this node is to be a miner, we gather references to various parts of the node that the [`start_mine` function](https://substrate.dev/rustdocs/master/sc_consensus_pow/fn.start_mine.html) requires, and define that we will attempt 500 rounds of mining for each block before pausing. Finally we call `start_mine`.

## The Light Client

The last thing in the `service.rs` file is constructing the [light client](https://www.parity.io/what-is-a-light-client/)'s service. This code is quite similar to the construction of the full service.

Instead of using the `with_import_queue` function we used previously, we use the `with_import_queue_and_fprb` function. FPRB stand for [`FinalityProofRequestBuilder`](https://substrate.dev/rustdocs/master/sc_network/config/trait.FinalityProofRequestBuilder.html). In chains with deterministic finality, light clients must request proofs of finality from full nodes. But in our chain, we do not have deterministic finality, so we can use the [`DummyFinalityProofRequestBuilder`](https://substrate.dev/rustdocs/master/sc_network/config/struct.DummyFinalityProofRequestBuilder.html) which does nothing except satisfying Rust's type checker.

Once the dummy request builder is configured, the `BlockImport` and import queue are configured exactly as they were in the full node.

## Note of Finality

If we run the `basic-pow` node now, we see in console logs, that the finalized block always remains at 0.

```
...
2020-03-22 12:50:09 Starting consensus session on top of parent 0x85811577d1033e918b425380222fd8c5aef980f81fa843d064d80fe027c79f5a
2020-03-22 12:50:09 Imported #189 (0x8581…9f5a)
2020-03-22 12:50:09 Prepared block for proposing at 190 [hash: 0xdd83ba96582acbed59aacd5304a9258962d1d4c2180acb8b77f725bd81461c4f; parent_hash: 0x8581…9f5a; extrinsics (1): [0x77a5…f7ad]]
2020-03-22 12:50:10 Idle (1 peers), best: #189 (0x8581…9f5a), finalized #0 (0xff0d…5cb9), ⬇ 0.2kiB/s ⬆ 0.4kiB/s
2020-03-22 12:50:15 Idle (1 peers), best: #189 (0x8581…9f5a), finalized #0 (0xff0d…5cb9), ⬇ 0 ⬆ 0
```

This is expected because Proof of Work is a consensus mechanism with probabilistic finality. This means a block is never truly finalized and can always be reverted. The further behind the blockchain head a block is, the less likely it is going to be reverted.
