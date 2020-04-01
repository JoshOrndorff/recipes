//! A dead simple runtime that has a single boolean storage value and three transactions. The transactions
//! available are Set, Clear, and Toggle.

// Some open questions:
// * How do I use storage? Can I do decl_storage! here like I would in a pallet?
// * What are core apis (eg. initialize_block, execute_block) actually supposed to do?
// * Which block authoring will be easiest to start with? Seems not babe because of the need to collect randomness in the runtime
// * Where does this core logic belong?

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
// Shouldn't be necessary if we're not using construct_runtime
// #![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use parity_scale_codec::{Decode, Encode};

use rstd::prelude::*;
use sp_api::impl_runtime_apis;
use sp_runtime::traits::{
    BlakeTwo256, Block as BlockT, Extrinsic, GetNodeBlockType, GetRuntimeBlockType,
};
use sp_runtime::{
    create_runtime_str, generic,
    transaction_validity::{TransactionLongevity, TransactionValidity, ValidTransaction},
    ApplyExtrinsicResult,
};
#[cfg(any(feature = "std", test))]
use sp_runtime::{BuildStorage, Storage};

use primitives::{crypto::Public, OpaqueMetadata};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
// pub type Signature = AnySignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
// pub type AccountId = <Signature as Verify>::Signer;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
// pub type AccountIndex = u32;

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

    // pub type SessionHandlers = Babe;

    // impl_opaque_keys! {
    //     pub struct SessionKeys {
    //         pub babe: Babe,
    //     }
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
};

/// Constants for Babe.

/// Since BABE is probabilistic this is the average expected block time that
/// we are targetting. Blocks will be produced at a minimum duration defined
/// by `SLOT_DURATION`, but some slots will not be allocated to any
/// authority and hence no block will be produced. We expect to have this
/// block time on average following the defined slot duration and the value
/// of `c` configured for BABE (where `1 - c` represents the probability of
/// a slot being empty).
/// This value is only used indirectly to define the unit constants below
/// that are expressed in blocks. The rest of the code should use
/// `SLOT_DURATION` instead (like the timestamp pallet for calculating the
/// minimum period).
/// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

pub const EPOCH_DURATION_IN_BLOCKS: u32 = 100;

// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

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
    fn assimilate_storage(&self, _storage: &mut Storage) -> Result<(), String> {
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
//     system::CheckVersion<Runtime>,
//     system::CheckGenesis<Runtime>,
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
            //TODO prolly need to do something with pre-runtime digest? This may be another reason to use
            // consensus that is totally out of the runtime.
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
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            // Store the header info we're give nfor later use when finalizing block.
            sp_io::storage::set(&HEADER_KEY, &header.encode());
        }
    }

    // https://substrate.dev/rustdocs/master/sp_api/trait.Metadata.html
    // "The Metadata api trait that returns metadata for the runtime."
    // impl sp_api::Metadata<Block> for Runtime {
    //     fn metadata() -> OpaqueMetadata {
    //         // Runtime::metadata().into()
    //         // Maybe this one can be omitted or just return () or something?
    //         // Would be really cool to return something that makes polkadot-js api happy,
    //         // but that seems unlikely.
    //         unimplemented!()
    //     }
    // }

    // https://substrate.dev/rustdocs/master/sc_block_builder/trait.BlockBuilderApi.html
    impl block_builder_api::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(_extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            // Executive::apply_extrinsic(extrinsic)
            // Maybe this is where the core flipping logic goes?
            Ok(Ok(()))
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            // https://substrate.dev/rustdocs/master/sp_runtime/generic/struct.Header.html
            let raw_header = sp_io::storage::get(&HEADER_KEY)
                .expect("We initialized with header, it never got mutated, qed");

            let mut header = <Block as BlockT>::Header::decode(&mut &*raw_header)
                .expect("we put a valid header in in the first place, qed");

            let raw_root = &sp_io::storage::root()[..];

            header.state_root = primitives::H256::from(sp_io::hashing::blake2_256(raw_root));
            header
        }

        fn inherent_extrinsics(_data: inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            // I'm not using any inherents, so I guess I'll just return an empty vec
            Vec::new()
        }

        fn check_inherents(
            _block: Block,
            _data: inherents::InherentData
        ) -> inherents::CheckInherentsResult {
            // I'm not using any inherents, so it should be safe to just return ok
            inherents::CheckInherentsResult::default()
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            // Lol how bad is this? What actually depends on it?
            <Block as BlockT>::Hash::from([0u8;32])
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(_tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
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

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> u64 {
            3000 //milliseconds
        }


        fn authorities() -> Vec<AuraId> {
            // Do we need 32 bytes?
            let alice = AuraId::from_slice(&[0]);

            let mut authorities = Vec::new();
            authorities.push(alice);
            authorities
        }
    }

    impl sp_finality_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            let alice = GrandpaId::from_slice(&[0]);

            let mut authorities = Vec::new();
            authorities.push((alice, 1));
            authorities
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(vec![0])
        }
    }

    impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(_header: &<Block as BlockT>::Header) {
            // we do not do anything.
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            seed.unwrap_or(vec![0])
        }

        fn decode_session_keys(
            _encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, primitives::crypto::KeyTypeId)>> {
            None
        }
    }

}
