use super::Event;
use crate::{Module, Trait, Error};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use sp_arithmetic::Permill;
use frame_support::{assert_ok, assert_noop, impl_outer_event, impl_outer_origin, parameter_types};
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
fn all_accumulators_start_at_one() {
	ExtBuilder::build().execute_with(|| {
		assert_eq!(FixedPoint::manual_value(), 1 << 16);
		assert_eq!(FixedPoint::permill_value(), Permill::one());
		assert_eq!(FixedPoint::fixed_value(), 1);
	})
}

#[test]
fn manual_impl_works() {
	ExtBuilder::build().execute_with(|| {
		// Setup some constants
		let one : u32 = 1 << 16;
		let half : u32 = one / 2;
		let quarter : u32 = half / 2;

		// Multiply by half
		assert_ok!(FixedPoint::update_manual(Origin::signed(1), half));

		// Ensure the new value is correct
		println!("Half is {}", half);
		assert_eq!(FixedPoint::manual_value(), half);

		// Multiply by half again
		assert_ok!(FixedPoint::update_manual(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::manual_value(), quarter);

		// Check for the correct events
		assert_eq!(System::events(), vec![
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::fixed_point(Event::ManualUpdated(
					half,
					half,
				)),
				topics: vec![],
			},
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::fixed_point(Event::ManualUpdated(
					half,
					quarter,
				)),
				topics: vec![],
			},
		]);
	})
}

#[test]
fn manual_impl_overflows() {
	ExtBuilder::build().execute_with(|| {

		// Although 2^17 is able to fit in a u32, we're using our u32s in a weird way where
		// only the first 16 bits represent integer positions, and the remaining 16 bits
		// represent fractional positions. 2^17 cannot fit in the 16 available integer
		// positions, thus we expect this to overflow.

		// Setup some constants
		let one : u32 = 1 << 16;

		// Multiply by 2 ^ 10
		assert_ok!(FixedPoint::update_manual(Origin::signed(1), one << 10));

		// Multiple by an additional 2 ^  7 which should cause the overflow
		assert_noop!(
			FixedPoint::update_manual(Origin::signed(1), one << 7),
			Error::<TestRuntime>::Overflow
		);
	})
}
