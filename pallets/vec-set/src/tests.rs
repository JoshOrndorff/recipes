use crate::{self as vec_set, Config, Error, RawEvent};
use frame_support::{assert_noop, assert_ok, construct_runtime, parameter_types};
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
		VecSet: vec_set::{Module, Call, Storage, Event<T>},
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
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn add_member_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));

		let expected_event = Event::vec_set(RawEvent::MemberAdded(1));

		assert_eq!(System::events()[0].event, expected_event,);

		assert_eq!(VecSet::members(), vec![1]);
	})
}

#[test]
fn cant_add_duplicate_members() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));

		assert_noop!(
			VecSet::add_member(Origin::signed(1)),
			Error::<TestRuntime>::AlreadyMember
		);
	})
}

#[test]
fn cant_exceed_max_members() {
	ExternalityBuilder::build().execute_with(|| {
		// Add 16 members, reaching the max
		for i in 0..16 {
			assert_ok!(VecSet::add_member(Origin::signed(i)));
		}

		// Try to add the 17th member exceeding the max
		assert_noop!(
			VecSet::add_member(Origin::signed(16)),
			Error::<TestRuntime>::MembershipLimitReached
		);
	})
}

#[test]
fn remove_member_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));
		assert_ok!(VecSet::remove_member(Origin::signed(1)));

		// check correct event emission
		let expected_event = Event::vec_set(RawEvent::MemberRemoved(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		// check storage changes
		assert_eq!(VecSet::members(), Vec::<u64>::new());
	})
}

#[test]
fn remove_member_handles_errors() {
	ExternalityBuilder::build().execute_with(|| {
		// 2 is NOT previously added as a member
		assert_noop!(
			VecSet::remove_member(Origin::signed(2)),
			Error::<TestRuntime>::NotMember
		);
	})
}
