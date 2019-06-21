# Incentive Design
> **Progress cannot always be monotonic because solutions to problems create new problems** -*Steven Pinker*

The first rule of incentive design is sometimes just *don't*. Human coordination does not always require explicit mechanisms guided by fees and reward payouts. In many cases, the mutual benefits of an interaction drive coordination; adding an explicit fee structure may be redundant and unnecessary.

With that said, we haven't *really* scaled human coordination. Although the internet has facilitated communication across borders and timezones, the tragedy of the commons (aka free rider problem) still limits our ability to collaborate in groups larger than [primitive thresholds](http://www.lifewithalacrity.com/2008/09/group-threshold.html). If you're not immediately convinced, consider any (broken) system for provisioning public goods -- social security, humanitarian aid, immigration, education, or even blockchain infrastructure maintenance. **Despite a relative abundance of resources, we consistently struggle to coordinate efficient and meaningful allocation to those in need.**

By aligning stakeholder incentives through dynamic reward/fee structures, blockchain will catalyze human coordination on an increasingly global scale. Now, let's talk incentive design.

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

## Closed Incentive Loop <a name = "closed"></a>

**To minimize hidden costs like inflation, we structure incentives as a closed loop in which fees cover rewards.**  

Not every system follows this rule; Bitcoin incentivizes the sustained security of the blockchain by minting new UTXOs to the *winning* miner upon committing the discovered block. Ceteris paribus, this constant inflation deteriorates the purchasing power of other Bitcoin holders for every new block added to the chain, but, in practice, Bitcoin's price is *resistant* to changes in supply (at least in the short-term).

By structuring mechanisms as closed incentive loops, we prioritize the economic sovereignty of passive actors. 

In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between transaction inputs and outputs are distributed evenly among the validator set. In [`SunshineDAO`](https://github.com/4meta5/SunshineDAO), we bond proposals for both the sponsor and the applicant to fund rewards for proposal processing. By paying for rewards with fees, neither of these examples rely on inflationary funding. This choice is preferrable as it minimizes the hidden costs to passive holders.

* [UTXO Closed Loop](#utxo)
* [SunshineDAO Closed Loop](#sun)

At the same time, not all mechanisms that incorporate closed incentive loops are exempt from inflationary risks. The most obvious example in [SunshineDAO](https://github.com/AmarRSingh/SunshineDAO) is the share dilution that occurs when *some* proposals are accepted and executed. As an example, grant proposals may request share ownership without committing stake above a transaction fee (the proposal fee). In this case, the minting of shares to execute the accepted proposal dilutes the ownership of existing members. Without making them explicitly aware of this possibility, the DAO risks volatility and collapse (`=>` mass member exit).

* [Dilution Safety Mechanism](#dilute)

## UTXO <a name = "utxo"></a>

Substrate developers need to **stay cognizant of the price paid for resource usage** within the runtime. This can be unintuitive for smart contract developers who are accustomed to interacting with a sandboxed execution environment like the EVM.

Indeed, Substrate is a bit more hands-on. When storage changes occur within a runtime function, they are not automatically reverted if the function panics thereafter. For this reason, it is imperative that any resource used by a transaction must explicitly be paid for within the module. For a more comprehensive explanation, check out [the tcr tutorial's best practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices): *If the resources used might be dependent on transaction parameters or pre-existing on-chain state, then your in-module fee structure must adapt accordingly.*

So how do we design a robust in-module fee structure? In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between inputs and outputs for valid transactions is distributed evenly among the authority set. This pattern demonstrates one approach for incentivizing validation via a floating transaction fee which varies in cost according to the value of the native currency and the relative size/activity of the validator set.

To properly incentivize the ecosystem's actors through the fee structure, the leftover value is distributed evenly among the authorities in the `spend_leftover` runtime function:

```rust
/// uxto-workshop/runtime/src/utxo.rs `decl_module{}`
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
> *Context*: SunshineDAO is a fund coordination DAO. Proposals to the DAO request share issuance and, optionally, stake capital. By burning shares, a member of the DAO redeems capital held by the DAO (in proportion to the number of shares they burn). For more information, see the [github](https://github.come/4meta5/SunshineDAO).

Imagine that our DAO has a pool of pending proposals. Each proposal requires sponsorship by a member of the DAO. As more proposals are submitted, the pool's size increases, and it becomes more difficult for voting members to keep track of the current state. To alleviate state bloat, stale proposals that have not passed in the defined voting period must be removed from the pool and successful proposals should be executed. Moreover, a cost should be attached to proposal submissions to prevent spam.

In summary, we need to guide actor incentives to
1. sponsor proposals
2. remove stale proposals
3. process successful proposals
4. mitigate proposal spam

Fortunately, we can use **(4)** to fund **(1)**, **(2)**, and **(3)**, thereby constructing a closed incentive loop. Specifically, we require a bond (~*collateral*) from both the applicant and the sponsoring member to submit a proposal. Within the `decl_module` block, we have a `propose` function which includes the following logic.

```rust
/// SunshineDAO/runtime/src/dao.rs `decl_module{}`
fn propose(origin, applicant: AccountId, shares: u32, tokenTribute: BalanceOf<T>) -> Result {
    let who = ensure_signed(origin)?;
    ensure!(Self::is_member(&who), "proposer is not a member of Dao");

    // reserve member's bond for proposal
    T::Currency::reserve(&who, Self::proposal_bond())
        .map_err(|_| "balance of proposer is too low")?;
    // reserve applicant's tokenTribute for proposal
    T::Currency::reserve(&applicant, tokenTribute)
        .map_err(|_| "balance of applicant is too low")?;
}
```

If the proposal is aborted within a short period after submission, then the bonds can be returned without penalty. 

```rust
/// SunshineDAO/runtime/src/dao.rs `decl_module{}`
fn abort(origin, hash: Vec<u8>) -> Result {
    // check that the abort is within the window
    ensure!(
        proposal.startTime + Self::abort_window() >= <system::Module<T>>::block_number(),
        "it is past the abort window"
    );

    // return the proposalBond back to the proposer because they aborted
    T::Currency::unreserve(&proposal.proposer, Self::proposal_bond());
    // and the tokenTribute (>= `proposalFee` amount) to the applicant
    T::Currency::unreserve(&proposal.applicant, proposal.tokenTribute);
}
```

However, if the proposal does not pass, then the bonds for both parties are transferred to the member that processes the proposal (thereby incentivizing **(2)** with our solution to **(4)**). 

```rust
/// SunshineDAO/runtime/src/dao.rs `decl_module{}`
fn process(origin, hash: Vec<u8>) -> Result {
    // IF NOT PASS
    // transfer the proposalBond back to the proposer
    T::Currency::unreserve(&proposal.proposer, Self::proposal_bond());
    // transfer proposer's proposal bond to the processer
    T::Currency::transfer(&proposal.proposer, &who, Self::proposal_bond());
    // return the applicant's tokenTribute
    T::Currency::unreserve(&proposal.applicant, proposal.tokenTribute);
    // transfer applicant's proposal fee to the processer
    T::Currency::transfer(&proposal.applicant, &who, Self::proposal_fee());
}
```

If the proposal passes, the sponsor's bond is returned, and the applicant's bond is split between the sponsor and the processing member (**(4)** => **(1)**, **(3)**).

```rust
/// SunshineDAO/runtime/src/dao.rs `decl_module{}`
fn process(origin, hash: Vec<u8>) -> Result {
    // IF PASS
    // transfer the proposalBond back to the proposer
    T::Currency::unreserve(&proposal.proposer, Self::proposal_bond());
    // and the applicant's tokenTribute
    T::Currency::unreserve(&proposal.applicant, proposal.tokenTribute);

    // split the proposal fee between the proposer and the processer
    let txfee = Self::proposal_fee().checked_mul(0.5);
    T::Currency::make_transfer(&proposal.applicant, &who, txfee);
    T::Currency::make_transfer(&proposal.applicant, &proposal.proposer, txfee);
}
```

The basic bonding pattern used in this example follows this pattern:
* Bonding stake => `T::Currency::reserve`
* Unbonding stake => `T::Currency::unreserve`
* Transferring bond => `T::Currency::make_transfer`

*NOTE: alternative to the `reserve => unreserve (=>) transfer` bonding pattern is introduced in [Scheduling Collateralization](./lock.md).*

## Dilution Safety Mechanism <a name = "dilute"></a>

*Instant withdrawals* protect DAO members from experiencing the outcome of proposals that they vehemently oppose. In the worst case scenario, a faction of the DAO that controls greater than the required threshold submits a proposal to grant themselves some ridiculous number of new shares, thereby diluting the shares of all other members (*[LOL](https://www.youtube.com/watch?v=Kk1sjbNcCxI)*). The instant withdrawal mechanism enables the minority faction under attack to exit the DAO while preserving their share ownership (because they can exit during the grace period before the proposal's execution). 

Let's consider a more common scenario in which a contentious vote results in the exit of the opposing half, thereby increasing the funding burden on the remaining DAO members. For our example, let's assume that the DAO maintains 100 outstanding shares backed by 100M DOTs. If a new share is requested without posting any additional stake, and the vote is split 50/50 with 100% voter turnout, the 50 shares that voted against the proposal could leave during the grace period. Under this scenario, the funding burden for the remaining members increases from 1% to 2% (1/100 to 1/50) in accordance with share dilution. 

In fact, a larger proposal can lead to a run on the DAO (mass member exit) under the previously presented scenario. With this scenario in mind, we place an additional constraint on proposals: **if a proposal's passage instigates member exit indicative of significant dilution, it shall not pass!** This protects the DAO against scenarios in which a contentious vote leaves a small subset of the DAO significantly diluted.

The invocation of this safety mechanism is predicated on the surrounding circumstances so we will halt any large proposal when mass member exit has made share issuance particularly dilutive. With this in mind, we can bound dilution for yes voters in the `vote` function. For every `yes` vote in which `approve == true`, we reset `proposal.maxVotes` to `total_shares` if `total_shares` has increased.

```rust
/// SunshineDAO/runtime/src/dao.rs `decl_module::vote()`
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
/// SunshineDAO/runtime/src/dao.rs `decl_module::process()` 
/// if dilution bound not satisfied, wait until there are more shares before passing
ensure!(
    Self::total_shares::get().checked_mul(Self::dilution_bound::get()) > proposal.maxVotes, 
    "Dilution bound not satisfied; wait until more shares to pass vote"
);
```

### Extension: Dynamic `dilution_bound` according to `TotalSharesRequested`

We may be able to improve this design by noting how the number of outstanding requested shares, `TotalSharesRequested`, provides a proxy for share issuance demand. Likewise, we could include it as an input for our `DilutionBound` mechanism. However, parameterization is already relatively confusing! We'll pause there for now -- file an issue or reach out if you have any clever ideas to formalize this without arbitrarily feeding mechanism complexity :)