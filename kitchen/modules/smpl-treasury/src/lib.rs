// simple treasury
#![cfg_attr(not(feature = "std"), no_std)]

use runtime_primitives::traits::{AccountIdConversion, Zero};
use runtime_primitives::{ModuleId, RuntimeDebug};
use support::traits::{Currency, ReservableCurrency, Get, ExistenceRequirement::AllowDeath};
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue};
use parity_scale_codec::{Encode, Decode};
use system::{self, ensure_signed};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use rstd::prelude::*;

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

#[derive(Encode, Decode)]
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
        TransferRequests get(fn treasury_requests): Vec<SpendRequest<T::AccountId, BalanceOf<T>>>;
        /// The members which vote on how taxes are spent
        Council get(fn council):  Vec<T::AccountId>;
        /// The proposals that have yet to be approved for spending
        Proposals get(fn proposals): map T::AccountId => Option<Proposal<T::AccountId, BalanceOf<T>, T::BlockNumber>>;
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

            let bond = T::Tax::get();
            T::Currency::reserve(&sender, bond)
                .map_err(|_| "Must be able to pay tax to make transfer")?;
            
            let requested_spend = SpendRequest {
                from: sender.clone(),
                to: dest.clone(),
                amount: amount.clone(),
            };
            <TransferRequests<T>>::append(&[requested_spend])?;
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
                let _ = T::Currency::transfer(&Self::account_id(), &proposal.to, proposal.amount, AllowDeath);
            }
        })
    }
}