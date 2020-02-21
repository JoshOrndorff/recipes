use super::*;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use frame_support::{assert_ok, assert_noop, impl_outer_event, impl_outer_origin, parameter_types};
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
}

mod double_map {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		double_map<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type DoubleMap = Module<TestRuntime>;

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
fn join_all_members_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		// correct panic upon existing member trying to join
		assert_noop!(
			DoubleMap::join_all_members(Origin::signed(1)),
			"already a member, can't join"
		);

		// correct event emission
		let expected_event = TestEvent::double_map(RawEvent::NewMember(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		// correct storage changes
		assert_eq!(DoubleMap::all_members(), vec![1]);
	})
}

#[test]
fn group_join_works() {
	ExtBuilder::build().execute_with(|| {
		// expected panic
		assert_noop!(
			DoubleMap::join_a_group(Origin::signed(1), 3, 5),
			"not a member, can't remove"
		);

		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		assert_ok!(DoubleMap::join_a_group(Origin::signed(1), 3, 5));

		// correct event emission
		let expected_event =
			TestEvent::double_map(RawEvent::MemberJoinsGroup(1, 3, 5));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		// correct storage changes
		assert_eq!(DoubleMap::group_membership(1), 3);
		assert_eq!(DoubleMap::member_score(3, 1), 5);
	})
}

#[test]
fn remove_member_works() {
	ExtBuilder::build().execute_with(|| {
		// action: user 1 joins
		assert_ok!(DoubleMap::join_all_members(Origin::signed(1)));
		// action: user 1 joins group 3 with score 5
		assert_ok!(DoubleMap::join_a_group(Origin::signed(1), 3, 5));
		// action: remove user 1
		assert_ok!(DoubleMap::remove_member(Origin::signed(1)));

		// check: correct event emitted
		let expected_event =
			TestEvent::double_map(RawEvent::RemoveMember(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		// check: user 1 should no longer belongs to group 3
		assert!(!<GroupMembership<TestRuntime>>::exists(1));
		assert!(!<MemberScore<TestRuntime>>::exists(3, 1));
	})
}

#[test]
fn remove_group_score_works() {
	ExtBuilder::build().execute_with(|| {
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
		let expected_event = TestEvent::double_map(RawEvent::RemoveGroup(3));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		// check: user 1, 2, 3 should no longer in the group
		assert!(!<MemberScore<TestRuntime>>::exists(3, 1));
		assert!(!<MemberScore<TestRuntime>>::exists(3, 2));
		assert!(!<MemberScore<TestRuntime>>::exists(3, 3));
	})
}
