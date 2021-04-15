use crate::tight::{self as check_membership, Config, Error, RawEvent};
use frame_support::{assert_noop, assert_ok, construct_runtime, parameter_types};
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
		CheckMembership: check_membership::{Module, Call, Event<T>},
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

impl vec_set::Config for TestRuntime {
	type Event = Event;
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
fn members_can_call() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(VecSet::add_member(Origin::signed(1)));

		assert_ok!(CheckMembership::check_membership(Origin::signed(1)));

		let expected_event = Event::check_membership(RawEvent::IsAMember(1));

		assert_eq!(System::events()[1].event, expected_event,);
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
