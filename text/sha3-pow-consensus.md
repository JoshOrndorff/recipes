# Sha3 Proof of Work Algorithms

`consensus/sha3-pow`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fconsensus%2Fsha3-pow%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/consensus/sha3-pow/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

[Proof of Work](https://en.wikipedia.org/wiki/Proof_of_work) is not a single consensus algorithm.
Rather it is a class of algorithms represented in Substrate by the
[`PowAlgorithm` trait](https://substrate.dev/rustdocs/v3.0.0/sc_consensus_pow/trait.PowAlgorithm.html). Before we
can build a PoW node we must specify a concrete PoW algorithm by implementing this trait. In this
recipe we specify two concrete PoW algorithms, both of which are based on the
[sha3 hashing algorithm](https://en.wikipedia.org/wiki/SHA-3).

## Minimal Sha3 PoW

First we turn our attention to a minimal working implementation. This consensus engine is kept
intentionally simple. It omits some features that make Proof of Work practical for real-world use
such as difficulty adjustment.

Begin by creating a struct that will implement the `PowAlgorithm Trait`.

```rust, ignore
/// A minimal PoW algorithm that uses Sha3 hashing.
/// Difficulty is fixed at 1_000_000
#[derive(Clone)]
pub struct MinimalSha3Algorithm;
```

Because this is a _minimal_ PoW algorithm, our struct can also be quite simple. In fact, it is a
[unit struct](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html). A more complex
PoW algorithm that interfaces with the runtime would need to hold a reference to the client. An
example of this (on an older Substrate codebase) can be seen in
[Kulupu](https://github.com/kulupu/kulupu/)'s
[RandomXAlgorithm](https://github.com/kulupu/kulupu/blob/3500b7f62fdf90be7608b2d813735a063ad1c458/pow/src/lib.rs#L137-L145).

### Difficulty

The first function we must provide returns the difficulty of the next block to be mined. In our
minimal sha3 algorithm, this function is quite simple. The difficulty is fixed. This means that as
more mining power joins the network, the block time will become faster.

```rust, ignore
impl<B: BlockT<Hash = H256>> PowAlgorithm<B> for MinimalSha3Algorithm {
	type Difficulty = U256;

	fn difficulty(&self, _parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
		// Fixed difficulty hardcoded here
		Ok(U256::from(1_000_000))
	}

	// --snip--
}
```

### Verification

Our PoW algorithm must also be able to verify blocks provided by other authors. We are first given
the pre-hash, which is a hash of the block before the proof of work seal is attached. We are also
given the seal, which testifies that the work has been done, and the difficulty that the block
author needed to meet. This function first confirms that the provided seal actually meets the target
difficulty, then it confirms that the seal is actually valid for the given pre-hash.

```rust, ignore
fn verify(
	&self,
	_parent: &BlockId<B>,
	pre_hash: &H256,
	_pre_digest: Option<&[u8]>,
	seal: &RawSeal,
	difficulty: Self::Difficulty,
) -> Result<bool, Error<B>> {
	// Try to construct a seal object by decoding the raw seal given
	let seal = match Seal::decode(&mut &seal[..]) {
		Ok(seal) => seal,
		Err(_) => return Ok(false),
	};

	// See whether the hash meets the difficulty requirement. If not, fail fast.
	if !hash_meets_difficulty(&seal.work, difficulty) {
		return Ok(false);
	}

	// Make sure the provided work actually comes from the correct pre_hash
	let compute = Compute {
		difficulty,
		pre_hash: *pre_hash,
		nonce: seal.nonce,
	};

	if compute.compute() != seal {
		return Ok(false);
	}

	Ok(true)
}
```

## Realistic Sha3 PoW

Having understood the fundamentals, we can now build a more realistic sha3 algorithm. The primary
difference here is that this algorithm will fetch the difficulty from the runtime via a
[runtime api](./runtime-api.md). This change allows the runtime to dynamically adjust the difficulty
based on block time. So if more mining power joins the network, the diffculty adjusts, and the
blocktime remains constant.

### Defining the `Sha3Algorithm` Struct

We begin as before by defining a struct that will implement the `PowAlgorithm` trait. Unlike before,
this struct must hold a reference to the
[`Client`](https://substrate.dev/rustdocs/v3.0.0/sc_service/client/struct.Client.html) so it can call the
appropriate runtime APIs.

```rust, ignore
/// A complete PoW Algorithm that uses Sha3 hashing.
/// Needs a reference to the client so it can grab the difficulty from the runtime.
pub struct Sha3Algorithm<C> {
	client: Arc<C>,
}
```

Next we provide a `new` method for conveniently creating instances of our new struct.

```rust, ignore
impl<C> Sha3Algorithm<C> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}
```

And finally we manually implement `Clone`. We cannot derive clone as we did for the
`MinimalSha3Algorithm`.

```rust, ignore
// Manually implement clone. Deriving doesn't work because
// it'll derive impl<C: Clone> Clone for Sha3Algorithm<C>. But C in practice isn't Clone.
impl<C> Clone for Sha3Algorithm<C> {
	fn clone(&self) -> Self {
		Self::new(self.client.clone())
	}
}
```

> It isn't critical to understand _why_ the manual `Clone` implementation is necessary, just that it
> is necessary.

### Implementing the `PowAlgorithm` trait

As before we implement the `PowAlgorithm` trait for our `Sha3Algorithm`. This time we supply more
complex trait bounds to ensure that the encapsulated client actually provides
the [`DifficultyAPI`](https://substrate.dev/rustdocs/v3.0.0/sp_consensus_pow/trait.DifficultyApi.html) necessary
to fetch the PoW difficulty from the runtime.

```rust, ignore
impl<B: BlockT<Hash = H256>, C> PowAlgorithm<B> for Sha3Algorithm<C>
where
	C: ProvideRuntimeApi<B>,
	C::Api: DifficultyApi<B, U256>,
{
	type Difficulty = U256;

	// --snip
}
```

The implementation of `PowAlgorithm`'s `difficulty` function, no longer returns a fixed value, but
rather calls into the runtime API which is guaranteed to exist because of the trait bounds. It also
maps any errors that may have occurred when using the API.

```rust, ignore
fn difficulty(&self, parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
	let parent_id = BlockId::<B>::hash(parent);
	self.client
		.runtime_api()
		.difficulty(&parent_id)
		.map_err(|err| {
			sc_consensus_pow::Error::Environment(
				format!("Fetching difficulty from runtime failed: {:?}", err)
			)
		})
}
```

The `verify` function is unchanged from the `MinimalSha3Algorithm` implementation.
