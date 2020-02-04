//! A Simple Charity which holds and governs a pot of funds.
//!
//! The Charity has a pot of funds. The Pot is unique because unlike other token-holding accounts,
//! it is not controlled by a cryptographic keypair. Rather it belongs to the pallet itself.
//! Anyone can donate funds to the pot through the donate extrinsic.
//! Anyone can propose where funds be allocated through the propose_charity_spend extrinsic. Each
//! spend proposal requires the proposer to lock some funds.
//! Each SpendPeriod, members of a council which was established at genesis vote on which proposal
//! they most prefer and that proposal is executed.
#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use rstd::prelude::*;
use sp_runtime::{
    traits::{AccountIdConversion, CheckedAdd, CheckedSub, Zero},
    ModuleId, RuntimeDebug,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use support::traits::{Currency, ExistenceRequirement::AllowDeath, Get, ReservableCurrency};
use support::{decl_event, decl_module, decl_storage, dispatch::{DispatchResult, DispatchError}, ensure, StorageValue};
use system::{self, ensure_signed};

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

// This pallet ID will be used to create the special Pot Account
const PALLET_ID: ModuleId = ModuleId(*b"SimpleCharity");

pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The currency type that the charity deals in
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	/// The amount that must be bonded to propose a fund recipient
	type ProposalBond: Get<BalanceOf<Self>>;
    /// Period between successive treasuery spends
    type CharitySpendPeriod: Get<Self::BlockNumber>;
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Proposal<AccountId, Balance> {
    /// Receiving Account
    to: AccountId,
    /// Expenditure amount
    amount: Balance,
}

decl_storage! {
    trait Store for Module<T: Trait> as SimpleTreasury {
        /// The members who vote on how treasury funds are spent
        Council get(fn council) config():  Vec<T::AccountId>;
        /// The proposals for treasury spending
        Proposals get(fn proposals): map T::AccountId => Option<Proposal<T::AccountId, BalanceOf<T>, T::BlockNumber>>;
    }
    add_extra_genesis {
        build(|config| {
            // Create the charity's pot of funds, and ensure it has the minimum required deposit
            let _ = T::Currency::make_free_balance_be(
                &<Module<T>>::account_id(),
                T::Currency::minimum_balance(),
            );
        });
    }
}

decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        <T as system::Trait>::AccountId,
        <T as system::Trait>::BlockNumber,
    {
		/// Donor has made a charitable donation to the charity
		DonationReceived(AccountId, Balance),
        /// Fund allocation proposed (proposed destination)
        AllocationProposed(AccountId, Balance),
        /// Funds allocated by the charity
        FundsAllocated(AccountId, Balance, BlockNumber),
        /// For testing purposes, to impl From<()> for TestEvent to assign `()` to balances::Event
		/// TODO Do we even need this?
        NullEvent(u32), // u32 could be aliases as an error code for mocking setup
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        const CharitySpendPeriodSpend: T::BlockNumber = T::TreasurySpend::get();

        /// Donate some funds to the charity
        fn donate(
            origin,
            amount: BalanceOf<T>
        ) -> DispatchResult {
            let donor = ensure_signed(origin)?;

            let _ = T::Currency::transfer(&donor, &Self::account_id(), &amount, AllowDeath);

            Self::deposit_event(RawEvent::DonationMade(donor, amount));
            Ok(())
        }

        /// Propose Spend
		///
        /// Anyone can propose the charity allocate funds to a particular cause in exchange for
		/// locking some funds.
		/// (discovery and discussion of worthwhile projects/people would be off-chain)
        fn spend(
            origin,
            dest: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            //ensure_root

			let _ = T::Currency::transfer(
				&Self::account_id(),
				&proposal.to,
				proposal.amount,
				AllowDeath,
			);

            Self::deposit_event(RawEvent::TreasuryProposal(dest, amount));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// Check that the signer is on the Treasury's Council
    pub fn is_on_council(who: &T::AccountId) -> bool {
        Self::council().contains(who)
    }

    /// The account ID of the Treasury's Council
    pub fn account_id() -> T::AccountId {
        PALLET_ID.into_account()
    }

    /// The balance of the Treasury's Council's pot of taxed funds
    fn pot() -> BalanceOf<T> {
        T::Currency::free_balance(&Self::account_id())
    }
}

impl<T: Trait> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
		let numeric_amount = amount.peek();

		// Must resolve into existing but better to be safe.
		let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

		Self::deposit_event(RawEvent::Deposit(numeric_amount));
	}
}

#[cfg(test)]
mod tests {
    use crate::*;
    use balances;
    use primitives::H256;
    use runtime_io;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use support::{assert_ok, assert_err, impl_outer_event, impl_outer_origin, parameter_types};

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

        pub const ExistentialDeposit: u64 = 0;
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
    }

    impl balances::Trait for TestRuntime {
        type Balance = u64;
        type OnFreeBalanceZero = ();
        type OnNewAccount = ();
        type Event = ();
        type TransferPayment = ();
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type TransferFee = TransferFee;
        type CreationFee = CreationFee;
    }

    mod treasury {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            treasury<T>,
        }
    }

    impl std::convert::From<()> for TestEvent {
        fn from(_unit: ()) -> Self {
            TestEvent::treasury(RawEvent::NullEvent(6))
        }
    }

    parameter_types! {
        pub const Tax: u64 = 2;
        pub const UserSpend: u64 = 10;
        pub const TreasurySpend: u64 = 10;
        pub const MinimumProposalAge: u64 = 3;
    }
    impl Trait for TestRuntime {
        type Event = TestEvent;
        type Currency = balances::Module<Self>;
        type Tax = Tax;
        type UserSpend = UserSpend;
        type TreasurySpend = TreasurySpend;
        type MinimumProposalAge = MinimumProposalAge;
    }

    pub type System = system::Module<TestRuntime>;
    pub type Balances = balances::Module<TestRuntime>;
    pub type Treasury = Module<TestRuntime>;

    // An alternative to `ExtBuilder` which includes custom configuration
    pub fn new_test_ext() -> runtime_io::TestExternalities {
        let mut t = system::GenesisConfig::default()
            .build_storage::<TestRuntime>()
            .unwrap();
        balances::GenesisConfig::<TestRuntime> {
            balances: vec![
                // members of council (can also be users)
                (1, 13),
                (2, 11),
                (3, 1),
                (4, 3),
                (5, 19),
                (6, 23),
                (7, 17),
                // users, not members of council
                (8, 1),
                (9, 22),
                (10, 46),
            ],
            vesting: vec![],
        }
        .assimilate_storage(&mut t)
        .unwrap();
        GenesisConfig::<TestRuntime> {
            council: vec![1, 2, 3, 4, 5, 6, 7],
        }
        .assimilate_storage(&mut t)
        .unwrap();
        t.into()
    }

    /// Verifying correct behavior of boilerplate
    #[test]
    fn new_test_ext_behaves() {
        new_test_ext().execute_with(|| {
            // check membership initiated correctly
            assert!(Treasury::is_on_council(&1));
            assert!(Treasury::is_on_council(&2));
            assert!(Treasury::is_on_council(&3));
            assert!(Treasury::is_on_council(&4));
            assert!(Treasury::is_on_council(&5));
            assert!(Treasury::is_on_council(&6));
            assert!(Treasury::is_on_council(&7));
        })
    }

    /// Transfer reserves tax == 2
    #[test]
    fn transfer_reserves_tax() {
        new_test_ext().execute_with(|| {
            assert_err!(
                Treasury::request_transfer(Origin::signed(3), 1, 1),
                "Must be able to pay tax to make transfer"
            );
            assert_ok!(Treasury::request_transfer(Origin::signed(1), 2, 8));
            assert_eq!(Balances::reserved_balance(&1), 2);
            let mock_spend_request = SpendRequest {
                from: 1,
                to: 2,
                amount: 8, // Balances::from()
            };
            // check that the expected spend request is in runtime storage
            assert!(Treasury::transfer_requests()
                .iter()
                .any(|a| *a == mock_spend_request));

            // check that user debt is correctly tracked
            assert_eq!(Treasury::user_debt(&1).unwrap(), 8,);

            // check that the correct event is emitted
            let expected_event = TestEvent::treasury(RawEvent::TransferRequested(
                1,
                2,
                8,
            ));
            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn propose_treasury_spend_works() {
        new_test_ext().execute_with(|| {
            assert_err!(
                Treasury::propose_treasury_spend(Origin::signed(8), 1, 10u64.into()),
                "must be on council to make proposal"
            );
            System::set_block_number(5);
            assert_ok!(Treasury::propose_treasury_spend(Origin::signed(1), 8, 10u64.into()));

            let expected_proposal = Proposal {
                to: 8,
                amount: 10u64.into(),
                when: 5u64.into(),
                support: 1u32,
            };
            assert_eq!(
                Treasury::proposals(1).unwrap(),
                expected_proposal
            );

            let expected_event = TestEvent::treasury(RawEvent::TreasuryProposal(
                8,
                10u64.into(),
            ));
            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    // TODO: test
    // - user_spend and expected behavior in different environments with `on_finalize`
    // - treasury_spend and expected behavior in different environments with `on_finalize`
    // - both in different order (need to test all possible overlapping configurations, maybe in a model checker like TLA+)
}
