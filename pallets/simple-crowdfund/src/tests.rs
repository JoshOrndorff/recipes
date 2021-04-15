use crate::{self as simple_crowdfund, Config, Error, FundInfo, ModuleId};

use frame_support::{
	assert_noop, assert_ok, construct_runtime, parameter_types,
	traits::{OnFinalize, OnInitialize},
};
use sp_core::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Percent, Permill,
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
		Crowdfund: simple_crowdfund::{Module, Call, Storage, Event<T>},
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

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: u64 = 1;
	pub const SpendPeriod: u64 = 2;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: u64 = 1;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: u64 = 1;
	pub const TipReportDepositPerByte: u64 = 1;
	pub const TreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
}

parameter_types! {
	pub const SubmissionDeposit: u64 = 1;
	pub const MinContribution: u64 = 10;
	pub const RetirementPeriod: u64 = 5;
}
impl Config for TestRuntime {
	type Event = Event;
	type Currency = Balances;
	type SubmissionDeposit = SubmissionDeposit;
	type MinContribution = MinContribution;
	type RetirementPeriod = RetirementPeriod;
}

use pallet_balances::Error as BalancesError;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<TestRuntime>()
		.unwrap();
	pallet_balances::GenesisConfig::<TestRuntime> {
		balances: vec![(1, 1000), (2, 2000), (3, 3000), (4, 4000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		Crowdfund::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		Crowdfund::on_initialize(System::block_number());
	}
}

#[test]
fn basic_setup_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(System::block_number(), 0);
		assert_eq!(Crowdfund::fund_count(), 0);
		assert_eq!(Crowdfund::funds(0), None);
		assert_eq!(Crowdfund::contribution_get(0, &1), 0);
	});
}

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		// Now try to create a crowdfund campaign
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		assert_eq!(Crowdfund::fund_count(), 1);
		// This is what the initial `fund_info` should look like
		let fund_info = FundInfo {
			beneficiary: 2,
			deposit: 1,
			raised: 0,
			// 5 blocks length + 3 block ending period + 1 starting block
			end: 9,
			goal: 1000,
		};
		assert_eq!(Crowdfund::funds(0), Some(fund_info));
		// User has deposit removed from their free balance
		assert_eq!(Balances::free_balance(1), 999);
		// Deposit is placed in crowdfund free balance
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 1);
	});
}

#[test]
fn create_handles_insufficient_balance() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Crowdfund::create(Origin::signed(1337), 2, 1000, 9),
			BalancesError::<TestRuntime, _>::InsufficientBalance
		);
	});
}

#[test]
fn contribute_works() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		assert_eq!(Balances::free_balance(1), 999);
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 1);

		// No contributions yet
		assert_eq!(Crowdfund::contribution_get(0, &1), 0);

		// User 1 contributes to their own crowdfund
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 49));
		// User 1 has spent some funds to do this, transfer fees **are** taken
		assert_eq!(Balances::free_balance(1), 950);
		// Contributions are stored in the trie
		assert_eq!(Crowdfund::contribution_get(0, &1), 49);
		// Contributions appear in free balance of crowdfund
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 50);
		// Last contribution time recorded
		assert_eq!(Crowdfund::funds(0).unwrap().raised, 49);
	});
}

#[test]
fn contribute_handles_basic_errors() {
	new_test_ext().execute_with(|| {
		// Cannot contribute to non-existing fund
		assert_noop!(
			Crowdfund::contribute(Origin::signed(1), 0, 49),
			Error::<TestRuntime>::InvalidIndex
		);
		// Cannot contribute below minimum contribution
		assert_noop!(
			Crowdfund::contribute(Origin::signed(1), 0, 9),
			Error::<TestRuntime>::ContributionTooSmall
		);

		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 101));

		// Move past end date
		run_to_block(10);

		// Cannot contribute to ended fund
		assert_noop!(
			Crowdfund::contribute(Origin::signed(1), 0, 49),
			Error::<TestRuntime>::ContributionPeriodOver
		);
	});
}

#[test]
fn withdraw_works() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		// Transfer fees are taken here
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));

		// Skip all the way to the end
		// Crowdfund is unsuccessful 100 + 200 + 300 < 1000
		run_to_block(50);

		// User can withdraw their full balance without fees
		assert_ok!(Crowdfund::withdraw(Origin::signed(1), 0));
		assert_eq!(Balances::free_balance(1), 999);

		assert_ok!(Crowdfund::withdraw(Origin::signed(2), 0));
		assert_eq!(Balances::free_balance(2), 2000);

		assert_ok!(Crowdfund::withdraw(Origin::signed(3), 0));
		assert_eq!(Balances::free_balance(3), 3000);
	});
}

#[test]
fn withdraw_handles_basic_errors() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		// Transfer fee is taken here
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 49));
		assert_eq!(Balances::free_balance(1), 950);

		run_to_block(5);

		// Cannot withdraw before fund ends
		assert_noop!(
			Crowdfund::withdraw(Origin::signed(1), 0),
			Error::<TestRuntime>::FundStillActive
		);

		// Skip to the retirement period
		// Crowdfund is unsuccessful 100 + 200 + 300 < 1000
		run_to_block(10);

		// Cannot withdraw if they did not contribute
		assert_noop!(
			Crowdfund::withdraw(Origin::signed(2), 0),
			Error::<TestRuntime>::NoContribution
		);
		// Cannot withdraw from a non-existent fund
		assert_noop!(
			Crowdfund::withdraw(Origin::signed(1), 1),
			Error::<TestRuntime>::InvalidIndex
		);
	});
}

#[test]
fn dissolve_works() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		// Transfer fee is taken here
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));

		// Skip all the way to the end
		// Crowdfund is unsuccessful 100 + 200 + 300 < 1000
		run_to_block(50);

		// Check initiator's balance.
		assert_eq!(Balances::free_balance(1), 899);
		// Check current funds (contributions + deposit)
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 601);

		// Account 7 dissolves the crowdfund claiming the remaining funds
		assert_ok!(Crowdfund::dissolve(Origin::signed(7), 0));

		// Fund account is emptied
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 0);
		// Dissolver account is rewarded
		assert_eq!(Balances::free_balance(7), 601);

		// Storage trie is removed
		assert_eq!(Crowdfund::contribution_get(0, &0), 0);
		// Fund storage is removed
		assert_eq!(Crowdfund::funds(0), None);
	});
}

#[test]
fn dissolve_handles_basic_errors() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		// Transfer fee is taken here
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));

		// Cannot dissolve an invalid fund index
		assert_noop!(
			Crowdfund::dissolve(Origin::signed(1), 1),
			Error::<TestRuntime>::InvalidIndex
		);
		// Cannot dissolve an active fund
		assert_noop!(
			Crowdfund::dissolve(Origin::signed(1), 0),
			Error::<TestRuntime>::FundNotRetired
		);

		run_to_block(10);

		// Cannot disolve an ended but not yet retired fund
		assert_noop!(
			Crowdfund::dissolve(Origin::signed(1), 0),
			Error::<TestRuntime>::FundNotRetired
		);
	});
}

#[test]
fn dispense_works() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 20, 1000, 9));
		// Transfer fee is taken here
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));
		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 400));

		// Skip to the retirement period
		// Crowdfund is successful 100 + 200 + 300 + 400  >= 1000
		run_to_block(10);

		// Check initiator's balance.
		assert_eq!(Balances::free_balance(1), 899);
		// Check current funds (contributions + deposit)
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 1001);

		// Account 7 dispenses the crowdfund
		assert_ok!(Crowdfund::dispense(Origin::signed(7), 0));

		// Fund account is emptied
		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 0);
		// Beneficiary account is funded
		assert_eq!(Balances::free_balance(20), 1000);
		// Dispensor account is rewarded deposit
		assert_eq!(Balances::free_balance(7), 1);

		// Storage trie is removed
		assert_eq!(Crowdfund::contribution_get(0, &0), 0);
		// Fund storage is removed
		assert_eq!(Crowdfund::funds(0), None);
	});
}

#[test]
fn dispense_handles_basic_errors() {
	new_test_ext().execute_with(|| {
		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		// Transfer fee is taken here
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));

		// Cannot dispense an invalid fund index
		assert_noop!(
			Crowdfund::dispense(Origin::signed(1), 1),
			Error::<TestRuntime>::InvalidIndex
		);
		// Cannot dispense an active fund
		assert_noop!(
			Crowdfund::dispense(Origin::signed(1), 0),
			Error::<TestRuntime>::FundStillActive
		);

		// Skip to the retirement period
		// Crowdfund is unsuccessful 100 + 200 + 300 < 1000
		run_to_block(10);

		// Cannot disopens an ended but unsuccessful fund
		assert_noop!(
			Crowdfund::dispense(Origin::signed(1), 0),
			Error::<TestRuntime>::UnsuccessfulFund
		);
	});
}
