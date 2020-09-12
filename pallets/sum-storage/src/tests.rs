use super::*;

use frame_support::{assert_ok, impl_outer_origin, parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
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
impl Trait for Test {
	type Event = ();
}
type TemplateModule = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.expect("test text")
		.into()
}

#[test]
fn default_sum_zero() {
	new_test_ext().execute_with(|| {
		assert_eq!(TemplateModule::get_sum(), 0);
	});
}

#[test]
fn sums_thing_one() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_thing_1(Origin::signed(1), 42));
		assert_eq!(TemplateModule::get_sum(), 42);
	});
}

#[test]
fn sums_thing_two() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_thing_2(Origin::signed(1), 42));
		assert_eq!(TemplateModule::get_sum(), 42);
	});
}

#[test]
fn sums_both_values() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_thing_1(Origin::signed(1), 42));
		assert_ok!(TemplateModule::set_thing_2(Origin::signed(1), 43));
		assert_eq!(TemplateModule::get_sum(), 85);
	});
}
