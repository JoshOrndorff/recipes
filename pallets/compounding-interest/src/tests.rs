use super::Event;
use crate::{Module, Trait};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, OnFinalize},
	Perbill,
};
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system::{self as system, EventRecord, Phase};

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
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

mod fixed_point {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		fixed_point,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type FixedPoint = Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		TestExternalities::from(storage)
	}
}

#[test]
fn deposit_withdraw_discrete_works() {
	ExtBuilder::build().execute_with(|| {
		// Deposit 10 tokens
		assert_ok!(FixedPoint::deposit_discrete(Origin::signed(1), 10));

		// Withdraw 5 tokens
		assert_ok!(FixedPoint::withdraw_discrete(Origin::signed(1), 5));

		// Check for the correct event
		assert_eq!(System::events(), vec![
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::fixed_point(Event::DepositedDiscrete(
					10,
				)),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::fixed_point(Event::WithdrewDiscrete(
					5,
				)),
				topics: vec![],
			},
		]);

		// Check that five tokens are still there
		assert_eq!(FixedPoint::discrete_account(), 5);
	})
}

#[test]
fn discrete_interest_works() {
	ExtBuilder::build().execute_with(|| {
		// Deposit 100 tokens
		assert_ok!(FixedPoint::deposit_discrete(Origin::signed(1), 100));

		// balance should not change after the 3rd block
		FixedPoint::on_finalize(3);
		assert_eq!(FixedPoint::discrete_account(), 100);

		// on_finalize should compute interest on 10th block
		FixedPoint::on_finalize(10);

		// Check for the correct event
		assert_eq!(System::events(), vec![
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::fixed_point(Event::DepositedDiscrete(
					100,
				)),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::fixed_point(Event::DiscreteInterestApplied(
					50,
				)),
				topics: vec![],
			},
		]);

		// Check that the balance has updated
		assert_eq!(FixedPoint::discrete_account(), 150);
	})
}
