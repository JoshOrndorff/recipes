use crate::{self as basic_token, Config, Error};
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
		BasicToken: basic_token::{Module, Call, Storage, Event<T>},
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
	type Event = ();
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
	type Event = ();
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		TestExternalities::from(storage)
	}
}

#[test]
fn init_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(BasicToken::init(Origin::signed(1)));
		assert_eq!(BasicToken::get_balance(1), 21000000);
	})
}

#[test]
fn cant_double_init() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(BasicToken::init(Origin::signed(1)));
		assert_noop!(
			BasicToken::init(Origin::signed(1)),
			Error::<TestRuntime>::AlreadyInitialized
		);
	})
}

#[test]
fn transfer_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(BasicToken::init(Origin::signed(1)));

		// Transfer 100 tokens from user 1 to user 2
		assert_ok!(BasicToken::transfer(Origin::signed(1), 2, 100));

		assert_eq!(BasicToken::get_balance(1), 20999900);
		assert_eq!(BasicToken::get_balance(2), 100);
	})
}

#[test]
fn cant_spend_more_than_you_have() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(BasicToken::init(Origin::signed(1)));
		assert_noop!(
			BasicToken::transfer(Origin::signed(1), 2, 21000001),
			Error::<TestRuntime>::InsufficientFunds
		);
	})
}
