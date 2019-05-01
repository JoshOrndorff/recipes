# DAO Proposal Invariants

*Progress cannot always be monotonic because solutions to problems create new problems.* - Steven Pinker

Modern DAO constructions are designed to better align long-term incentives while also providing a fast, safe withdrawal mechanism. These two properties are often referred to as *lock-in* and *instant withdrawals*:
1. **Lock-In**: Members that vote in favor of a proposal must stay in the DAO until the proposal is formally added to the DAO (and possibly even some time after)
2. **Instant Withdrawals**: Members that vote against a proposal that has passed or is passing can exit during a grace period during which the proposal's supporters are *locked-in*, thereby facilitating the exit

In the context of [SunshineDAO](https://github.com/AmarRSingh/SunshineDAO) (and [MolochDAO](https://github.com/MolochVentures/moloch)), each proposal constitutes a request for minting new shares in exchange for staking some capital. If the proposal is a grant application, then it will not offer significant capital to stake, but instead request share ownership to burn for the purposes of the grant. DAO members that vote against the grant proposal are allowed to exit before the dilution (caused by minting new shares for the grant recipient).

* By allowing Guild members to ragequit and exit at any time, Moloch protects its members from 51% attacks and from supporting proposals they vehemently oppose.
* In the worst case, one or more Guild members who control >50% of the shares could submit a proposal to grant themselves a ridiculous number of new shares, thereby diluting all other members of their claims to the Guild Bank assets and effectively stealing from them. If this were to happen, everyone else would ragequit during the grace period and take their share of the Guild Bank assets with them, and the proposal would have no impact.
* In the more likely case of a contentious vote, those who oppose strongly enough can leave and increase the funding burden on those who choose to stay. Let's say the Guild has 100 outstanding shares and $100M worth of ETH in the Guild Bank. If a project proposal requests 1 newly minted share (~$1M worth), the vote is split 50/50 with 100% voter turnout, and the 50 who voted No all ragequit and take their $50M with them, then the remaining members would be diluting themselves twice as much: 1/51 = ~2% vs. 1/101 = ~1%.
* In this fashion, the ragequit mechanism also provides an interesting incentive in favor of Guild cohesion. Guild members are disincentivized from voting Yes on proposals that they believe will make other members ragequit. Those who do vote Yes on contentious proposals will be forced to additionally dilute themselves proportional to the fraction of Voting Shares that ragequit in response.

<!-- Plasma uses fraud proofs for its exit; Cosmos also maintains instant withdrawals and in-protocol delegation... -->

## Dilution Safety Mechanisms

*Instant withdrawals* protect members from experiencing the outcome of proposals that they vehemently oppose. In the worst case scenario, a faction of the DAO that controls greater than the required threshold submits a proposal to grant themselves some ridiculous number of new shares, thereby diluting the shares of all other members (I've seen this attack [before](https://www.youtube.com/watch?v=Kk1sjbNcCxI)). The *instant withdrawal* mechanism enables the faction under attack to exit the DAO while preserving their share ownership (because they can exit during the grace period before the proposal's execution). 

Let's consider a more common scenario in which a contentious vote results in the exit of the opposing half, thereby increasing the funding burden on the remaining DAO members. For our example, let's assume that the DAO maintains 100 outstanding shares backed by 100M DOTs. If a new share is requested without posting any additional stake, and the vote is split 50/50 with 100% voter turnout, the 50 that voted against the proposal could leave during the grace period. Under this scenario, the funding burden for the remaining members increases from 1% to 2% (1/100 to 1/50) in accordance with share dilution. 

In fact, a larger proposal can lead to a run on the DAO (mass exit paranoia) under the previously presented scenario. With this nsigma event in mind, we place an additional constraint on proposals: **if a proposal's passage instigates member exit indicative of significant dilution, it shall not pass!** This protects the DAO against scenarios in which a contentious vote leaves a small subset of the DAO significantly diluted.

How is causation determined between the proposal's passage and member exodus? Actually, our actions are predicated on the surrounding circumstances so we will halt any large proposal when mass member exit has made share issuance particularly dilutive. With this in mind, we can bound dilution for yes voters in the `vote` function. For every `yes` vote in which `approve == true`, we reset `proposal.maxVotes` to `total_shares` if `total_shares` has increased.

```rust
if approve {
    Self::voters_for::mutate(hash, |voters| voters.push(&who));
    Self::proposals_for::mutate(&who, |props| props.push(hash));
    Self::voter_id::mutate(hash, |voters| voters.push(&who));
    Self::vote_of::insert(&(hash, who), true);

    // to bound dilution for yes votes
    if Self::total_shares::get() > proposal.maxVotes {
        proposal.maxVotes = Self::total_shares::get();
    }
    proposal.yesVotes += Self::member_shares(&who);

} else {
    proposal.noVotes += Self::member_shares(&who);
}
```

With this logic placed in `vote`, we can safeguard against aggressive dilution by preventing the passage of proposals under egregious situations:

```rust
/// in `process`
// if dilution bound not satisfied, wait until there are more shares before passing
ensure!(
    Self::total_shares::get().checked_mul(Self::dilution_bound::get()) > proposal.maxVotes, 
    "Dilution bound not satisfied; wait until more shares to pass vote"
);
```

We may be able to improve this design by noting how `TotalSharesRequested`, which represents the number of outstanding requested shares, provides a proxy for share issuance demand. Likewise, we could include it as an input for our `DilutionBound` mechanism. However, parameterization is already relatively confusing!

## Balancing Instant Withdrawals and Lock-In Voting

The second limitation comes from the clash between *instant withdrawals* and *lock in*. To uphold both, it is necessary to set the condition that no exiting member may support any pending proposals. If a member supports a single pending proposal, then lock-in is violated and incentives are disaligned (=> opens an attack vector in which the member supports a malicious proposal before leaving the DAO unscathed).

* how do we prove this variant is upheld throughout the code
* need to set up tests to ensure that this invariant is held forever hereafter; how can this be done in a way that limits conditional paths

*Problem*
* need to


## Lock-In vs Instant Withdrawal

* MolochDAO's ragequit
* mechanism design is using a proposal queue to enforce some invariant

* alternative is to enforce the variant manually
* how do we unit test to ensure the variant is enforced (what verification techniques can we use)

* as a general implementation rule, start with a simple functional working product and then iterate with increased complexity

In more ways than one, the logic for coordinating action within a DAO mirrors Proof of Stake consensus mechanisms

* the proposal process
    * **the grace period is for instant withdrawals which are important for security (the dao -> plasma chains)**
    * open problem: implementing a fast, asynchronous proposal algorithm that does not suffer from head of line blocking but also ensures against *malicious exits* (exits by validators that voted yes on a pending proposal)
        * consider incorporating fraud proofs into this scheme? Used in plasma for sure as well as asynchronous consensus algorithms
        * futures as an optimization to prevent head-of-line blocking for the proposal process

## RANDOM THOUGHTS

Proposals are just extrinsics and they should just be made as generic as possible to allow for their use in a very modular fashion, but how can we define them to take shape, gain shape, and then gradually become more abstract as well. Shades of formalization?