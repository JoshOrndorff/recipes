//! A dead simple runtime that has a single boolean storage value and three transactions. The transactions
//! available are Set, Clear, and Toggle.

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use parity_scale_codec::{Decode, Encode};

use sp_std::if_std;
use sp_std::prelude::*;
use sp_api::impl_runtime_apis;
use sp_runtime::{
	ApplyExtrinsicResult,
	create_runtime_str,
	generic,
	MultiSignature,
	transaction_validity::{
		TransactionLongevity,
		TransactionSource,
		TransactionValidity,
		ValidTransaction
	},
	traits::{
		BlakeTwo256,
		Block as BlockT,
		Extrinsic,
		GetNodeBlockType,
		GetRuntimeBlockType,
		IdentifyAccount,
		Verify,
	}
};
// This strange-looking import is usually done by the `construct_runtime!` macro
use sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder;
#[cfg(feature = "std")]
use sp_storage::well_known_keys;

#[cfg(any(feature = "std", test))]
use sp_runtime::{BuildStorage, Storage};

use sp_core::OpaqueMetadata;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// Include the genesis helper module when building to std
#[cfg(feature = "std")]
pub mod genesis;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
/// This is not currently used in the runtime but is exposed for compatability with outer
/// nodes that expect it.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
/// This is not currently used in the runtime but is exposed for compatability with outer
/// nodes that expect it.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
//pub type Balance = u128;

/// Index of a transaction in the chain.
// pub type Index = u32;

/// A hash of some data used by the chain.
// pub type Hash = primitives::H256;

/// Digest item type.
// pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	// impl_opaque_keys! {
	//	 pub struct SessionKeys {
	//		 pub babe: Babe,
	//	 }
	// }
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("frameless-runtime"),
	impl_name: create_runtime_str!("frameless-runtime"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

/// The main struct in this module. In frame this comes from `construct_runtime!`
pub struct Runtime;

impl GetNodeBlockType for Runtime {
	type NodeBlock = opaque::Block;
}

impl GetRuntimeBlockType for Runtime {
	type RuntimeBlock = opaque::Block;
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Default))]
pub struct GenesisConfig;

#[cfg(feature = "std")]
impl BuildStorage for GenesisConfig {
	fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
		// Declare the storage items we need
		let storage_items = vec![
			(ONLY_KEY.encode(), false.encode()),
			(well_known_keys::CODE.into(), WASM_BINARY.to_vec()),
		];

		// Put them into genesis storage
		storage.top.extend(
			storage_items.into_iter()
		);

		Ok(())
	}
}

/// The address format for describing accounts.
// pub type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, FramelessTransaction>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

pub const ONLY_KEY: [u8; 1] = [0];
pub const HEADER_KEY: [u8; 6] = *b"header";

// I guess we won't need any of this when using our own unchecked extrinsic type
// The SignedExtension to the basic transaction logic.
// pub type SignedExtra = (
//	 system::CheckVersion<Runtime>,
//	 system::CheckGenesis<Runtime>,
// );
/// Unchecked extrinsic type as expected by this runtime.

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub enum FramelessTransaction {
	Set,
	Clear,
	Toggle,
	//TODO in future define call
}

impl Extrinsic for FramelessTransaction {
	type Call = ();
	type SignaturePayload = ();

	fn new(_call: Self::Call, _signed_data: Option<Self::SignaturePayload>) -> Option<Self> {
		Some(Self::Toggle)
	}
}

impl_runtime_apis! {
	// https://substrate.dev/rustdocs/master/sp_api/trait.Core.html
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			if_std!{
				println!("Entering execute_block with {:?}", block);
			}
			Self::initialize_block(&block.header);

			for transaction in block.extrinsics {
				let previous_state = sp_io::storage::get(&ONLY_KEY)
					.map(|bytes| <bool as Decode>::decode(&mut &*bytes).unwrap_or(false))
					.unwrap_or(false);

				let next_state = match (previous_state, transaction) {
					(_, FramelessTransaction::Set) => true,
					(_, FramelessTransaction::Clear) => false,
					(prev_state, FramelessTransaction::Toggle) => !prev_state,
				};

				sp_io::storage::set(&ONLY_KEY, &next_state.encode());
			}

			//TODO is this necessary? What method is it even calling?
			// In frame executive, they call final_checks, but that might be different
			Self::finalize_block();
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			if_std!{
				println!("Entering initialize_block with {:?}", header);
			}
			// Store the header info we're given for later use when finalizing block.
			sp_io::storage::set(&HEADER_KEY, &header.encode());
		}
	}

	// https://substrate.dev/rustdocs/master/sc_block_builder/trait.BlockBuilderApi.html
	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			if_std!{
				println!("Entering apply_extrinsic");
			}

			let previous_state = sp_io::storage::get(&ONLY_KEY)
				.map(|bytes| <bool as Decode>::decode(&mut &*bytes).unwrap_or(false))
				.unwrap_or(false);

			let next_state = match (previous_state, extrinsic) {
				(_, FramelessTransaction::Set) => true,
				(_, FramelessTransaction::Clear) => false,
				(prev_state, FramelessTransaction::Toggle) => !prev_state,
			};

			sp_io::storage::set(&ONLY_KEY, &next_state.encode());
			Ok(Ok(()))
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			if_std!{
				println!("Entering finalize_block");
			}
			// https://substrate.dev/rustdocs/master/sp_runtime/generic/struct.Header.html
			let raw_header = sp_io::storage::get(&HEADER_KEY)
				.expect("We initialized with header, it never got mutated, qed");

			// Clear the raw header out of storage when we are done with it.
			sp_io::storage::clear(&HEADER_KEY);

			let mut header = <Block as BlockT>::Header::decode(&mut &*raw_header)
				.expect("we put a valid header in in the first place, qed");

			let raw_state_root = &sp_io::storage::root()[..];

			let state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();
			header.state_root = state_root;
			if_std!{
				println!("Returning header: {:?}", header);
			}
			header
		}

		fn inherent_extrinsics(_data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			// I'm not using any inherents, so I guess I'll just return an empty vec
			Vec::new()
		}

		fn check_inherents(
			_block: Block,
			_data: sp_inherents::InherentData
		) -> sp_inherents::CheckInherentsResult {
			sp_inherents::CheckInherentsResult::default()
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			// Lol how bad is this? What actually depends on it?
			<Block as BlockT>::Hash::from([0u8;32])
		}
	}

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

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(vec![0])
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(_header: &<Block as BlockT>::Header) {
			// we do not do anything.
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			seed.unwrap_or_else(|| vec![0])
		}

		fn decode_session_keys(
			_encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			None
		}
	}

}
