use super::Event;
use crate::{Module, Trait};
use frame_support::{
	assert_err, assert_ok, impl_outer_event, impl_outer_origin, parameter_types, traits::OnFinalize,
};
use frame_system as system;
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
impl system::Trait for TestRuntime {
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
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

mod constant_config {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		constant_config,
		system<T>,
	}
}

parameter_types! {
	pub const MaxAddend: u32 = 100;
	pub const ClearFrequency: u64 = 10;
}
impl Trait for TestRuntime {
	type Event = TestEvent;
	type MaxAddend = MaxAddend;
	type ClearFrequency = ClearFrequency;
}

pub type System = system::Module<TestRuntime>;
pub type ConstantConfig = Module<TestRuntime>;

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.expect("test text");
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
		.into_iter().map(|r| r.event)
		.filter_map(|e| {
			if let TestEvent::constant_config(inner) = e { Some(inner) } else { None }
		})
		.collect::<Vec<_>>();

		let expected_events = vec![
			Event::Added(10, 100, 110),
			Event::Added(110, 100, 210),
			Event::Added(210, 100, 310),
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
			let expected_event = TestEvent::constant_config(Event::Cleared(110));
			
			assert_eq!(
				System::events()[1].event,
				expected_event,
			);
		
			assert_eq!(ConstantConfig::single_value(), 0);
	})
}
