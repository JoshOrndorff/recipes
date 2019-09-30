use runtime_primitives::traits::{Hash, EnsureOrigin};
use support::{
    decl_event, decl_module, decl_storage,
    dispatch::{Dispatchable, Parameter},
    ensure, StorageMap, StorageValue,
};
use parity_scale_codec::{Encode, Decode};
use system::{self, ensure_signed};

/// Type alias to simplify proposal management
pub type ProposalIndex = u32;
/// Type alias for managing council members
pub type MemberCount = u32;

pub trait Trait: system::Trait {
    /// The outer origin type.
    type Origin: From<RawOrigin<Self::AccountId>>;

    /// The outer call dispatch type
    type Proposal: Parameter + Dispatchable<Origin = <Self as Trait>::Origin>;

    /// The outer event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// special origin type
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RawOrigin<AccountId> {
    // (n, m) It has been approved by n members of the total m
    Members(MemberCount, MemberCount),
    // for verifying member approval of a proposal
    Member(AccountId),
}

// origin type for this module
pub type Origin<T> = RawOrigin<<T as system::Trait>::AccountId>;

#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
/// Tracking internal state for ongoing proposals
pub struct PropState<AccountId> {
    /// The proposal's unique index.
    index: ProposalIndex,
    /// The number of approval votes that are needed to pass the motion.
    threshold: MemberCount,
    /// The current set of voters that approved it.
    ayes: Vec<AccountId>,
    /// The current set of voters that rejected it.
    nays: Vec<AccountId>,
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Hash = <T as system::Trait>::Hash,
    {
        // new member's accountId
        NewMember(AccountId),
        // a motion (given hash) has been proposed by an account with a threshold
        Proposed(AccountId, ProposalIndex, Hash, MemberCount),
        // proposalIndex, voter's accountId, vote(yes/no)
        Voted(AccountId, Hash, bool, MemberCount, MemberCount),
        // proposal_hash; bool is true if executed correctly
        Executed(Hash, bool),
        // member exits; bool is true if executed correctly
        MemberExit(AccountId, bool),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as DAO {
        /// The hashes of the active proposals.
        pub Proposals get(proposals): Vec<T::Hash>;
        /// Actual proposal for a given hash, if it's current.
        pub ProposalOf get(proposal_of): map T::Hash => Option<<T as Trait>::Proposal>;
        /// The proposal's internal state
        pub ProposalState get(proposal_state): map T::Hash => Option<PropState<T::AccountId>>;
        /// Proposals so far.
        pub ProposalCount get(proposal_count): u32;
        /// tracking membership
        pub CurrentMember get(current_member): Vec<T::AccountId>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: <T as system::Trait>::Origin {
        fn deposit_event() = default;

        fn propose(origin, #[compact] threshold: MemberCount, proposal: Box<<T as Trait>::Proposal>) {
            let proposer = ensure_signed(origin)?;
            ensure!(Self::is_member(&proposer), "proposer not a member");

            let proposal_hash = <T as system::Trait>::Hashing::hash_of(&proposal);
            // check that the proposal doesn't already exist
            ensure!(!<ProposalOf<T>>::exists(proposal_hash), "proposal has already been added to the q");

            // add the proposal => make related storage changes
            <Proposals<T>>::mutate(|props| props.push(proposal_hash));
            <ProposalOf<T>>::insert(proposal_hash, proposal.clone());

            if threshold < 2 {
                // only requires approval of one member (proposer's tacit support satisfies)
                let total_members = Self::current_member().len() as MemberCount;
                let ok = proposal.dispatch(RawOrigin::Members(1, total_members).into()).is_ok();
                Self::deposit_event(RawEvent::Executed(proposal_hash, ok));
            } else {
                let index = (Self::proposal_count() + 1) as ProposalIndex;
                <ProposalCount>::mutate(|count| *count + 1);
                let mut ayes = Vec::new();
                let mut nays: Vec<T::AccountId> = Vec::new();
                ayes.push(proposer.clone());
                let prop_state = PropState {
                    index,
                    threshold,
                    ayes,
                    nays,
                };
                <ProposalState<T>>::insert(proposal_hash, prop_state);

                Self::deposit_event(RawEvent::Proposed(proposer, index, proposal_hash, threshold));
            }
        }

        // execution occurs in here as well
        pub fn vote(origin, proposal_hash: T::Hash, approve: bool) {
            let voter = ensure_signed(origin)?;
            ensure!(Self::is_member(&voter), "voter not a member");

            // check if an associated ProposalState exists for the given proposal
            let mut voting = Self::proposal_state(&proposal_hash).ok_or("proposal must exist")?;

            let position_yes = voting.ayes.iter().position(|a| a == &voter);
            let position_no = voting.nays.iter().position(|a| a == &voter);

            if approve {
                if position_yes.is_none() {
                    voting.ayes.push(voter.clone());
                } else {
                    return Err("duplicate vote")
                }
                // executes if the previous vote was no
                if let Some(pos) = position_no {
                    // ability to change vote at no cost prevents bribery attacks
                    voting.nays.swap_remove(pos);
                }
            } else {
                if position_no.is_none() {
                    voting.nays.push(voter.clone());
                } else {
                    return Err("duplicate vote");
                }
                if let Some(pos) = position_yes {
                    voting.ayes.swap_remove(pos);
                }
            }

            let yes_votes = voting.ayes.len() as MemberCount;
            let no_votes = voting.nays.len() as MemberCount;
            Self::deposit_event(RawEvent::Voted(voter, proposal_hash, approve, yes_votes, no_votes));

            let total_members = Self::current_member().len() as MemberCount;
            let approved = yes_votes >= voting.threshold;
            let disapproved = total_members.saturating_sub(no_votes) < voting.threshold;
            if approved || disapproved {
                if approved {
                    if let Some(p) = <ProposalOf<T>>::take(&proposal_hash) {
                        let origin = RawOrigin::Members(voting.threshold, total_members).into();
                        let ok = p.dispatch(origin).is_ok();
                        Self::deposit_event(RawEvent::Executed(proposal_hash, ok));
                    }
                } // else is disapproval

                // remove vote
                <ProposalState<T>>::remove(&proposal_hash);
                <Proposals<T>>::mutate(|props| props.retain(|x| x != &proposal_hash));
            } else {
                // update ProposalState post-voting
                <ProposalState<T>>::insert(&proposal_hash, voting);
            }
        }

        // simple join function (everyone is accepted)
        pub fn join(origin) {
            let new_member = ensure_signed(origin)?;

            // check that they aren't already a member
            ensure!(!Self::is_member(&new_member), "new_member is already a member");

            // add new member to the set of members
            <CurrentMember<T>>::mutate(|members| members.push(new_member.clone()));

            Self::deposit_event(RawEvent::NewMember(new_member));
        }

        // existing member exits
        pub fn exit(origin) {
            let old_member = ensure_signed(origin)?;

            // check that they are an existing member
            ensure!(Self::is_member(&old_member), "origin of exit request is not a member");

            // remove existing member
            <CurrentMember<T>>::mutate(|members| members.retain(|x| x != &old_member));

            Self::deposit_event(RawEvent::MemberExit(old_member, true));
        }
    }
}


impl<T: Trait> Module<T> {
    pub fn is_member(who: &T::AccountId) -> bool {
        Self::current_member().iter().any(|&ref a| a == who)
    }
}

#[cfg(test)]
mod tests {
    use super::RawEvent;
    use super::*;
    use crate::tests::*;
    use crate::tests::{Call, Event as OuterEvent, Origin};
    use runtime_primitives::traits::BlakeTwo256;
    use support::{assert_noop, assert_ok, Hashable};
    use system;

    #[test]
    fn basic_setup_works() {
        let mut t = new_test_ext();
        with_externalities(&mut t, || {
            System::set_block_number(1);
            assert_eq!(DAO::proposals(), Vec::<H256>::new());
        });
    }

    #[test]
    fn add_existing_member_fails() {
        let mut t = new_test_ext();
        with_externalities(&mut t, || {
            System::set_block_number(1);
            assert_ok!(DAO::join(Origin::signed(1)));
            assert_noop!(
                DAO::join(Origin::signed(1)),
                "new_member is already a member"
            );
        });
    }

    #[test]
    fn non_member_exit_fails() {
        let mut t = new_test_ext();
        with_externalities(&mut t, || {
            System::set_block_number(1);
            assert_noop!(
                DAO::exit(Origin::signed(2)),
                "origin of exit request is not a member"
            );
        });
    }
}
