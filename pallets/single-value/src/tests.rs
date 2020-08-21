use crate::*;
use crate::{Module, Trait};
use frame_support::{
	assert_noop, assert_ok, dispatch::DispatchError, impl_outer_origin, parameter_types,
};
use frame_system::{self as system, RawOrigin};
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
	type Event = ();
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

impl Trait for TestRuntime {}

pub type SingleValue = Module<TestRuntime>;

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
fn set_value_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(SingleValue::set_value(Origin::signed(1), 10));

		assert_eq!(SingleValue::stored_value(), 10);
		// Another way of accessing the storage. This pattern is needed if it is a StorageMap
		assert_eq!(<StoredValue>::get(), 10);
	})
}

#[test]
fn set_value_no_root() {
	ExtBuilder::build().execute_with(|| {
		assert_noop!(
			SingleValue::set_value(RawOrigin::Root.into(), 10),
			DispatchError::BadOrigin
		);
	})
}

#[test]
fn set_account_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(SingleValue::set_account(Origin::signed(1)));

		assert_eq!(SingleValue::stored_account(), 1)
	})
}

#[test]
fn set_account_no_root() {
	ExtBuilder::build().execute_with(|| {
		assert_noop!(
			SingleValue::set_account(RawOrigin::Root.into()),
			DispatchError::BadOrigin
		);
	})
}
