
use frame_system::{ self as system, RawOrigin };
use frame_support::{assert_ok, assert_noop, impl_outer_origin, parameter_types, dispatch::DispatchError };
use sp_runtime::{Perbill, traits::{IdentityLookup, BlakeTwo256}, testing::Header};
use sp_io::TestExternalities;
use sp_core::H256;
use crate::{Module, Trait};

impl_outer_origin!{
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
	type Event = ();
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

impl Trait for TestRuntime {}

pub type System = system::Module<TestRuntime>;
pub type HelloSubstrate = Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn say_hello_works() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(HelloSubstrate::say_hello(Origin::signed(1)));
	})
}

#[test]
fn say_hello_no_root() {
	ExtBuilder::build().execute_with(|| {
		assert_noop!(HelloSubstrate::say_hello(RawOrigin::Root.into()), DispatchError::BadOrigin);
	})
}
