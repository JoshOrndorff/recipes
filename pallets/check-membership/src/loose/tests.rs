use crate::loose::*;
use frame_support::{assert_noop, assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
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
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

mod check_membership {
	pub use crate::loose::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		vec_set<T>,
		system<T>,
		check_membership<T>,
	}
}

impl vec_set::Trait for TestRuntime {
	type Event = TestEvent;
}

impl Trait for TestRuntime {
	type Event = TestEvent;
	type MembershipSource = VecSet;
}

pub type System = system::Module<TestRuntime>;
pub type VecSet = vec_set::Module<TestRuntime>;
pub type CheckMembership = Module<TestRuntime>;

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
fn members_can_call() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));

		assert_ok!(CheckMembership::check_membership(Origin::signed(1)));

		let expected_event = TestEvent::check_membership(RawEvent::IsAMember(1));

		assert_eq!(
			System::events()[1].event,
			expected_event,
		);
	})
}

#[test]
fn non_members_cant_call() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			CheckMembership::check_membership(Origin::signed(1)),
			Error::<TestRuntime>::NotAMember
		);
	})
}
