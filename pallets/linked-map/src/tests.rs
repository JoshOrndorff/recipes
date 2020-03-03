use super::RawEvent;
use crate::{Module, Trait};
use primitives::H256;
use runtime_io;
use runtime_primitives::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use support::{assert_ok, assert_err, impl_outer_event, impl_outer_origin, parameter_types};
use system;

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

mod linked_map {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		linked_map<T>,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type LinkedMap = Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> runtime_io::TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		runtime_io::TestExternalities::from(storage)
	}
}

#[test]
fn add_member_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(LinkedMap::add_member(Origin::signed(1)));

		let expected_event =
			TestEvent::linked_map(RawEvent::MemberAdded(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		let counter = LinkedMap::the_counter();
		assert_eq!(counter, 1);
		assert_eq!(LinkedMap::the_list(counter), 1);
		let lcounter = LinkedMap::the_counter();
		assert_eq!(lcounter, 1);
		assert_eq!(LinkedMap::linked_list(lcounter), 1);

		assert_ok!(LinkedMap::add_member(Origin::signed(2)));

		let counter2 = LinkedMap::the_counter();
		assert_eq!(counter2, 2);
		assert_eq!(LinkedMap::the_list(counter2), 2);
		let lcounter2 = LinkedMap::the_counter();
		assert_eq!(lcounter2, 2);
		assert_eq!(LinkedMap::linked_list(lcounter2), 2);
	})
}

#[test]
fn remove_works() {
	ExtBuilder::build().execute_with(|| {
		assert_err!(
			LinkedMap::remove_member_unbounded(Origin::signed(1), 1),
			"an element doesn't exist at this index"
		);
		assert_ok!(LinkedMap::add_member(Origin::signed(1)));

		let expected_event =
			TestEvent::linked_map(RawEvent::MemberAdded(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		// check event is emitted
		let counter = LinkedMap::the_counter();
		assert_eq!(counter, 1);

		// remove unbounded doesn't decrement counter
		assert_ok!(LinkedMap::remove_member_unbounded(Origin::signed(1), 1));
		let expected_event =
			TestEvent::linked_map(RawEvent::MemberRemoved(1));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		let counter2 = LinkedMap::the_counter();
		// the counter doesn't decrement because the list was unbounded (counter always increases)
		assert_eq!(counter2, 1);

		// add a new member
		assert_ok!(LinkedMap::add_member(Origin::signed(2))); // note: counter increments

		// remove bounded decrements counter
		assert_ok!(LinkedMap::remove_member_bounded(Origin::signed(1), 2));
		let expected_event2 =
			TestEvent::linked_map(RawEvent::MemberRemoved(2));
		assert!(System::events().iter().any(|a| a.event == expected_event2));
		let counter2 = LinkedMap::the_counter();
		// counter decrements (from 2 to 1)
		assert_eq!(counter2, 1);

		assert_ok!(LinkedMap::remove_member_linked(Origin::signed(1), 1));
		let expected_event3 =
			TestEvent::linked_map(RawEvent::MemberRemoved(1));
		assert!(System::events().iter().any(|a| a.event == expected_event3));
		// no required counter for linked map
	})
}
