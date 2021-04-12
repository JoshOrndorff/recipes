use crate::{self as double_map, Config, GroupMembership, MemberScore, RawEvent};
use frame_support::{
	assert_noop, assert_ok, construct_runtime, parameter_types,
	storage::{StorageDoubleMap, StorageMap},
};
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
		DoubleMap: double_map::{Module, Call, Storage, Event<T>},
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
fn join_all_members_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		// correct panic upon existing member trying to join
		assert_noop!(
			DoubleMap::join_all_members(Origin::signed(1)),
			"already a member, can't join"
		);

		// correct event emission
		let expected_event = Event::double_map(RawEvent::NewMember(1));

		assert_eq!(System::events()[0].event, expected_event,);
		// correct storage changes
		assert_eq!(DoubleMap::all_members(), vec![1]);
	})
}

#[test]
fn group_join_works() {
	ExternalityBuilder::build().execute_with(|| {
		// expected panic
		assert_noop!(
			DoubleMap::join_a_group(Origin::signed(1), 3, 5),
			"not a member, can't remove"
		);

		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		assert_ok!(DoubleMap::join_a_group(Origin::signed(1), 3, 5));

		// correct event emission
		let expected_event = Event::double_map(RawEvent::MemberJoinsGroup(1, 3, 5));

		assert_eq!(System::events()[1].event, expected_event,);

		// correct storage changes
		assert_eq!(DoubleMap::group_membership(1), 3);
		assert_eq!(DoubleMap::member_score(3, 1), 5);
	})
}

#[test]
fn remove_member_works() {
	ExternalityBuilder::build().execute_with(|| {
		// action: user 1 joins
		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		// action: user 1 joins group 3 with score 5
		assert_ok!(DoubleMap::join_a_group(Origin::signed(1), 3, 5));
		// action: remove user 1
		assert_ok!(DoubleMap::remove_member(Origin::signed(1)));

		// check: correct event emitted
		let expected_event = Event::double_map(RawEvent::RemoveMember(1));

		assert_eq!(System::events()[2].event, expected_event,);

		// check: user 1 should no longer belongs to group 3
		assert!(!<GroupMembership<TestRuntime>>::contains_key(1));
		assert!(!<MemberScore<TestRuntime>>::contains_key(3, 1));
	})
}

#[test]
fn remove_group_score_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		assert_ok!(DoubleMap::join_all_members(Origin::signed(2)));
		assert_ok!(DoubleMap::join_all_members(Origin::signed(3)));
		assert_ok!(DoubleMap::join_a_group(Origin::signed(1), 3, 5));
		assert_ok!(DoubleMap::join_a_group(Origin::signed(2), 3, 5));
		assert_ok!(DoubleMap::join_a_group(Origin::signed(3), 3, 5));

		assert_noop!(
			DoubleMap::remove_group_score(Origin::signed(4), 3),
			"member isn't in the group, can't remove it"
		);

		assert_noop!(
			DoubleMap::remove_group_score(Origin::signed(1), 2),
			"member isn't in the group, can't remove it"
		);

		assert_ok!(DoubleMap::remove_group_score(Origin::signed(1), 3));

		// correct event emitted
		let expected_event = Event::double_map(RawEvent::RemoveGroup(3));

		assert_eq!(System::events()[6].event, expected_event,);

		// check: user 1, 2, 3 should no longer in the group
		assert!(!<MemberScore<TestRuntime>>::contains_key(3, 1));
		assert!(!<MemberScore<TestRuntime>>::contains_key(3, 2));
		assert!(!<MemberScore<TestRuntime>>::contains_key(3, 3));
	})
}
