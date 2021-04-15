use crate::{self as fixed_point, Config, Error, Event as PalletEvent};
use frame_support::{assert_noop, assert_ok, construct_runtime, parameter_types};
use sp_arithmetic::Permill;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use substrate_fixed::types::U16F16;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		FixedPoint: fixed_point::{Module, Call, Event},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Index = u64;
	type Call = Call;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl Config for TestRuntime {
	type Event = Event;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn all_accumulators_start_at_one() {
	ExternalityBuilder::build().execute_with(|| {
		assert_eq!(FixedPoint::manual_value(), 1 << 16);
		assert_eq!(FixedPoint::permill_value(), Permill::one());
		assert_eq!(FixedPoint::fixed_value(), 1);
	})
}

#[test]
fn manual_impl_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Setup some constants
		let one: u32 = 1 << 16;
		let half: u32 = one / 2;
		let quarter: u32 = half / 2;

		// Multiply by half
		assert_ok!(FixedPoint::update_manual(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::manual_value(), half);

		// Multiply by half again
		assert_ok!(FixedPoint::update_manual(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::manual_value(), quarter);

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::fixed_point(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			PalletEvent::ManualUpdated(half, half),
			PalletEvent::ManualUpdated(half, quarter),
		];

		assert_eq!(our_events, expected_events);
	})
}

#[test]
fn manual_impl_overflows() {
	ExternalityBuilder::build().execute_with(|| {
		// Although 2^17 is able to fit in a u32, we're using our u32s in a weird way where
		// only the first 16 bits represent integer positions, and the remaining 16 bits
		// represent fractional positions. 2^17 cannot fit in the 16 available integer
		// positions, thus we expect this to overflow.

		// Setup some constants
		let one: u32 = 1 << 16;

		// Multiply by 2 ^ 10
		assert_ok!(FixedPoint::update_manual(Origin::signed(1), one << 10));

		// Multiple by an additional 2 ^  7 which should cause the overflow
		assert_noop!(
			FixedPoint::update_manual(Origin::signed(1), one << 7),
			Error::<TestRuntime>::Overflow
		);
	})
}

#[test]
fn permill_impl_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Setup some constants
		let half = Permill::from_percent(50);
		let quarter = Permill::from_percent(25);

		// Multiply by half
		assert_ok!(FixedPoint::update_permill(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::permill_value(), half);

		// Multiply by half again
		assert_ok!(FixedPoint::update_permill(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::permill_value(), quarter);

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::fixed_point(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			PalletEvent::PermillUpdated(half, half),
			PalletEvent::PermillUpdated(half, quarter),
		];

		assert_eq!(our_events, expected_events);
	})
}

// Permill can only hold values in the range [0, 1] so it is impossible to overflow.
// #[test]
// fn manual_impl_overflows() {}

#[test]
fn fixed_impl_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Setup some constants
		let one = U16F16::from_num(1);
		let half = one / 2;
		let quarter = half / 2;

		// Multiply by half
		assert_ok!(FixedPoint::update_fixed(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::fixed_value(), half);

		// Multiply by half again
		assert_ok!(FixedPoint::update_fixed(Origin::signed(1), half));

		// Ensure the new value is correct
		assert_eq!(FixedPoint::fixed_value(), quarter);

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::fixed_point(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			PalletEvent::FixedUpdated(half, half),
			PalletEvent::FixedUpdated(half, quarter),
		];

		assert_eq!(our_events, expected_events);
	})
}

#[test]
fn fixed_impl_overflows() {
	ExternalityBuilder::build().execute_with(|| {
		// U16F16 has 16 bits of integer storage, so just like with our manual
		// implementation, a value of 2 ^ 17 will cause overflow.

		// Multiply by 2 ^ 10
		assert_ok!(FixedPoint::update_fixed(
			Origin::signed(1),
			U16F16::from_num(1 << 10)
		));

		// Multiple by an additional 2 ^  7 which should cause the overflow
		assert_noop!(
			FixedPoint::update_fixed(Origin::signed(1), U16F16::from_num(1 << 7)),
			Error::<TestRuntime>::Overflow
		);
	})
}
