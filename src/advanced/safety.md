# Safety First

Substrate's runtime is different in many ways from other development environments. Unlike conventional software development kits that abstract away low-level decisions, Substrate grants developer's close-to-the-metal, fine-grain control over the implementation. Increased flexibility should foster efficient, modular, and extensible applications. However, to quote the late Uncle Ben, *with great power comes great responsibility*.

Developers building with Substrate have to exercise incredible caution. Indeed, the the cost of err scales with the amount of capital secured by the application. To keep things simple at first, there are two base rules to keep in back of your mind when building with Substrate. These rules may not hold in every situation; Substrate offers unique optimization *in context*.

1. Modules should be independent pieces of code without unnecessary, significant external dependencies.
2. First Verify, Then Write

## First Verify, Then Write

Within each runtime module function, it is important to perform all checks prior to any storage changes. When coding on most smart contract platforms, the stakes are much lower because panics on contract calls will revert any storage changes. 

Conversely, Substrate requires much closer attention to detail because mid-function panics will persist any prior changes made to storage. **Substrate developers must write `ensure!` checks at the top of each runtime function's logic to verify that all of the requisite checks pass before performing any storage changes.**

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
    /// More business logic...
}
```

When a check needs to be made, but ownership of local variables does not need to be persisted, the developer should create a local scope to test the given variant before proceeding. An example of this pattern would be similar to [membership uniqueness](./unique.md) in which the given check is verified within closed scopes to minimize the persistence of `BTreeMap<T>`.