# Economic Security

Substrate's runtime is different in many ways from other conventional development environments. Rather than opting for safety by limiting the flexibility of the underlying implementation, Substrate grants developer's low-level, fine-grain control over implementation details. Although this increased flexibility allows for efficient, modular, and extensible applications, it also places more responsibility on developers to verify the safety of their logic.

With this in mind, there are a few informal rules that developers should follow when building with Substrate. These rules do not hold for every situation; Substrate's bare metal abstractions are designed to foster optimization in context.
1. Modules should be independent pieces of code without unnecessary, significant external dependencies.
2. [First Verify, Then Write](#fw)

Substrate developers also need to be especially cognizent economic security. Unlike smart contract platforms, Substrate does not add the gas abstraction for fees and it is therefore up to the Substrate developer to incorporate a robust in-module fee structure when appropriate.
1. [UTXO Processing, A Robust In-Module Fee Structure](#utxo)
2. [Dilution Safety Mechanism](#dilution)

## First Verify, Then Write <a name = "fw"></a>

Within each runtime module function, it is important to perform all checks prior to any storage changes. When coding on most smart contract platforms, the stakes are much lower because panics on contract calls will revert any storage changes. 

Conversely, Substrate requires much closer attention to detail because mid-function panics will persist any prior changes made to storage. With this in mind, Substrate developers are encouraged to write `ensure!` checks at the top of each runtime function's logic to verify any necessary conditions prior to the consequent changes to storage. It is recommended to verify that all of these necessary checks pass before performing any storage changes.

```rust
/// from Sunshine/runtime/src/dao.rs propose method
fn propose(origin, applicant: AccountId, shares: u32, tokenTribute: BalanceOf<T>) -> Result {
    let who = ensure_signed(origin)?;
    ensure!(Self::is_member(&who), "proposer is not a member of Dao");

    // check that too many shares aren't requsted ( 100% is a high upper bound)
    ensure!(shares <= Self::total_shares(), "too many shares requested");

    // check that applicant doesn't have a pending application
    ensure!(!(Self::applicants::exists(&applicant)), "applicant has pending application");

    // check that the TokenTribute covers at least the `ProposalFee`
    ensure!(Self::proposal_fee() >= tokenTribute, 
    "The token tribute does not cover the applicant's required bond");
    /// Other logic...
}
```

When a check needs to be made, but ownership of local variables does not need to be persisted, the developer should create a local scope to test the given variant before proceeding. An example of this pattern would be similar to [membership uniqueness](./unique.md) in which the given check is verified within closed scopes to minimize the persistence of `BTreeMap<T>` for example.

## UTXO Processing, A Robust In-Module Fee Structure <a name = "utxo"></a>

Substrate developers need to **stay cognizant of the price paid for resource usage** within the runtime. This can be unintuitive for former smart contract developers previously abstracting out the cost of individual operations and relying on conditional reversion.

Substrate is a bit more hands-on. When storage changes occur within a runtime function, they are not automatically reverted if the function panics thereafter. For this reason, it is imperative that any resource used by a transaction must explicitly be paid for within the module. For more details, check out [the tcr tutorial's best practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices): *If the resources used might be dependent on transaction parameters or pre-existing on-chain state, then your in-module fee structure must adapt accordingly.*

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

## Dilution Safety Mechanisms <a name = "dilution"></a>

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