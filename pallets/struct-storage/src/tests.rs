use crate::{self as struct_storage, Config, InnerThing, RawEvent, SuperThing};
use frame_support::{assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{AtLeast32Bit, BlakeTwo256, IdentityLookup},
};

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

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		StructStorage: struct_storage::{Module, Call, Storage, Event<T>},
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
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for TestRuntime {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
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
fn insert_inner_works() {
	ExternalityBuilder::build().execute_with(|| {
		// prepare hash
		let data = H256::from_low_u64_be(16);
		// insert inner thing
		assert_ok!(StructStorage::insert_inner_thing(
			Origin::signed(1),
			3u32,
			data,
			7u64.into()
		));

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
		let expected_event = Event::struct_storage(RawEvent::NewInnerThing(3u32, data, 7u64));

		assert_eq!(System::events()[0].event, expected_event,);
	})
}

#[test]
fn insert_super_thing_with_existing_works() {
	ExternalityBuilder::build().execute_with(|| {
		// prepare hash
		let data = H256::from_low_u64_be(16);
		// insert inner first (tested in direct test above)
		assert_ok!(StructStorage::insert_inner_thing(
			Origin::signed(1),
			3u32,
			data,
			7u64.into()
		));
		// insert super with existing inner
		assert_ok!(StructStorage::insert_super_thing_with_existing_inner(
			Origin::signed(1),
			3u32,
			5u32
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

		let expected_event = Event::struct_storage(RawEvent::NewSuperThingByExistingInner(
			5u32,
			3u32,
			data,
			7u64.into(),
		));

		assert_eq!(System::events()[1].event, expected_event,);
	})
}

#[test]
fn insert_super_with_new_inner_works() {
	ExternalityBuilder::build().execute_with(|| {
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

		//Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::struct_storage(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			RawEvent::NewInnerThing(3u32, data, 7u64),
			RawEvent::NewSuperThingByNewInner(5u32, 3u32, data, 7u64.into()),
		];

		assert_eq!(our_events, expected_events);
	})
}
