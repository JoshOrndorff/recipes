## Dilution Safety Mechanism <a name = "dilute"></a>

Not all mechanisms that incorporate closed incentive loops are exempt from inflationary risks. The most obvious example in [SunshineDAO](https://github.com/AmarRSingh/SunshineDAO) is the share dilution that occurs when *some* proposals are accepted and executed. As an example, grant proposals may request share ownership without committing stake above a transaction fee (the proposal fee). In this case, the minting of shares to execute the accepted proposal dilutes the ownership of existing members. Without making them explicitly aware of this possibility, the DAO risks volatility and collapse (`=>` mass member exit).

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