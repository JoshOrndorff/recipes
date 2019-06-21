# Declarative Programming

**NOTE**: *this recipe pairs well with [Robust Path Handling](./paths.md)*

Within each runtime module function, it is important to perform all checks prior to any storage changes. When coding on most smart contract platforms, the stakes are lower because panics on contract calls will revert any storage changes. Conversely, Substrate requires greater attention to detail because mid-function panics will persist any prior changes made to storage. 

**Substrate developers should use [`ensure!`](https://crates.parity.io/srml_support/macro.ensure.html) checks at the top of each runtime function's logic to verify that all of the requisite checks pass before performing any storage changes.** *Note that this is similar to [`require()`](https://ethereum.stackexchange.com/questions/15166/difference-between-require-and-assert-and-the-difference-between-revert-and-thro) checks at the top of function bodies in Solidity contracts.*

```rust
/// from Sunshine/runtime/src/dao.rs propose method
fn propose(origin, applicant: AccountId, shares: u32, tokenTribute: BalanceOf<T>) -> Result {
    let who = ensure_signed(origin)?;
    // check that the sender is a member of the DAO
    ensure!(Self::is_member(&who), "sponsor is not a member of Dao");

    // check that too many shares aren't requested (100% is a high upper bound)
    ensure!(shares <= Self::total_shares(), "too many shares requested");

    // check that applicant doesn't have a pending application
    ensure!(!(Self::applicants::exists(&applicant)), "applicant has pending application");

    // check that the TokenTribute covers at least the `ProposalFee`
    ensure!(Self::proposal_fee() >= tokenTribute, 
            "The token tribute does not cover the applicant's required bond");
    /// storage changes start here...
}
```

When a check needs to be made, but ownership of locally declared variables does not need to be persisted, the developer should create a local scope to test the required variant before proceeding. An example of this pattern is how the [membership uniqueness](../proto/unique.md) recipe verifies the nonexistence of duplicate UTXOs within closed scopes to minimize the persistence of `BTreeMap<T>`.

For more in-depth explanations of this pattern, see the relevant section in the [Substrate Collectables tutorial](https://github.com/shawntabrizi/substrate-collectables-workshop/blob/master/3/buying-a-kitty.md#remember-verify-first-write-last) as well as [Substrate Best Practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices). *This [github comment](https://github.com/shawntabrizi/substrate-collectables-workshop/pull/55#discussion_r258147961) is also very useful for visualizing the `verify first, write last` pattern in practice.*

**Bonus Reading**
* [Condition-Oriented Programming](https://www.parity.io/condition-oriented-programming/)
* [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts)

### Verifying Signed Messages

It is often useful to designate some functions as permissioned and, therefore, accessible only by a defined group. In this case, we check that the transaction that invokes the runtime function is signed before verifying that the signature corresponds to a member of the permissioned set. In [SunshineDAO](https://github.com/4meta5/SunshineDAO), all of the runtime module functions can only be called by members of the DAO. At the top of every runtime module function, the following check is included.

```rust
let who = ensure_signed(origin)?;
ensure!(Self::is_member(&who), "sponsor is not a member of Dao");
```

To read more about checking for signed messages, see the relevant section in the [Substrate collectables tutorial](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/storing-a-value?id=checking-for-a-signed-message).