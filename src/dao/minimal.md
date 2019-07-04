# Minimal (Non-)Viable DAO

[`mini-dao`](https://github.com/4meta5/mini-dao) is a minimal governance module pulling almost exclusively from the [council module](https://github.com/paritytech/substrate/tree/master/srml/council) that will be used in [Polkadot's](https://research.web3.foundation/en/latest/polkadot/) on-chain governance. It is *minimal* in the sense that it lacks basic incentives, weighted signalling, and other features that might be expected from a fully-functioning DAO.

* [Custom Origin Type](#origin)
* [Membership Management](#member)
* [Generic Proposals](#propose)
* [1 Member 1 Vote](#vote)

> *some note here about all the things we can and should build on top of it*

## Custom Origin Type <a name = "origin"></a>

This module uses a special origin type to specify conditions for dispatching special extrinsics called `Proposal`s. 

In our `Trait` declaration, we define the `Origin` type according to our special origin type and we also define the bounds on the outer call dispatch type known as `Proposal`.

```rust
pub trait Trait: system::Trait {
	type Origin: From<RawOrigin<Self::AccountId>>;

	type Proposal: Parameter + Dispatchable<Origin=<Self as Trait>::Origin>;

	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

We use the special origin type to verify the threshold of members that have approved a proposal (the first field). In addition, we dispatch proposals by verifying that the participant that makes this request is a member of the DAO (using the second field).

```rust
/// special origin type
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RawOrigin<AccountId> {
	Members(MemberCount, MemberCount),
	// the main address for the DAO
	Member(AccountId),
}

// origin type for this module
pub type Origin<T> = RawOrigin<<T as system::Trait>::AccountId>; 
```

A simple example of how this works can be found in the `propose` runtime method. If the threshold defined by the member making a proposal is less than 2, then the member making the proposal already satisfies the threshold requirements (we assume their tacit support).

```rust
// fn propose in decl_module
if threshold < 2 {
	let total_members = Self::current_member().len() as MemberCount;
	let ok = proposal.dispatch(RawOrigin::Members(1, total_members).into()).is_ok();
}
```

In the above code, we set the second field of `RawOrigin::Members` to the total number of members in the DAO and the first field to the required threshold. The proposal is accordingly dispatched from the `RawOrigin` and any error is safely handled with [`.is_ok()`](https://doc.rust-lang.org/std/result/).

## Managing Membership <a name = "member"></a>

Runtime methods for most DAOs are permissioned. Indeed, even for the `join` method, we check that the requester is not already a member by verifying non-membership. Likewise, we include a runtime method `is_member` to easily verify membership or non-membership at the top of all runtime methods.

```rust
impl<T: Trait> Module<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		Self::current_member().iter()
			.any(|&ref a| a == who)
	}
}
```

At the top of runtime methods, we use `is_member` to check if the requester is a member or not. For example, the top of `vote`, we make sure that the voter is a member of the DAO.

```rust
// fn vote in decl_module
let voter = ensure_signed(origin)?;
ensure!(Self::is_member(&voter), "voter not a member");
```

The `mini-dao` has simple and permissionless `join` and `leave` methods that make it easy for anyone to participate in the DAO. The join function adds a new member by pushing the associated `AccountId` to the membership vector in runtime storage (in the `decl_storage` block).

```rust
// fn join in decl_module
<CurrentMember<T>>::mutate(|members| members.push(new_member.clone()));
```

To leave, the `exit` runtime method allows a member to remove their `AccountId` from the same vector.

```rust
// fn exit in decl_module
<CurrentMember<T>>::mutate(|members| members.retain(|x| x != &old_member));
```

[`retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain) allows you to remove elements by specifying a condition within the closure that must hold for all remaining elements (elements that uphold this condition are kept in the vector).

## Generic Proposals

In the `propose` runtime method, members submit proposals to the DAO. The submission includes the `threshold` for passage as a `#[compact]` field and the proposal is placed in a smart pointer (box). The function header looks like this

```rust
fn propose(origin, #[compact] threshold: MemberCount, proposal: Box<<T as Trait>::Proposal>)
```

[`#[compact]`](https://crates.parity.io/parity_codec/struct.Compact.html) is more space-efficient and less compute-efficient so it should be preferred for parameters that are not being heavily manipulated (~constants). 

We place a `Box<>` around `Proposal` because it is an [owned trait object](https://users.rust-lang.org/t/when-should-i-use-box-t/4411/2). By making it a smart pointer, we ensure that the pointer to the respective heap allocation is dropped once the proposal is no longer relevant.

After checking that the `proposer` is a member of the DAO, we take a hash of the proposal and first check that it does not already exist in the queue of proposals.

```rust
let proposal_hash = T::Hashing::hash_of(&proposal);
ensure!(!<ProposalOf<T>>::exists(proposal_hash), "proposal has already been added to the q");
```

The rest of the `propose` runtime method consists of storage changes for adding the proposal and instantiating the `ProposalState`. The `ProposalState` is used for tracking the progress of the proposal throughout the voting process. It is defined outside of the `decl_module` block.

```rust
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
```

We instantiate the required fields and insert the associated `ProposalState` object into the relevant storage item.

```rust
// fn propose in decl_module
let index = (Self::proposal_count() + 1) as ProposalIndex;
<ProposalCount<T>>::mutate(|count| *count + 1);
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
```

## 1 Member 1 Vote

Because anyone can join `mini-dao`, there isn't any [anti-sybil mechanism](https://en.wikipedia.org/wiki/Sybil_attack). There is no cost associated with each member's vote so permissionless membership implies the voting process can be manipulated -- anyone can create an arbitrary number of accounts to falsify support for their proposals.

After checking whether the voter is a member, the `vote` function verifies that the `proposal` has an associated `ProposalState` with a neat error handling pattern.

```rust
// fn vote in decl_module
let mut voting = Self::proposal_state(&proposalHash).ok_or("proposal must exist")?;
```

Thereafter, the `ProposalState` is used to identify the position of the voter's vote through the `.iter().position()` method.

```rust
// fn vote in decl_module
let position_yes = voting.ayes.iter().position(|a| a == &voter);
let position_no = voting.nays.iter().position(|a| a == &voter);
```

If the `vote` boolean fed as an input is `true`, then the appropriate branch of an `if` statement is executed. The basic logic of this branch consists of checking (1) if the vote in favor of the proposal has been made before and, if so, return a duplicate vote error (2) if the voter is switching their vote from a `no` to `yes`, then the position in the `nays` is removed.

```rust
// fn vote in decl_module
if position_yes.is_none() {
    voting.ayes.push(voter.clone());
} else {
    return Err("duplicate vote")
}
if let Some(pos) = position_no {
    voting.nays.swap_remove(pos);
}
```

The ability to change the vote at no cost is important to prevent bribery attacks in which participants can easily sell their votes once they've been committed. *See [here](http://hackingdistributed.com/2018/07/02/on-chain-vote-buying/) for more information with respect to on-chain vote buying.*

Next, the `ProposalState` is checked to see if conditions fo execution have been fulfilled. This consists of checking the tallies for `yes_votes` and `no_votes` before dispatching the proposal if the required approval threshold has been met.

```rust
// fn vote in decl_module
let yes_votes = voting.ayes.len() as MemberCount;
let no_votes = voting.nays.len() as MemberCount;
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
	}
}
```

If the proposal has been rejected by enough members to ensure that it will not reach the approval threshold, it is removed the proposal queue.

```rust
// fn vote in decl_module
<ProposalState<T>>::remove(&proposal_hash);
<Proposals<T>>::mutate(|props| props.retain(|x| x != &proposal_hash));
```

If the proposal has not been `approved` or `disapproved`, the `ProposalState` is updated with the member's vote.

```rust
<ProposalState<T>>::insert(&proposal_hash, voting);
```