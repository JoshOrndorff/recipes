//! Weight Fee Runtime.
//!
//! This runtime demonstrates several ways to convert weights to fees and how to charge
//! fees in various assets.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// Include the genesis helper module when building to std
#[cfg(feature = "std")]
pub mod genesis;

use sp_std::prelude::*;
use sp_core::{OpaqueMetadata, H256};
use sp_runtime::{
	MultiSignature,
	ApplyExtrinsicResult,
	create_runtime_str,
	generic,
	transaction_validity::{TransactionValidity, TransactionSource},
};
use sp_runtime::traits::{
	BlakeTwo256,
	Block as BlockT,
	Convert,
	IdentifyAccount,
	IdentityLookup,
	Verify,
};
use frame_support::{
	traits::Get,
	weights::Weight,
};
use sp_api::impl_runtime_apis;
use sp_version::RuntimeVersion;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use frame_system as system;

// These structs are used in one of the commented-by-default implementations of
// transaction_payment::Trait. Don't warn when they are unused.
#[allow(unused_imports)]
use sp_runtime::traits::ConvertInto;
#[allow(unused_imports)]
use generic_asset::{SpendingAssetCurrency, AssetCurrency, AssetIdProvider};

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use sp_runtime::{Permill, Perbill};
pub use frame_support::{StorageValue, construct_runtime, parameter_types, traits::Randomness};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

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
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("weight-fee-runtime"),
	impl_name: create_runtime_str!("weight-fee-runtime"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const MaximumBlockWeight: Weight = 1_000_000_000;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
	pub const Version: RuntimeVersion = VERSION;
}

impl system::Trait for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = IdentityLookup<AccountId>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Maximum weight of each block. With a default weight system of 1byte == 1weight, 4mb is ok.
	type MaximumBlockWeight = MaximumBlockWeight;
	/// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
	type MaximumBlockLength = MaximumBlockLength;
	/// Portion of the block weight that is available to all normal transactions.
	type AvailableBlockRatio = AvailableBlockRatio;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type ModuleToIndex = ModuleToIndex;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = balances::AccountData<Balance>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}

impl timestamp::Trait for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const TransferFee: u128 = 0;
	pub const CreationFee: u128 = 0;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
}

impl generic_asset::Trait for Runtime {
    /// The type for recording an account's balance.
    type Balance = Balance;
    type AssetId = u32;
    type Event = Event;
}

impl sudo::Trait for Runtime {
	type Event = Event;
	type Call = Call;
}

impl weights::Trait for Runtime {}


// --------------------- Multiple Options for WeightToFee -----------------------

/// Convert from weight to balance via a simple coefficient multiplication. The associated type C
/// encapsulates a constant in units of balance per weight.
pub struct LinearWeightToFee<C>(sp_std::marker::PhantomData<C>);

impl<C> Convert<Weight, Balance> for LinearWeightToFee<C>
	where C: Get<Balance> {

	fn convert(w: Weight) -> Balance {
		// substrate-node a weight of 10_000 (smallest non-zero weight) to be mapped to 10^7 units of
		// fees, hence:
		let coefficient = C::get();
		Balance::from(w).saturating_mul(coefficient)
	}
}

/// Convert from weight to balance via a quadratic curve. The type parameters encapsulate the
/// coefficients.
pub struct QuadraticWeightToFee<C0, C1, C2>(C0, C1, C2);

impl<C0, C1, C2> Convert<Weight, Balance> for QuadraticWeightToFee<C0, C1, C2>
	where C0: Get<Balance>, C1: Get<Balance>, C2: Get<Balance> {

	fn convert(w: Weight) -> Balance {
		let c0 = C0::get();
		let c1 = C1::get();
		let c2 = C2::get();
		let w = Balance::from(w);

		// All the safe math reduces to
		// c0 + c1 * w + c2 * w * w

		let c1w = c1.saturating_mul(w);
		let c2w2 = c2.saturating_mul(w).saturating_mul(w);

		c0.saturating_add(c1w).saturating_add(c2w2)
	}
}

// --------------------- An Option to Currency to Collect Fees -----------------------
#[allow(dead_code)]
type FixedGenericAsset<T> = AssetCurrency<T, FixedAssetId>;

pub struct FixedAssetId;
impl AssetIdProvider for FixedAssetId {
	type AssetId = u32;
	fn asset_id() -> Self::AssetId {
		13
	}
}

parameter_types! {
	// Used with LinearWeightToFee conversion. Leaving this constant in tact when using other
	// conversion techniques is harmless.
	pub const FeeWeightRatio: u128 = 1_000;

	// Used with QuadraticWeightToFee conversion. Leaving these constants in tact when using other
	// conversion techniques is harmless.
	pub const WeightFeeConstant: u128 = 1_000;
	pub const WeightFeeLinear: u128 = 100;
	pub const WeightFeeQuadratic : u128 = 10;

	// Establish the base- and byte-fees. These are used in all configurations.
	pub const TransactionBaseFee: u128 = 0;
	pub const TransactionByteFee: u128 = 1;
}

impl transaction_payment::Trait for Runtime {

	// The asset in which fees will be collected.
	// Enable exactly one of the following options.
	type Currency = Balances; // The balances pallet (The most common choice)
	//type Currency = FixedGenericAsset<Self>; // A generic asset whose ID is hard-coded above.
	//type Currency = SpendingAssetCurrency<Self>; // A generic asset whose ID is stored in the
	                                               // generic_asset pallet's runtime storage

	// What to do when fees are paid. () means take no additional actions.
	type OnTransactionPayment = ();

	// Base fee is a fixed amount applied to every transaction
	type TransactionBaseFee = TransactionBaseFee;

	// Byte fee is multiplied by the length of the
	// serialized transaction in bytes
	type TransactionByteFee = TransactionByteFee;

	// Function to convert dispatch weight to a chargeable fee.
	// Enable exactly one of the following options.
	//type WeightToFee = ConvertInto;
	//type WeightToFee = LinearWeightToFee<FeeWeightRatio>;
	type WeightToFee = QuadraticWeightToFee<WeightFeeConstant, WeightFeeLinear, WeightFeeQuadratic>;

	//TODO Explore how to change FeeMultiplierUpdate
	type FeeMultiplierUpdate = ();
}

// --------------------------------------------


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Module, Call, Storage, Config, Event<T>},
		Timestamp: timestamp::{Module, Call, Storage, Inherent},
		Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
		GenericAsset: generic_asset::{Module, Call, Storage, Config<T>, Event<T>},
		RandomnessCollectiveFlip: randomness_collective_flip::{Module, Call, Storage},
		Sudo: sudo::{Module, Call, Config<T>, Storage, Event<T>},
		TransactionPayment: transaction_payment::{Module, Storage},
		// The Recipe Pallets
		Weights: weights::{Module, Call, Storage},
	}
);

/// The address format for describing accounts.
pub type Address = AccountId;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	system::CheckVersion<Runtime>,
	system::CheckGenesis<Runtime>,
	system::CheckEra<Runtime>,
	system::CheckNonce<Runtime>,
	system::CheckWeight<Runtime>,
	transaction_payment::ChargeTransactionPayment<Runtime>
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various pallets.
pub type Executive = frame_executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Runtime, AllModules>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			RandomnessCollectiveFlip::random_seed()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
			Vec::new()
		}

		fn decode_session_keys(
			_encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			None
		}
	}
}
