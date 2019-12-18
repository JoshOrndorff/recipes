// not simple treasury
#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use rstd::prelude::*;
use runtime_primitives::{
    traits::{AccountIdConversion, CheckedAdd, CheckedSub, Zero},
    ModuleId, RuntimeDebug,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use support::traits::{Currency, ExistenceRequirement::AllowDeath, Get, ReservableCurrency};
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue};
use system::{self, ensure_signed};

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
const MODULE_ID: ModuleId = ModuleId(*b"example ");

pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The currency type for this module
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    /// The collateral associated with a transfer
    type Tax: Get<BalanceOf<Self>>;
    /// Period between successive batch spends
    type UserSpend: Get<Self::BlockNumber>;
    /// Period between successive treasuery spends
    type TreasurySpend: Get<Self::BlockNumber>;
    /// Minimum amount of time until a proposal might get approved
    type MinimumProposalAge: Get<Self::BlockNumber>;
}

#[derive(Encode, Decode, PartialEq)]
pub struct SpendRequest<AccountId, Balance> {
    /// Sending account
    from: AccountId,
    /// Receiving account
    to: AccountId,
    /// Send amount
    amount: Balance,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Proposal<AccountId, Balance, BlockNumber> {
    /// Receiving Account
    to: AccountId,
    /// Expenditure amount
    amount: Balance,
    /// Submission blocknumber (for age requirement)
    when: BlockNumber,
    /// Simple support metric
    support: u32,
}

decl_storage! {
    trait Store for Module<T: Trait> as SmplTreasury {
        /// The amount, the address to which it is sent
        TransferRequests get(fn transfer_requests): Vec<SpendRequest<T::AccountId, BalanceOf<T>>>;
        /// Track user debt (to prevent excessive requests beyond means)
        UserDebt get(fn user_debt): map T::AccountId => Option<BalanceOf<T>>;
        /// The members which vote on how taxes are spent
        Council get(fn council) config():  Vec<T::AccountId>;
        /// The proposals for treasury spending
        Proposals get(fn proposals): map T::AccountId => Option<Proposal<T::AccountId, BalanceOf<T>, T::BlockNumber>>;
    }
    add_extra_genesis {
        build(|config| {
            // Create the receiving treasury account
            let _ = T::Currency::make_free_balance_be(
                &<Module<T>>::account_id(),
                T::Currency::minimum_balance(),
            );

            <Council<T>>::put(&config.council);
        });
    }
}

decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        <T as system::Trait>::AccountId,
        <T as system::Trait>::BlockNumber
    {
        /// New spend request
        TransferRequested(AccountId, AccountId, Balance),
        /// Transfer request approved and delayed spend is executed
        TransferExecuted(AccountId, Balance, BlockNumber),
        /// Treasury spend proposed (proposed destination)
        TreasuryProposal(AccountId, Balance),
        /// Treasury spend executed
        TreasurySpent(AccountId, Balance, BlockNumber),
        /// For testing purposes, to impl From<()> for TestEvent to assign `()` to balances::Event
        NullEvent(u32), // u32 could be aliases as an error code for mocking setup
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        const Tax: BalanceOf<T> = T::Tax::get();
        const UserSpend: T::BlockNumber = T::UserSpend::get();
        const TreasurySpend: T::BlockNumber = T::TreasurySpend::get();
        const MinimumProposalAge: T::BlockNumber = T::MinimumProposalAge::get();

        /// Transfer Request
        ///
        /// SpendRequest`s are queued in the `SpendQ` which is dispatched every `SpendPeriod`
        /// The tax amount is reserved from sender and paid when executing the spend (in an atomic operation)
        fn request_transfer(
            origin,
            dest: T::AccountId,
            amount: BalanceOf<T>
        ) -> Result {
            let sender = ensure_signed(origin)?;

            // the bond calculation could depend on the transfer amount
            let bond = T::Tax::get();
            T::Currency::reserve(&sender, bond)
                .map_err(|_| "Must be able to pay tax to make transfer")?;
            // error message could print the tax amount

            // could add some ensure statement to prevent excessive requests by checking `UserDebt`

            let requested_spend = SpendRequest {
                from: sender.clone(),
                to: dest.clone(),
                amount: amount.clone(),
            };
            <TransferRequests<T>>::append(&[requested_spend])?;
            if let Some(mut new_debt) = <UserDebt<T>>::get(sender.clone()) {
                new_debt.checked_add(&amount.clone()).ok_or("overflowed upon adding user_debt")?;
            } else {
                <UserDebt<T>>::insert(sender.clone(), amount.clone());
            }
            Self::deposit_event(RawEvent::TransferRequested(sender, dest, amount));
            Ok(())
        }

        /// Propose Spend
        ///
        /// members can propose capital spending to addresses (from the pot)
        /// (discovery and discussion of worthwhile projects/people would be off-chain)
        fn propose_treasury_spend(
            origin,
            dest: T::AccountId,
            amount: BalanceOf<T>,
        ) -> Result {
            let proposer = ensure_signed(origin)?;
            ensure!(Self::is_on_council(&proposer), "must be on council to make proposal");

            // <*could add a bond associated with proposals here*>

            let proposed_expenditure = Proposal {
                to: dest.clone(),
                amount: amount.clone(),
                when: <system::Module<T>>::block_number(),
                support: 1u32,
            };
            // if previous proposal exists, it is overwritten
            <Proposals<T>>::insert(&proposer, proposed_expenditure);

            Self::deposit_event(RawEvent::TreasuryProposal(dest, amount));
            Ok(())
        }

        /// Stupid Vote
        ///
        /// No anti-sybil mechanism for voters
        /// 1 vote per call, but no limit on votes
        fn stupid_vote(
            origin,
            vote: T::AccountId,
        ) -> Result {
            let voter = ensure_signed(origin)?;
            ensure!(Self::is_on_council(&voter), "the voter is on the council");
            if let Some(mut proposal) = <Proposals<T>>::get(vote) {
                proposal.support += 1;
            } else {
                return Err("proposal associated with vote does not exist")
            }
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::UserSpend::get()).is_zero() {
                // every `UserSpend` number of blocks,
                // spend the funds according to member preferences
                Self::user_spend();
            }

            if (n % T::TreasurySpend::get()).is_zero() {
                Self::treasury_spend();
            }

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
        MODULE_ID.into_account()
    }

    /// The balance of the Treasury's Council's pot of taxed funds
    fn pot() -> BalanceOf<T> {
        T::Currency::free_balance(&Self::account_id())
    }

    /// The user spend method for executing transfers requested by users
    fn user_spend() {
        <TransferRequests<T>>::get().into_iter().for_each(|request| {
            // execute the transfer request
            let _ = T::Currency::transfer(&request.from, &request.to, request.amount, AllowDeath);
            // update the UserDebt storage value designed to manage spending requests queue
            let mut new_debt = <UserDebt<T>>::get(&request.from).expect("the debt is only declared in one method and forgiven in this method; weak qed");
            new_debt.checked_sub(&request.amount).expect("this amount could not underflow because every call matches the symmetric request in `transfer_request` `=>` > 0 always; qed");
            <UserDebt<T>>::insert(&request.from, new_debt);
            // get the tax
            let tax_to_pay = T::Tax::get();
            // unreserve the tax from the sender
            T::Currency::unreserve(&request.from, tax_to_pay);
            // pay the associated tax from the sender to the treasury account
            let _ = T::Currency::transfer(&request.from, &Self::account_id(), tax_to_pay, AllowDeath);
        });
    }

    /// The treasury pot spends according to proposals and votes
    ///
    /// too much iteration in the runtime?
    fn treasury_spend() {
        let mut budget_remaining = Self::pot();
        // take a slice of proposals with a certain age
        let required_age = T::MinimumProposalAge::get();
        let now = <system::Module<T>>::block_number();
        let mut old_enough_proposals = Vec::new();
        <Council<T>>::get().into_iter().for_each(|member| {
            if let Some(proposal) = <Proposals<T>>::get(member) {
                if now - proposal.when > required_age.into() {
                    old_enough_proposals.push(proposal);
                }
            }
        });

        // sort based on support and pay off as many as possible with the budget
        old_enough_proposals.into_iter().for_each(|proposal| {
            if proposal.amount <= budget_remaining {
                budget_remaining -= proposal.amount;
                let _ = T::Currency::transfer(
                    &Self::account_id(),
                    &proposal.to,
                    proposal.amount,
                    AllowDeath,
                );
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*; //{Module, Trait, RawEvent, SpendRequest, GenesisConfig};
    use balances;
    use primitives::H256;
    use runtime_io;
    use runtime_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup, OnFinalize},
        Perbill,
    };
    use support::{assert_err, impl_outer_event, impl_outer_origin, parameter_types, traits::Get};
    use system::ensure_signed;

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
        fn from(unit: ()) -> Self {
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

    /// Auxiliary method for simulating block time passing
    fn run_to_block(n: u64) {
        while System::block_number() < n {
            Treasury::on_finalize(System::block_number());
            System::set_block_number(System::block_number() + 1);
        }
    }

    /// Verifying correct behavior of boilerplate
    #[test]
    fn new_test_ext_behaves() {
        new_test_ext().execute_with(|| {
            // check membership initiated correctly
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let second_account = ensure_signed(Origin::signed(2)).unwrap();
            let third_account = ensure_signed(Origin::signed(3)).unwrap();
            let fourth_account = ensure_signed(Origin::signed(4)).unwrap();
            let fifth_account = ensure_signed(Origin::signed(5)).unwrap();
            let sixth_account = ensure_signed(Origin::signed(6)).unwrap();
            let seventh_account = ensure_signed(Origin::signed(7)).unwrap();
            assert!(Treasury::is_on_council(&first_account));
            assert!(Treasury::is_on_council(&second_account));
            assert!(Treasury::is_on_council(&third_account));
            assert!(Treasury::is_on_council(&fourth_account));
            assert!(Treasury::is_on_council(&fifth_account));
            assert!(Treasury::is_on_council(&sixth_account));
            assert!(Treasury::is_on_council(&seventh_account));
        })
    }

    /// Transfer reserves tax == 2
    fn transfer_reserves_tax() {
        new_test_ext().execute_with(|| {
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let second_account = ensure_signed(Origin::signed(2)).unwrap();
            assert_err!(
                Treasury::request_transfer(Origin::signed(3), first_account, 1),
                "Must be able to pay tax to make transfer"
            );
            Treasury::request_transfer(Origin::signed(1), second_account, 8);
            assert_eq!(Balances::reserved_balance(&first_account), 2);
            let mock_spend_request = SpendRequest {
                from: first_account.clone(),
                to: second_account.clone(),
                amount: 8, // Balances::from()
            };
            // check that the expected spend request is in runtime storage
            assert!(Treasury::transfer_requests()
                .iter()
                .any(|a| *a == mock_spend_request));

            // check that user debt is correctly tracked
            assert_eq!(Treasury::user_debt(&first_account).unwrap(), 8,);

            // check that the correct event is emitted
            let expected_event = TestEvent::treasury(RawEvent::TransferRequested(
                first_account.clone(),
                second_account.clone(),
                10,
            ));
            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn propose_treasury_spend_works() {
        new_test_ext().execute_with(|| {
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let eighth_account = ensure_signed(Origin::signed(8)).unwrap();
            assert_err!(
                Treasury::propose_treasury_spend(Origin::signed(8), first_account, 10u64.into()),
                "must be on council to make proposal"
            );
            System::set_block_number(5);
            Treasury::propose_treasury_spend(Origin::signed(1), eighth_account, 10u64.into());

            let expected_proposal = Proposal {
                to: eighth_account.clone(),
                amount: 10u64.into(),
                when: 5u64.into(),
                support: 1u32,
            };
            assert_eq!(
                Treasury::proposals(first_account).unwrap(),
                expected_proposal
            );

            let expected_event = TestEvent::treasury(RawEvent::TreasuryProposal(
                eighth_account.clone(),
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
