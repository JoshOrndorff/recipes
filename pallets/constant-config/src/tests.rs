use crate::{self as constant_config, Config, Event as PalletEvent};
use frame_support::{
	assert_err, assert_ok, construct_runtime, parameter_types, traits::OnFinalize,
};
use frame_system as system;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		ConstantConfig: constant_config::{Module, Call, Storage, Event},
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

parameter_types! {
	pub const MaxAddend: u32 = 100;
	pub const ClearFrequency: u64 = 10;
}
impl Config for TestRuntime {
	type Event = Event;
	type MaxAddend = MaxAddend;
	type ClearFrequency = ClearFrequency;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn max_added_exceeded_errs() {
	ExternalityBuilder::build().execute_with(|| {
		assert_err!(
			ConstantConfig::add_value(Origin::signed(1), 101),
			"value must be <= maximum add amount constant"
		);
	})
}

#[test]
fn overflow_checked() {
	ExternalityBuilder::build().execute_with(|| {
		let test_num: u32 = u32::max_value() - 99;
		assert_ok!(ConstantConfig::set_value(Origin::signed(1), test_num));

		assert_err!(
			ConstantConfig::add_value(Origin::signed(1), 100),
			"Addition overflowed"
		);
	})
}

#[test]
fn add_value_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(ConstantConfig::set_value(Origin::signed(1), 10));

		assert_ok!(ConstantConfig::add_value(Origin::signed(2), 100));

		assert_ok!(ConstantConfig::add_value(Origin::signed(3), 100));

		assert_ok!(ConstantConfig::add_value(Origin::signed(4), 100));

		//Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::constant_config(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			PalletEvent::Added(10, 100, 110),
			PalletEvent::Added(110, 100, 210),
			PalletEvent::Added(210, 100, 310),
		];

		assert_eq!(our_events, expected_events);
	})
}

#[test]
fn on_finalize_clears() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(5);
		assert_ok!(ConstantConfig::set_value(Origin::signed(1), 10));

		assert_ok!(ConstantConfig::add_value(Origin::signed(2), 100));

		ConstantConfig::on_finalize(10);
		let expected_event = Event::constant_config(PalletEvent::Cleared(110));

		assert_eq!(System::events()[1].event, expected_event,);

		assert_eq!(ConstantConfig::single_value(), 0);
	})
}
