use super::*;

use frame_support::{
	impl_outer_origin, assert_ok, assert_noop, parameter_types,
	traits::{OnInitialize, OnFinalize},
};
use frame_support::traits::{Contains, ContainsLengthBound};
use sp_core::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
use sp_runtime::{
	Perbill, Permill, Percent, testing::Header, DispatchResult,
	traits::{BlakeTwo256, IdentityLookup},
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
	pub const BlockHashCount: u32 = 250;
	pub const MaximumBlockWeight: u32 = 4 * 1024 * 1024;
	pub const MaximumBlockLength: u32 = 4 * 1024 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
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
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl balances::Trait for Test {
	type Balance = u64;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
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
impl Trait for Test {
	type Event = ();
	type Currency = Balances;
	type SubmissionDeposit = SubmissionDeposit;
	type MinContribution = MinContribution;
	type RetirementPeriod = RetirementPeriod;
}

type System = system::Module<Test>;
type Balances = balances::Module<Test>;
type Crowdfund = Module<Test>;
use balances::Error as BalancesError;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	balances::GenesisConfig::<Test>{
		balances: vec![(1, 1000), (2, 2000), (3, 3000), (4, 4000)],
	}.assimilate_storage(&mut t).unwrap();
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
			BalancesError::<Test, _>::InsufficientBalance
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
		assert_noop!(Crowdfund::contribute(Origin::signed(1), 0, 49), Error::<Test>::InvalidIndex);
		// Cannot contribute below minimum contribution
		assert_noop!(Crowdfund::contribute(Origin::signed(1), 0, 9), Error::<Test>::ContributionTooSmall);

		// Set up a crowdfund
		assert_ok!(Crowdfund::create(Origin::signed(1), 2, 1000, 9));
		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 101));

		// Move past end date
		run_to_block(10);

		// Cannot contribute to ended fund
		assert_noop!(Crowdfund::contribute(Origin::signed(1), 0, 49), Error::<Test>::ContributionPeriodOver);
	});
}

// #[test]
// fn fix_deploy_data_works() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		// Add deploy data
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into()
// 		));
//
// 		let fund = Crowdfund::funds(0).unwrap();
//
// 		// Confirm deploy data is stored correctly
// 		assert_eq!(
// 			fund.deploy_data,
// 			Some(DeployData {
// 				code_hash: <Test as system::Trait>::Hash::default(),
// 				code_size: 0,
// 				initial_head_data: vec![0].into(),
// 			}),
// 		);
// 	});
// }
//
// #[test]
// fn fix_deploy_data_handles_basic_errors() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		// Cannot set deploy data by non-owner
// 		assert_noop!(Crowdfund::fix_deploy_data(
// 			Origin::signed(2),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into()),
// 			Error::<Test>::InvalidOrigin
// 		);
//
// 		// Cannot set deploy data to an invalid index
// 		assert_noop!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			1,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into()),
// 			Error::<Test>::InvalidFundIndex
// 		);
//
// 		// Cannot set deploy data after it already has been set
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
//
// 		assert_noop!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![1].into()),
// 			Error::<Test>::ExistingDeployData
// 		);
// 	});
// }
//
// #[test]
// fn onboard_works() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		// Add deploy data
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
//
// 		// Fund crowdfund
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 1000));
//
// 		run_to_block(10);
//
// 		// Endings count incremented
// 		assert_eq!(Crowdfund::endings_count(), 1);
//
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
//
// 		let fund = Crowdfund::funds(0).unwrap();
// 		// Crowdfund is now assigned a parachain id
// 		assert_eq!(fund.parachain, Some(0.into()));
// 		// This parachain is managed by Slots
// 		assert_eq!(Slots::managed_ids(), vec![0.into()]);
// 	});
// }
//
// #[test]
// fn onboard_handles_basic_errors() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		// Fund crowdfund
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 1000));
//
// 		run_to_block(10);
//
// 		// Cannot onboard invalid fund index
// 		assert_noop!(Crowdfund::onboard(Origin::signed(1), 1, 0.into()), Error::<Test>::InvalidFundIndex);
// 		// Cannot onboard crowdfund without deploy data
// 		assert_noop!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()), Error::<Test>::UnsetDeployData);
//
// 		// Add deploy data
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
//
// 		// Cannot onboard fund with incorrect parachain id
// 		assert_noop!(Crowdfund::onboard(Origin::signed(1), 0, 1.into()), SlotsError::<Test>::ParaNotOnboarding);
//
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
//
// 		// Cannot onboard fund again
// 		assert_noop!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()), Error::<Test>::AlreadyOnboard);
// 	});
// }
//
// #[test]
// fn begin_retirement_works() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		// Add deploy data
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
//
// 		// Fund crowdfund
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 1000));
//
// 		run_to_block(10);
//
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
// 		// Fund is assigned a parachain id
// 		let fund = Crowdfund::funds(0).unwrap();
// 		assert_eq!(fund.parachain, Some(0.into()));
//
// 		// Off-boarding is set to the crowdfund account
// 		assert_eq!(Slots::offboarding(ParaId::from(0)), Crowdfund::fund_account_id(0));
//
// 		run_to_block(50);
//
// 		// Retire crowdfund to remove parachain id
// 		assert_ok!(Crowdfund::begin_retirement(Origin::signed(1), 0));
//
// 		// Fund should no longer have parachain id
// 		let fund = Crowdfund::funds(0).unwrap();
// 		assert_eq!(fund.parachain, None);
//
// 	});
// }
//
// #[test]
// fn begin_retirement_handles_basic_errors() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		// Add deploy data
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
//
// 		// Fund crowdfund
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 1000));
//
// 		run_to_block(10);
//
// 		// Cannot retire fund that is not onboarded
// 		assert_noop!(Crowdfund::begin_retirement(Origin::signed(1), 0), Error::<Test>::NotParachain);
//
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
// 		// Fund is assigned a parachain id
// 		let fund = Crowdfund::funds(0).unwrap();
// 		assert_eq!(fund.parachain, Some(0.into()));
//
// 		// Cannot retire fund whose deposit has not been returned
// 		assert_noop!(Crowdfund::begin_retirement(Origin::signed(1), 0), Error::<Test>::ParaHasDeposit);
//
// 		run_to_block(50);
//
// 		// Cannot retire invalid fund index
// 		assert_noop!(Crowdfund::begin_retirement(Origin::signed(1), 1), Error::<Test>::InvalidFundIndex);
//
// 		// Cannot retire twice
// 		assert_ok!(Crowdfund::begin_retirement(Origin::signed(1), 0));
// 		assert_noop!(Crowdfund::begin_retirement(Origin::signed(1), 0), Error::<Test>::NotParachain);
// 	});
// }
//
// #[test]
// fn withdraw_works() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		// Transfer fee is taken here
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));
//
// 		// Skip all the way to the end
// 		run_to_block(50);
//
// 		// User can withdraw their full balance without fees
// 		assert_ok!(Crowdfund::withdraw(Origin::signed(1), 0));
// 		assert_eq!(Balances::free_balance(1), 999);
//
// 		assert_ok!(Crowdfund::withdraw(Origin::signed(2), 0));
// 		assert_eq!(Balances::free_balance(2), 2000);
//
// 		assert_ok!(Crowdfund::withdraw(Origin::signed(3), 0));
// 		assert_eq!(Balances::free_balance(3), 3000);
// 	});
// }
//
// #[test]
// fn withdraw_handles_basic_errors() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		// Transfer fee is taken here
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 49));
// 		assert_eq!(Balances::free_balance(1), 950);
//
// 		run_to_block(5);
//
// 		// Cannot withdraw before fund ends
// 		assert_noop!(Crowdfund::withdraw(Origin::signed(1), 0), Error::<Test>::FundNotEnded);
//
// 		run_to_block(10);
//
// 		// Cannot withdraw if they did not contribute
// 		assert_noop!(Crowdfund::withdraw(Origin::signed(2), 0), Error::<Test>::NoContributions);
// 		// Cannot withdraw from a non-existent fund
// 		assert_noop!(Crowdfund::withdraw(Origin::signed(1), 1), Error::<Test>::InvalidFundIndex);
// 	});
// }
//
// #[test]
// fn dissolve_works() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		// Transfer fee is taken here
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));
//
// 		// Skip all the way to the end
// 		run_to_block(50);
//
// 		// Check initiator's balance.
// 		assert_eq!(Balances::free_balance(1), 899);
// 		// Check current funds (contributions + deposit)
// 		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 601);
//
// 		// Dissolve the crowdfund
// 		assert_ok!(Crowdfund::dissolve(Origin::signed(1), 0));
//
// 		// Fund account is emptied
// 		assert_eq!(Balances::free_balance(Crowdfund::fund_account_id(0)), 0);
// 		// Deposit is returned
// 		assert_eq!(Balances::free_balance(1), 900);
// 		// Treasury account is filled
// 		assert_eq!(Balances::free_balance(Treasury::account_id()), 600);
//
// 		// Storage trie is removed
// 		assert_eq!(Crowdfund::contribution_get(0,&0), 0);
// 		// Fund storage is removed
// 		assert_eq!(Crowdfund::funds(0), None);
//
// 	});
// }
//
// #[test]
// fn dissolve_handles_basic_errors() {
// 	new_test_ext().execute_with(|| {
// 		// Set up a crowdfund
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		// Transfer fee is taken here
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 100));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(2), 0, 200));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(3), 0, 300));
//
// 		// Cannot dissolve an invalid fund index
// 		assert_noop!(Crowdfund::dissolve(Origin::signed(1), 1), Error::<Test>::InvalidFundIndex);
// 		// Cannot dissolve a fund in progress
// 		assert_noop!(Crowdfund::dissolve(Origin::signed(1), 0), Error::<Test>::InRetirementPeriod);
//
// 		run_to_block(10);
//
// 		// Onboard fund
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
//
// 		// Cannot dissolve an active fund
// 		assert_noop!(Crowdfund::dissolve(Origin::signed(1), 0), Error::<Test>::HasActiveParachain);
// 	});
// }
//
// #[test]
// fn fund_before_auction_works() {
// 	new_test_ext().execute_with(|| {
// 		// Create a crowdfund before an auction is created
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 9));
// 		// Users can already contribute
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 49));
// 		// Fund added to NewRaise
// 		assert_eq!(Crowdfund::new_raise(), vec![0]);
//
// 		// Some blocks later...
// 		run_to_block(2);
// 		// Create an auction
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		// Add deploy data
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
// 		// Move to the end of auction...
// 		run_to_block(12);
//
// 		// Endings count incremented
// 		assert_eq!(Crowdfund::endings_count(), 1);
//
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
//
// 		let fund = Crowdfund::funds(0).unwrap();
// 		// Crowdfund is now assigned a parachain id
// 		assert_eq!(fund.parachain, Some(0.into()));
// 		// This parachain is managed by Slots
// 		assert_eq!(Slots::managed_ids(), vec![0.into()]);
// 	});
// }
//
// #[test]
// fn fund_across_multiple_auctions_works() {
// 	new_test_ext().execute_with(|| {
// 		// Create an auction
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		// Create two competing crowdfunds, with end dates across multiple auctions
// 		// Each crowdfund is competing for the same slots, so only one can win
// 		assert_ok!(Crowdfund::create(Origin::signed(1), 1000, 1, 4, 30));
// 		assert_ok!(Crowdfund::create(Origin::signed(2), 1000, 1, 4, 30));
//
// 		// Contribute to all, but more money to 0, less to 1
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 0, 300));
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 1, 200));
//
// 		// Add deploy data to all
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(1),
// 			0,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
// 		assert_ok!(Crowdfund::fix_deploy_data(
// 			Origin::signed(2),
// 			1,
// 			<Test as system::Trait>::Hash::default(),
// 			0,
// 			vec![0].into(),
// 		));
//
// 		// End the current auction, fund 0 wins!
// 		run_to_block(10);
// 		assert_eq!(Crowdfund::endings_count(), 1);
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(1), 0, 0.into()));
// 		let fund = Crowdfund::funds(0).unwrap();
// 		// Crowdfund is now assigned a parachain id
// 		assert_eq!(fund.parachain, Some(0.into()));
// 		// This parachain is managed by Slots
// 		assert_eq!(Slots::managed_ids(), vec![0.into()]);
//
// 		// Create a second auction
// 		assert_ok!(Slots::new_auction(Origin::ROOT, 5, 1));
// 		// Contribute to existing funds add to NewRaise
// 		assert_ok!(Crowdfund::contribute(Origin::signed(1), 1, 10));
//
// 		// End the current auction, fund 1 wins!
// 		run_to_block(20);
// 		assert_eq!(Crowdfund::endings_count(), 2);
// 		// Onboard crowdfund
// 		assert_ok!(Crowdfund::onboard(Origin::signed(2), 1, 1.into()));
// 		let fund = Crowdfund::funds(1).unwrap();
// 		// Crowdfund is now assigned a parachain id
// 		assert_eq!(fund.parachain, Some(1.into()));
// 		// This parachain is managed by Slots
// 		assert_eq!(Slots::managed_ids(), vec![0.into(), 1.into()]);
// 	});
// }
