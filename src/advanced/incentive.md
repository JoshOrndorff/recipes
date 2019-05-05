# Incentive Design
> **Progress cannot always be monotonic because solutions to problems create new problems** -*Steven Pinker*

The first rule of incentive design is sometimes just *don't*. Just like in the real world, human coordination does not always require explicit mechanisms guided by fees and reward payouts. In more cases than not, the mutual benefits of an interaction drive coordination; adding an explicit fee structure may be redundant and unnecessary.

With that said, we haven't really scaled human coordination. Although the internet has facilitated communication across borders and timezones, the tragedy of the commons (aka free rider problem) still limits our ability to collaborate in groups larger than [primitive thresholds](http://www.lifewithalacrity.com/2008/09/group-threshold.html). If you're not immediately convinced, consider any (broken) system for provisioning public goods -- social security, humanitarian aid, immigration, education, or even blockchain infrastructure maintenance. **Despite a relative abundance of resources, we consistently struggle to coordinate efficient and meaningful allocation to those in need.**

By aligning stakeholder incentives through dynamic reward/fee structures, I believe blockchain can catalyze human coordination on an increasingly global scale. Now, let's talk incentive design.

* [Minimizing Hidden Costs](#hide)
* [Closed Incentive Loops](#closed)
* [Ex1. UTXO](#utxo)
* [Ex2. SunshineDAO](#sun)
* [Bonus: Dilution Safety for SunshineDAO](#dilute) 

## Minimizing Hidden Costs <a name = "hide"></a>

As Substrate developers, it is important to reflect on the *hidden costs* of our mechanisms and consider how to communicate these costs transparently to all active and potential users. Above all else, we do not want to create another system where the few that are familiar with the rules thrive while most struggle with basic interaction (*see modern capitalism*).

To not repeat the mistakes of the past, it is important for us to recognize instances of abusive mechanism design. For this we may look no further than [inflation](https://mises.org/library/how-central-banking-increased-inequality). The [Cantillon Effect](https://www.aier.org/article/sound-money-project/cantillon-effects-and-money-neutrality) describes the phenomenon wherein minting more currency benefits those that receive it first because market prices do not immediately reflect an increase in supply. 

> *The first recipient of the new supply of money is in the convenient position of being able to spend extra dollars before prices have increased. But whoever is last in line receives his share of new dollars after prices have increased. This is why when the Treasury’s deficit is monetized, inflation is referred to as a non-legislated tax. In these cases, the government has seized purchasing power (rather than physical bills) from its citizens without congressional approval.* ~ [Cantillon Effects and Monetary Neutrality](https://www.aier.org/article/sound-money-project/cantillon-effects-and-money-neutrality)

This does NOT mean that inflation or central banks are inherently *evil*, but rather that we need to be increasingly wary of the dilutive effects of minting new tokens/shares/currency. To be clear, the problem is NOT that inflation enforces a tax on public wealth by extracting purchasing power, but rather that this action is entirely swept under the rug as *neutral* monetary policy. **Hidden costs like inflation threaten the economic sovereignty of participants and discredit the mechanism's legitimacy.**

Transparency isn't a criteria just because it's *the right way* to do things. Hidden costs increase complexity and add [mental overhead](https://nakamotoinstitute.org/static/docs/micropayments-and-mental-transaction-costs.pdf), thereby limiting the diversity of users and rendering simulation increasingly unrealistic. 

<!-- **Should Go Through This Somewhere; maybe cache this and tweet storm it later?**
As an example, let's consider the scenario in which a DAO receives thousands of proposals to mint new shares in a short time period. While calculating expected dilution from the acceptance of a single proposal is not unbearable, the complexity blows up when users consider the probability of a proposal's acceptance in the context of all the other submitted proposals. To make such a system work, significant UI innovation must occur to make the hidden costs of inflation clear to DAO members(**`*`**)
* Even then, the inherent unpredictability of proposal acceptance makes it extremely unlikely that any coordination DAO will ever work...the proof goes something like this; let's say we have a system that *perfectly* predicts the probability of proposal passage; this system will influence the proposal passage, thereby changing the true probabilities in inherently unpredictable ways
* one system we could borrow from would be something like the Fed where guidance is given by the DAO members regarding the areas they would like to fund and how much funding they would like to provide. -->

## Closed Incentive Loop <a name = "closed"></a>

**To minimize hidden costs like inflation, we structure incentives as a closed loop in which fees cover rewards.**  

Not every system follows this rule; Bitcoin incentivizes the sustained security of the blockchain by minting new UTXOs to the *winning* miner upon commiting the discovered block. Ceteris paribus, this constant inflation deteriorates the purchasing power of other Bitcoin holders for every new block added to the chain, but, in practice, Bitcoin's price is *resistant* to changes in supply (at least in the short-term).

By designing our mechanisms as closed incentive loops, we opt for a [different](() "NOT best for every application") approach that prioritizes the economic sovereignty of passive actors. In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between transaction inputs and outputs are distributed evenly among the validator set. In [`SunshineDAO`](https://github.com/4meta5/SunshineDAO), we bond proposals for both the sponsor and the applicant to fund rewards for proposal processing. By paying for rewards with fees, neither of these examples rely on inflationary funding. This choice is preferrable as it minimizes the hidden costs to passive holders.

* [UTXO Closed Loop](#utxo)
* [SunshineDAO Closed Loop](#sun)

At the same time, not all mechanisms that incorporate closed incentive loops are exempt from inflationary risks. The most obvious example in [SunshineDAO](https://github.com/AmarRSingh/SunshineDAO) is the share dilution that occurs when *some* proposals are accepted and executed. As an example, grant proposals may request share ownership without committing stake above a transaction fee (the proposal bond). In this case, the minting of shares to satisfy the accepted proposal dilutes the ownership of existing members. Without making them explicitly aware of this possibility, the DAO risks volatility and collapse (mass member exit).

* [SunshineDAO Dilution Safety](#dilute)

## UTXO <a name = "utxo"></a>

Substrate developers need to **stay cognizant of the price paid for resource usage** within the runtime. This can be unintuitive for smart contract developers who are accustomed to interacting with a sandboxed execution environment like the EVM.

Indeed, Substrate is a bit more hands-on. When storage changes occur within a runtime function, they are not automatically reverted if the function panics thereafter. For this reason, it is imperative that any resource used by a transaction must explicitly be paid for within the module. For a more comprehensive explanation, check out [the tcr tutorial's best practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices): *If the resources used might be dependent on transaction parameters or pre-existing on-chain state, then your in-module fee structure must adapt accordingly.*

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

## SunshineDAO <a name = "sun"></a>

Sustained stakeholder interaction is nontrivial in a digital context (*see [AGP vote turnouts](https://forum.aragon.org/t/evaluating-the-agp-1-voting-results-makes-me-think-we-need-an-aragon-community-token-act/290)*). Although we will not cover voter incentives here, we will introduce a **closed fee structure designed to** 
1. mitigate proposal spam
2. remove stale entries

* trash analogy here
* we don't want to issue shares because we want to minimize hidden costs

* so get right into the code here...

* use `Currency` in lieu of importing `balances`

* bond patterns (from `SunshineDAO`)
* reserve, unreserve, transfer

* *an alternative bonding approach* is covered in scheduling collateralization
* what is preferrable? Good question to know the answer to...might depend on the DApp's assumptions

* awareness of bribery / collusive actor dynamics means that you have to think of the scenario in which the processer is the proposal sponsor -- do we want to make this not allowed? or would we prefer to make it so that this isn't advantageous!


## Bonus: Dilution Safety Mechanisms <a name = "dilute"></a>

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

We may be able to improve this design by noting how `TotalSharesRequested`, which represents the number of outstanding requested shares, provides a proxy for share issuance demand. Likewise, we could include it as an input for our `DilutionBound` mechanism. However, parameterization is already relatively confusing...