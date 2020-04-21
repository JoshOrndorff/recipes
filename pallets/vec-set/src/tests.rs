use crate::{Module, RawEvent, Trait};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use frame_support::{assert_ok, assert_err, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system as system;

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

mod vec_set {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		vec_set<T>,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type VecSet = Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
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
fn add_member_err_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));

		assert_err!(
			VecSet::add_member(Origin::signed(1)),
			"must not be a member to be added"
		);
	})
}

#[test]
fn add_member_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));

		let expected_event = TestEvent::vec_set(RawEvent::MemberAdded(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		assert_eq!(VecSet::members(), vec![1]);
	})
}

#[test]
fn remove_member_err_works() {
	ExtBuilder::build().execute_with(|| {
		// 2 is NOT previously added as a member
		assert_err!(
			VecSet::remove_member(Origin::signed(2)),
			"must be a member in order to leave"
		);
	})
}

#[test]
fn remove_member_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));
		assert_ok!(VecSet::remove_member(Origin::signed(1)));
		assert_ok!(VecSet::add_member(Origin::signed(2)));

		// check correct event emission
		let expected_event = TestEvent::vec_set(RawEvent::MemberRemoved(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		// check storage changes
		assert_eq!(VecSet::members(), vec![2]);
	})
}
