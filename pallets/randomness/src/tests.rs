use crate::{Event, Module, Trait};
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system::{self as system, EventRecord, Phase};
use pallet_babe::SameAuthoritiesForever;
use sp_core::H256;
use sp_io;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for TestRuntime {
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

mod randomness {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		randomness,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
	type CollectiveFlipRandomnessSource = CollectiveFlip;
	type BabeRandomnessSource = Babe;
}

parameter_types! {
	pub const EpochDuration: u64 = 10;
	pub const ExpectedBlockTime: u64 = 3000;
}

impl pallet_babe::Trait for TestRuntime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = SameAuthoritiesForever;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1500;
}

impl pallet_timestamp::Trait for TestRuntime {
	type Moment = u64;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
}

pub type System = system::Module<TestRuntime>;
pub type Randomness = Module<TestRuntime>;
pub type CollectiveFlip = pallet_randomness_collective_flip::Module<TestRuntime>;
pub type Babe = pallet_babe::Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> sp_io::TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = sp_io::TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn flip_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(Randomness::call_collective_flip(Origin::signed(1)));

		// Check for the event
		assert_eq!(
			System::events(),
			vec![EventRecord {
				phase: Phase::Initialization,
				event: TestEvent::randomness(Event::CollectiveFlip(H256::zero(), H256::zero(),)),
				topics: vec![],
			}]
		);
	})
}

#[test]
fn babe_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(Randomness::call_babe_vrf(Origin::signed(1)));

		// Check for the event
		assert_eq!(
			System::events(),
			vec![EventRecord {
				phase: Phase::Initialization,
				event: TestEvent::randomness(Event::BabeVRF(
					// Let's see how deterministic these are...
					H256::from_slice(&[
						0x89, 0xeb, 0x0d, 0x6a, 0x8a, 0x69, 0x1d, 0xae, 0x2c, 0xd1, 0x5e, 0xd0,
						0x36, 0x99, 0x31, 0xce, 0x0a, 0x94, 0x9e, 0xca, 0xfa, 0x5c, 0x3f, 0x93,
						0xf8, 0x12, 0x18, 0x33, 0x64, 0x6e, 0x15, 0xc3
					]),
					H256::from_slice(&[
						0x9f, 0x0e, 0x44, 0x4c, 0x69, 0xf7, 0x7a, 0x49, 0xbd, 0x0b, 0xe8, 0x9d,
						0xb9, 0x2c, 0x38, 0xfe, 0x71, 0x3e, 0x09, 0x63, 0x16, 0x5c, 0xca, 0x12,
						0xfa, 0xf5, 0x71, 0x2d, 0x76, 0x57, 0x12, 0x0f
					]),
				)),
				topics: vec![],
			}]
		);
	})
}
