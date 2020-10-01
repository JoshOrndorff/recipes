use super::Event;
use crate::{Module, Trait};
use frame_support::{
	assert_ok, impl_outer_event, impl_outer_origin, parameter_types, traits::OnFinalize,
};
use sp_core::H256;
use sp_io::TestExternalities;
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
impl frame_system::Trait for TestRuntime {
	type BaseCallFilter = ();
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
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

mod fixed_point {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		fixed_point,
		frame_system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = frame_system::Module<TestRuntime>;
pub type FixedPoint = Module<TestRuntime>;

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
fn deposit_withdraw_discrete_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Deposit 10 tokens
		assert_ok!(FixedPoint::deposit_discrete(Origin::signed(1), 10));

		// Withdraw 5 tokens
		assert_ok!(FixedPoint::withdraw_discrete(Origin::signed(1), 5));

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter().map(|r| r.event)
			.filter_map(|e| {
				if let TestEvent::fixed_point(inner) = e { Some(inner) } else { None }
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			Event::DepositedDiscrete(10,),
			Event::WithdrewDiscrete(5,),
		];

		assert_eq!(our_events, expected_events);

		// Check that five tokens are still there
		assert_eq!(FixedPoint::discrete_account(), 5);
	})
}

#[test]
fn discrete_interest_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Deposit 100 tokens
		assert_ok!(FixedPoint::deposit_discrete(Origin::signed(1), 100));

		// balance should not change after the 3rd block
		FixedPoint::on_finalize(3);
		assert_eq!(FixedPoint::discrete_account(), 100);

		// on_finalize should compute interest on 10th block
		FixedPoint::on_finalize(10);

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter().map(|r| r.event)
			.filter_map(|e| {
				if let TestEvent::fixed_point(inner) = e { Some(inner) } else { None }
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			Event::DepositedDiscrete(100,),
			Event::DiscreteInterestApplied(50,),
		];

		assert_eq!(our_events, expected_events);

		// Check that the balance has updated
		assert_eq!(FixedPoint::discrete_account(), 150);
	})
}
