//! A Super Runtime.
//!
//! This runtime demonstrates most of the recipe pallets in a single super runtime.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// Include the genesis helper module when building to std
#[cfg(feature = "std")]
pub mod genesis;

use check_membership::{loose as check_membership_loose, tight as check_membership_tight};
use frame_system as system;
use pallet_transaction_payment::CurrencyAdapter;
use sp_api::impl_runtime_apis;
use sp_core::{OpaqueMetadata, H256};
use sp_runtime::traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, IdentityLookup, Verify};
use sp_runtime::{
	create_runtime_str, generic,
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
	construct_runtime, parameter_types,
	traits::Randomness,
	weights::{
		constants::{RocksDbWeight, WEIGHT_PER_SECOND},
		IdentityFee,
	},
};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

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
	spec_name: create_runtime_str!("super-runtime"),
	impl_name: create_runtime_str!("super-runtime"),
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

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
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
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const TransferFee: u128 = 0;
	pub const CreationFee: u128 = 0;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const TransactionByteFee: u128 = 1;
}

impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}

// ---------------------- Recipe Pallet Configurations ----------------------

impl basic_token::Config for Runtime {
	type Event = Event;
}

impl charity::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
}

impl compounding_interest::Config for Runtime {
	type Event = Event;
}

parameter_types! {
	pub const MaxAddend: u32 = 1738;
	pub const ClearFrequency: u32 = 10;
}

impl constant_config::Config for Runtime {
	type Event = Event;
	type MaxAddend = MaxAddend;
	type ClearFrequency = ClearFrequency;
}

// The following two configuration traits are for the loosely and tightly coupled variants
// of the check membership pallet. Both pallets are located in the same `check-membership` crate.
impl check_membership_loose::Config for Runtime {
	type Event = Event;
	// You can choose either the `vec-set` or `map-set` implementation of the `AccountSet` trait
	type MembershipSource = VecSet;
	// type MembershipSource = MapSet;
}

impl check_membership_tight::Config for Runtime {
	type Event = Event;
}

// The following two configuration traits are for two different instances of the deafult-instance
// pallet. Notice that only the second instance has to explicitly specify an instance.
impl default_instance::Config for Runtime {
	type Event = Event;
}

impl default_instance::Config<default_instance::Instance2> for Runtime {
	type Event = Event;
}

impl double_map::Config for Runtime {
	type Event = Event;
}

impl fixed_point::Config for Runtime {
	type Event = Event;
}

impl generic_event::Config for Runtime {
	type Event = Event;
}

impl hello_substrate::Config for Runtime {}

// The following two configuration traits are for two different instances of the last-caller pallet
impl last_caller::Config<last_caller::Instance1> for Runtime {
	type Event = Event;
}

impl last_caller::Config<last_caller::Instance2> for Runtime {
	type Event = Event;
}

impl map_set::Config for Runtime {
	type Event = Event;
}

impl ringbuffer_queue::Config for Runtime {
	type Event = Event;
}

impl randomness::Config for Runtime {
	type Event = Event;
	type RandomnessSource = RandomnessCollectiveFlip;
}

impl reservable_currency::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
}

parameter_types! {
	pub const SubmissionDeposit: u128 = 10;
	pub const MinContribution: u128 = 10;
	pub const RetirementPeriod: u32 = 10;
}

impl simple_crowdfund::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type SubmissionDeposit = SubmissionDeposit;
	type MinContribution = MinContribution;
	type RetirementPeriod = RetirementPeriod;
}

impl simple_event::Config for Runtime {
	type Event = Event;
}

impl simple_map::Config for Runtime {
	type Event = Event;
}

impl storage_cache::Config for Runtime {
	type Event = Event;
}

impl struct_storage::Config for Runtime {
	type Event = Event;
}

impl vec_set::Config for Runtime {
	type Event = Event;
}

// ---------------------- End of Recipe Pallet Configurations ----------------------

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Module, Call, Storage, Config, Event<T>},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
		Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Module, Storage},
		// The Recipe Pallets
		BasicToken: basic_token::{Module, Call, Storage, Event<T>},
		Charity: charity::{Module, Call, Storage, Config, Event<T>},
		CheckMembershipLoose: check_membership_loose::{Module, Call, Event<T>},
		CheckMembershipTight: check_membership_tight::{Module, Call, Event<T>},
		CompoundingInterest: compounding_interest::{Module, Call, Storage, Event},
		ConstantConfig: constant_config::{Module, Call, Storage, Event},
		DefaultInstance1: default_instance::{Module, Call, Storage, Event<T>},
		DefaultInstance2: default_instance::<Instance2>::{Module, Call, Storage, Event<T>},
		DoubleMap: double_map::{Module, Call, Storage, Event<T>},
		FixedPoint: fixed_point::{Module, Call, Storage, Event},
		HelloSubstrate: hello_substrate::{Module, Call},
		GenericEvent: generic_event::{Module, Call, Event<T>},
		LastCaller1: last_caller::<Instance1>::{Module, Call, Storage, Event<T>},
		LastCaller2: last_caller::<Instance2>::{Module, Call, Storage, Event<T>},
		MapSet: map_set::{Module, Call, Storage, Event<T>},
		RingbufferQueue: ringbuffer_queue::{Module, Call, Storage, Event<T>},
		RandomnessDemo: randomness::{Module, Call, Storage, Event},
		ReservableCurrency: reservable_currency::{Module, Call, Event<T>},
		SimpleCrowdfund: simple_crowdfund::{Module, Call, Storage, Event<T>},
		SimpleEvent: simple_event::{Module, Call, Event},
		SimpleMap: simple_map::{Module, Call, Storage, Event<T>},
		StorageCache: storage_cache::{Module, Call, Storage, Event<T>},
		StructStorage: struct_storage::{Module, Call, Storage, Event<T>},
		VecSet: vec_set::{Module, Call, Storage, Event<T>},
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
	system::CheckSpecVersion<Runtime>,
	system::CheckTxVersion<Runtime>,
	system::CheckGenesis<Runtime>,
	system::CheckEra<Runtime>,
	system::CheckNonce<Runtime>,
	system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various pallets.
pub type Executive =
	frame_executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Runtime, AllModules>;

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
