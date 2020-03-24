use sp_core::{U256, H256};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::Block as BlockT;
use parity_scale_codec::{Encode, Decode};
use sc_consensus_pow::{PowAlgorithm, Error};
use sp_consensus_pow::Seal as RawSeal;
use sha3::{Sha3_256, Digest};
use rand::{thread_rng, SeedableRng, rngs::SmallRng};

/// A concrete PoW Algorithm that uses Sha3 hashing.
#[derive(Clone)]
pub struct Sha3Algorithm;

/// Determine whether the given hash satisfies the given difficulty.
/// The test is done by multiplying the two together. If the product
/// overflows the bounds of U256, then the product (and thus the hash)
/// was too high.
fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
	let num_hash = U256::from(&hash[..]);
	let (_, overflowed) = num_hash.overflowing_mul(difficulty);

	!overflowed
}

/// A Seal struct that will be encoded to a Vec<u8> as used as the
/// `RawSeal` type.
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Seal {
	pub difficulty: U256,
	pub work: H256,
	pub nonce: H256,
}

/// A not-yet-computed attempt to solve the proof of work. Calling the
/// compute method will compute the hash and return the seal.
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Compute {
	pub difficulty: U256,
	pub pre_hash: H256,
	pub nonce: H256,
}

impl Compute {
	pub fn compute(self) -> Seal {
		let work = H256::from_slice(Sha3_256::digest(&self.encode()[..]).as_slice());

		Seal {
			nonce: self.nonce,
			difficulty: self.difficulty,
			work: work,
		}
	}
}

// Here we implement the general PowAlgorithm trait for our concrete Sha3Algorithm
impl<B: BlockT<Hash=H256>> PowAlgorithm<B> for Sha3Algorithm {
	type Difficulty = U256;

	fn difficulty(&self, _parent: &BlockId<B>) -> Result<Self::Difficulty, Error<B>> {
		// This basic PoW uses a fixed difficulty.
		// Raising this difficulty will make the block time slower.
		Ok(U256::from(1000_000))
	}

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
}
