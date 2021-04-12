use crate::{self as reservable_currency, Config, RawEvent};
use frame_support::{assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
use sp_io;
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
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		ReservableCurrency: reservable_currency::{Module, Call, Event<T>},
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
	type Balance = u64;
	type MaxLocks = ();
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl Config for TestRuntime {
	type Event = Event;
	type Currency = Balances;
}

// An alternative to `ExternalityBuilder` which includes custom configuration
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<TestRuntime>()
		.unwrap();
	pallet_balances::GenesisConfig::<TestRuntime> {
		// Provide some initial balances
		balances: vec![(1, 10000), (2, 11000), (3, 12000), (4, 13000), (5, 14000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext: sp_io::TestExternalities = t.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Verifying correct behavior of boilerplate
#[test]
fn new_test_ext_behaves() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::free_balance(&1), 10000);
	})
}

#[test]
fn new_test_ext_reserve_funds() {
	new_test_ext().execute_with(|| {
		// Lock half of 1's balance : (1, 10000) -> (1, 5000)
		assert_ok!(ReservableCurrency::reserve_funds(Origin::signed(1), 5000));
		// Test and see if we received a LockFunds event
		let expected_event = Event::reservable_currency(RawEvent::LockFunds(1, 5000, 1));

		assert_eq!(System::events()[1].event, expected_event,);
		// Test and see if (1, 5000) holds
		assert_eq!(Balances::free_balance(&1), 5000);
		// Make sure that our 5000 is actually reserved
		assert_eq!(Balances::reserved_balance(&1), 5000);
	})
}

#[test]
fn new_test_ext_unreserve_funds() {
	new_test_ext().execute_with(|| {
		// Lock balance, test lock event, test free balance
		assert_ok!(ReservableCurrency::reserve_funds(Origin::signed(1), 5000));
		let lock_event = Event::reservable_currency(RawEvent::LockFunds(1, 5000, 1));
		assert!(System::events().iter().any(|a| a.event == lock_event));
		assert_eq!(Balances::free_balance(&1), 5000);

		// Unlock balance, test event, test free balance
		assert_ok!(ReservableCurrency::unreserve_funds(Origin::signed(1), 5000));
		let unlock_event = Event::reservable_currency(RawEvent::UnlockFunds(1, 5000, 1));
		assert!(System::events().iter().any(|a| a.event == unlock_event));
		assert_eq!(Balances::free_balance(&1), 10000);
	})
}

#[test]
fn new_test_ext_transfer_funds() {
	new_test_ext().execute_with(|| {
		// Transfer 4000 funds -> check for TransferFunds event -> check for (1, 6000) / (2, 15000)
		assert_ok!(ReservableCurrency::transfer_funds(
			Origin::signed(1),
			2,
			4000
		));
		let transfer_event = Event::reservable_currency(RawEvent::TransferFunds(1, 2, 4000, 1));
		assert!(System::events().iter().any(|a| a.event == transfer_event));
		assert_eq!(Balances::free_balance(&1), 6000);
		assert_eq!(Balances::free_balance(&2), 15000);
	})
}

#[test]
fn new_test_ext_unreserve_and_transfer() {
	new_test_ext().execute_with(|| {
		// Reserve 4000 -> check for (1, 6000) -> check for reserved::(1, 4000)
		assert_ok!(ReservableCurrency::reserve_funds(Origin::signed(1), 4000));
		assert_eq!(Balances::free_balance(&1), 6000);
		assert_eq!(Balances::reserved_balance(&1), 4000);
		let reserve_event = Event::reservable_currency(RawEvent::LockFunds(1, 4000, 1));
		assert!(System::events().iter().any(|a| a.event == reserve_event));

		// Punish one, for a value of 6000 collateral. Because one only has 4000 collateral reserved
		// all of it is unreserved and transferred
		assert_ok!(ReservableCurrency::unreserve_and_transfer(
			Origin::signed(1),
			1,
			2,
			6000
		));
		let transfer_event = Event::reservable_currency(RawEvent::TransferFunds(1, 2, 4000, 1));
		assert!(System::events().iter().any(|a| a.event == transfer_event));
		//Test if reserved::(1, 0) -> test if (1, 8000) -> test if (2, 13000)
		assert_eq!(Balances::reserved_balance(&1), 0);
		assert_eq!(Balances::free_balance(&1), 6000);
		assert_eq!(Balances::free_balance(&2), 15000);
	})
}
