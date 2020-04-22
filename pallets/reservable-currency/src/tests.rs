use crate::*;
use balances;
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system::{self as system};
use sp_core::H256;
use sp_io;
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

    pub const ExistentialDeposit: u64 = 1;
    pub const TransferFee: u64 = 0;
    pub const CreationFee: u64 = 0;
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

impl balances::Trait for TestRuntime {
    type Balance = u64;
    type Event = TestEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = system::Module<TestRuntime>;
}

mod reservable_currency {
    pub use crate::Event;
}

impl_outer_event! {
    pub enum TestEvent for TestRuntime {
        system<T>,
        reservable_currency<T>,
        balances<T>,
    }
}

impl Trait for TestRuntime {
    type Event = TestEvent;
    type Currency = balances::Module<Self>;
}

pub type System = system::Module<TestRuntime>;
pub type Balances = balances::Module<TestRuntime>;
pub type ReservableCurrency = Module<TestRuntime>;

// An alternative to `ExtBuilder` which includes custom configuration
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    balances::GenesisConfig::<TestRuntime> {
        // Provide some initial balances
        balances: vec![(1, 10000), (2, 11000), (3, 12000), (4, 13000), (5, 14000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
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
        let expected_event = TestEvent::reservable_currency(RawEvent::LockFunds(1, 5000, 1));
        assert!(System::events().iter().any(|a| a.event == expected_event));
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
        let lock_event = TestEvent::reservable_currency(RawEvent::LockFunds(1, 5000, 1));
        assert!(System::events().iter().any(|a| a.event == lock_event));
        assert_eq!(Balances::free_balance(&1), 5000);

        // Unlock balance, test event, test free balance
        assert_ok!(ReservableCurrency::unreserve_funds(Origin::signed(1), 5000));
        let unlock_event = TestEvent::reservable_currency(RawEvent::UnlockFunds(1, 5000, 1));
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
        let transfer_event = TestEvent::reservable_currency(RawEvent::TransferFunds(1, 2, 4000, 1));
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
        let reserve_event = TestEvent::reservable_currency(RawEvent::LockFunds(1, 4000, 1));
        assert!(System::events().iter().any(|a| a.event == reserve_event));

        // Punish one, for a value of 6000 collateral. Because one only has 4000 collateral reserved
        // all of it is unreserved and transferred
        assert_ok!(ReservableCurrency::unreserve_and_transfer(
            Origin::signed(1),
            1,
            2,
            6000
        ));
        let transfer_event = TestEvent::reservable_currency(RawEvent::TransferFunds(1, 2, 4000, 1));
        assert!(System::events().iter().any(|a| a.event == transfer_event));
        //Test if reserved::(1, 0) -> test if (1, 8000) -> test if (2, 13000)
        assert_eq!(Balances::reserved_balance(&1), 0);
        assert_eq!(Balances::free_balance(&1), 6000);
        assert_eq!(Balances::free_balance(&2), 15000);
    })
}
