# Incentive Design
> this introduction is *rough*

The first rule of incentive design is sometimes just *don't*...Just like in the real world, human coordination does not always require explicit incentive mechanisms guided by fees and reward payouts. In more cases than not, the mutual benefits of an interaction drive coordination; adding an explicit fee structure may be redundant and unnecessary for these cases.

In all honesty, it seems to me that modern human coordination is often still limited by primitive [group thresholds](http://www.lifewithalacrity.com/2008/09/group-threshold.html). We still operate through intermediate entities. National and state laws still limit our ability to coordinate on an increasingly global scale. 

By aligning stakeholder incentives through dynamic reward/fee structures, I think blockchain can foster significant innovation as it pertains to humanity's coordination. No, blockchain has not officially solved the tragedy of the commons, but I still maintain an unwavering, naive optimism that, some day, a mechanism built on a blockchain will alleviate some free rider problem. 

* [Minimizing Hidden Costs](#hide)
* [Dilution Safety Mechanism](#dilute) 
* [Closed Incentive Loops](#closed) _________________
* [Example: SunshineDAO](#sun) _________________
* [Example: UTXO](#utxo) ___

## Minimizing Hidden Costs <a name = "hide"></a>

Between now and then, I think it's important for us to constantly keep *accessibility* in mind as a design criterion. Above all else, we should reflect on the *hidden costs* of our mechanism and consider how to communicate these costs transparently to all active and potential users.

The most obvious example in [SunshineDAO](https://github.com/AmarRSingh/SunshineDAO) is the share dilution that occurs when *some* proposals are accepted and executed. As an example, grant proposals may request share ownership without committing stake above a transaction fee (the proposal bond). In this case, the minting of shares to satisfy the accepted proposal would dilute existing members. Without making them explicitly aware of this possibility, the DAO risks volatility and collapse (mass member exit). Therefore, 

* picture for inflation


* This is the most obvious source of inflation, but if we create an open incentive structure, then we could mint new shares as a reward for DAO actors, but that would also act as a cost to all other members and this complexity is not at all transparent...

## Dilution Safety Mechanisms <a name = "dilute"></a>

*Instant withdrawals* protect DAO members from experiencing the outcome of proposals that they vehemently oppose. In the worst case scenario, a faction of the DAO that controls greater than the required threshold submits a proposal to grant themselves some ridiculous number of new shares, thereby diluting the shares of all other members (I've seen this attack [before](https://www.youtube.com/watch?v=Kk1sjbNcCxI)). The *instant withdrawal* mechanism enables the minority faction under attack to exit the DAO while preserving their share ownership (because they can exit during the grace period before the proposal's execution). 

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

With this logic placed in `vote`, we can safeguard against aggressive dilution by preventing the passage of proposals that are particularly contentious:

```rust
/// in `process`
// if dilution bound not satisfied, wait until there are more shares before passing
ensure!(
    Self::total_shares::get().checked_mul(Self::dilution_bound::get()) > proposal.maxVotes, 
    "Dilution bound not satisfied; wait until more shares to pass vote"
);
```

We may be able to improve this design by noting how `TotalSharesRequested`, which represents the number of outstanding requested shares, provides a proxy for share issuance demand. Likewise, we could include it as an input for our `DilutionBound` mechanism. However, parameterization is already relatively confusing!

### SunshineDAO Closed Incentive Loop <a name = "sun"></a>

Sustained stakeholder interaction is nontrivial in a digital context (*see [AGP vote turnouts](https://forum.aragon.org/t/evaluating-the-agp-1-voting-results-makes-me-think-we-need-an-aragon-community-token-act/290)*). Although we will not cover voter incentives here, we will introduce a **closed fee structure designed to** 
1. mitigate proposal spam
2. remove stale entries

* trash analogy here...we don't want to issue shares because we want to minimize hidden costs

* use `Currency` in lieu of importing `balances`

* bond patterns (from `SunshineDAO`)
* reserve, unreserve, transfer

* awareness of bribery / collusive actor dynamics means that you have to think of the scenario in which the processer is the proposal sponsor -- do we want to make this not allowed? or would we prefer to make it so that this isn't advantageous!


### UTXO Processing, A Robust In-Module Fee Structure <a name = "utxo"></a>

Substrate developers need to **stay cognizant of the price paid for resource usage** within the runtime. This can be unintuitive for former smart contract developers previously abstracting out the cost of individual operations and relying on conditional reversion.

Indeed, Substrate is a bit more hands-on. When storage changes occur within a runtime function, they are not automatically reverted if the function panics thereafter. For this reason, it is imperative that any resource used by a transaction must explicitly be paid for within the module. For more details, check out [the tcr tutorial's best practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices): *If the resources used might be dependent on transaction parameters or pre-existing on-chain state, then your in-module fee structure must adapt accordingly.*

So how do we design a robust in-module fee structure? In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between inputs and outputs for valid transactions is distributed evenly among the authority set. This pattern demonstrates one approach for incentivizing validation via a floating transaction fee which varies in cost according to the value of the native currency and the relative size/activity of the validator set.

To properly incentivize the ecosystem's actors through the fee structure, the leftover value is distributed evenly among the authorities in the `spend_leftover` runtime function:

```rust
/// Redistribute combined leftover value evenly among chain authorities
fn spend_leftover(authorities: &[H256]) {
    let leftover = <LeftoverTotal<T>>::take();
    let share_value: Value = leftover
        .checked_div(authorities.len() as Value)
        .ok_or("No authorities")
        .unwrap();
    if share_value == 0 { return }

    let remainder = leftover
        .checked_sub(share_value * authorities.len() as Value)
        .ok_or("Sub underflow")
        .unwrap();
    <LeftoverTotal<T>>::put(remainder as Value);

    for authority in authorities {
        let utxo = TransactionOutput {
            value: share_value,
            pubkey: *authority,
            salt: <system::Module<T>>::block_number().as_(),
        };

        let hash = BlakeTwo256::hash_of(&utxo);

        if !<UnspentOutputs<T>>::exists(hash) {
            <UnspentOutputs<T>>::insert(hash, utxo);
            runtime_io::print("leftover share sent to");
            runtime_io::print(hash.as_fixed_bytes() as &[u8]);
        } else {
            runtime_io::print("leftover share wasted due to hash collision");
        }
    }
}
```

## Metagovernance of Dilution Bound and Other Margin Metrics

* governing all of these parameters in a consummable way
* explaining every choice and how to make it differently
* we need to audit every mechanism design decision

* Transaction Fees and How They Flow in a Closed System
* Dilution Bound?