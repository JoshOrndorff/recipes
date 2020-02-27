use crate::*;
use primitives::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, AtLeast32Bit},
	Perbill,
};
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};

// hacky Eq implementation for testing InnerThing
impl<Hash: Clone, Balance: Copy + AtLeast32Bit> PartialEq for InnerThing<Hash, Balance> {
	fn eq(&self, other: &Self) -> bool {
		self.number == other.number
	}
}
impl<Hash: Clone, Balance: Copy + AtLeast32Bit> Eq for InnerThing<Hash, Balance> {}
// "" for SuperThing
impl<Hash: Clone, Balance: Copy + AtLeast32Bit> PartialEq for SuperThing<Hash, Balance> {
	fn eq(&self, other: &Self) -> bool {
		self.super_number == other.super_number
	}
}
impl<Hash: Clone, Balance: Copy + AtLeast32Bit> Eq for SuperThing<Hash, Balance> {}

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
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
}
// note: very unrealistic for most test envs
parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
	pub const TransferFee: u64 = 0;
	pub const CreationFee: u64 = 0;
}
impl balances::Trait for TestRuntime {
	type Balance = u64;
	type Event = TestEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Module<TestRuntime>;
}

mod struct_storage {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		struct_storage<T>,
		system<T>,
		balances<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type StructStorage = Module<TestRuntime>;

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
fn insert_inner_works() {
	ExtBuilder::build().execute_with(|| {
		// prepare hash
		let data = H256::from_low_u64_be(16);
		// insert inner thing
		assert_ok!(StructStorage::insert_inner_thing(Origin::signed(1), 3u32, data, 7u64.into()));

		// check storage matches expectations
		let expected_storage_item = InnerThing {
			number: 3u32,
			hash: data,
			balance: 7u64,
		};
		assert_eq!(
			StructStorage::inner_things_by_numbers(3u32),
			expected_storage_item
		);

		// check events emitted match expectations
		let expected_event =
			TestEvent::struct_storage(RawEvent::NewInnerThing(3u32, data, 7u64));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn insert_super_thing_with_existing_works() {
	ExtBuilder::build().execute_with(|| {
		// prepare hash
		let data = H256::from_low_u64_be(16);
		// insert inner first (tested in direct test above)
		assert_ok!(StructStorage::insert_inner_thing(Origin::signed(1), 3u32, data, 7u64.into()));
		// insert super with existing inner
		assert_ok!(StructStorage::insert_super_thing_with_existing_inner(Origin::signed(1), 3u32, 5u32));

		// check storage matches expectations
		let expected_inner = InnerThing {
			number: 3u32,
			hash: data,
			balance: 7u64,
		};
		assert_eq!(StructStorage::inner_things_by_numbers(3u32), expected_inner);
		let expected_outer = SuperThing {
			super_number: 5u32,
			inner_thing: expected_inner.clone(),
		};
		assert_eq!(
			StructStorage::super_things_by_super_numbers(5u32),
			expected_outer
		);

		let expected_event = TestEvent::struct_storage(RawEvent::NewSuperThingByExistingInner(
			5u32,
			3u32,
			data,
			7u64.into(),
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn insert_super_with_new_inner_works() {
	ExtBuilder::build().execute_with(|| {
		// prepare hash
		let data = H256::from_low_u64_be(16);
		// insert super with new inner
		assert_ok!(StructStorage::insert_super_thing_with_new_inner(
			Origin::signed(1),
			3u32,
			data,
			7u64.into(),
			5u32,
		));

		// check storage matches expectations
		let expected_inner = InnerThing {
			number: 3u32,
			hash: data,
			balance: 7u64,
		};
		assert_eq!(StructStorage::inner_things_by_numbers(3u32), expected_inner);
		let expected_outer = SuperThing {
			super_number: 5u32,
			inner_thing: expected_inner.clone(),
		};
		assert_eq!(
			StructStorage::super_things_by_super_numbers(5u32),
			expected_outer
		);

		let expected_event =
			TestEvent::struct_storage(RawEvent::NewInnerThing(3u32, data, 7u64));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		let expected_event2 = TestEvent::struct_storage(RawEvent::NewSuperThingByNewInner(
			5u32,
			3u32,
			data,
			7u64.into(),
		));
		assert!(System::events().iter().any(|a| a.event == expected_event2));
	})
}
