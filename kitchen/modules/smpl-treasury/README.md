# treasury module

This `smpl-treasury` started as a simpler version of the `treasury` module, but it has since veered from the treasury's design. While it demonstrates similar patterns, the `smpl-treasury` defines a module in which the users that would like to transfer funds must call the runtime method `request_transfer`:

```rust
// in decl_module {}
fn request_transfer(
    origin, 
    dest: T::AccountId, 
    amount: BalanceOf<T>
) -> Result {
    let sender = ensure_signed(origin)?;

    let bond = T::Tax::get();
    T::Currency::reserve(&sender, bond)
        .map_err(|_| "Must be able to pay tax to make transfer")?;
    
    let requested_spend = SpendRequest {
        from: sender.clone(),
        to: dest.clone(),
        amount: amount.clone(),
    };
    <TransferRequests<T>>::append(&[requested_spend])?;
    Self::deposit_event(RawEvent::TransferRequested(sender, dest, amount));
    Ok(())
}
```

This method reserves collateral from the sender before appending the `SpendRequest` to a runtime storage value.

```rust
// in decl_storage
TransferRequests get(fn treasury_requests): Vec<SpendRequest<T::AccountId, BalanceOf<T>>>;
```

This vector of spend requests contains the fields `from`, `to`, `amount`.

```rust
#[derive(Encode, Decode)]
pub struct SpendRequest<AccountId, Balance> {
    /// Sending account
    from: AccountId,
    /// Receiving account
    to: AccountId,
    /// Send amount
    amount: Balance,
}
```

In essence, this runtime queues requests before batching execution every `UserSpend` number of blocks. `UserSpend` is a [configurable module constant](https://substrate.dev/recipes/storage/constants.html). It is used in [`on_finalize`](https://github.com/substrate-developer-hub/recipes/blob/master/src/tour/loop.md)

```rust
fn on_finalize(n: T::BlockNumber) {
    if (n % T::UserSpend::get()).is_zero() {
        // every `UserSpend` number of blocks,
        // spend the funds according to member preferences
        Self::user_spend();
    }

    if (n % T::TreasurySpend::get()).is_zero() {
        Self::treasury_spend();
    }

}
```

If you looked closely in the `request_transfer` runtime method, a uniform fee called `T::Tax::get()` is taken from the sender for every transfer. It is reserved using `reserve` when the request is initially queued.

```rust
// in decl_module::request_transfer
let bond = T::Tax::get();
T::Currency::reserve(&sender, bond)
    .map_err(|_| "Must be able to pay tax to make transfer")?;
```

It is transferred to the module's pot when the transfer is executed.

```rust
let _ = T::Currency::transfer(&request.from, &request.to, request.amount, AllowDeath);
// get the tax
let tax_to_pay = T::Tax::get();
// unreserve the tax from the sender
T::Currency::unreserve(&request.from, tax_to_pay);
// pay the associated tax from the sender to the treasury account
let _ = T::Currency::transfer(&request.from, &Self::account_id(), tax_to_pay, AllowDeath);
```

This example demonstrates one pattern for expressing a shared account with substrate. The `ModuleId` is a constant.

```rust
const MODULE_ID: ModuleId = ModuleId(*b"example ");
```

This type comes from `sr-primitives`, which our `Cargo.toml` aliases as `runtime_primitives` so the import is

```rust
use runtime_primitives::ModuleId;
```

To convert this identity into an `AccountId`, it is necessary to also import `AccountIdConversion` from `sr-primitives::traits::AccountIdConversion`.

```rust
use runtime_primitives::traits::AccountIdConversion;
```

This is used to access the `ModuleId` associated with the module-driven governance with the `into_account` method

```rust
// in impl<T: Trait> Module<T>
pub fn account_id() -> T::AccountId {
    MODULE_ID.into_account()
}
```

In our module, a uniform `Tax` is sent to the `ModuleId` for every `SpendRequest` executed. With `ModuleId` identifier, it is straightforward to define runtime logic to govern scheduled spending from this account as well.

In our example, there is a `Council` storage item which is a list of `AccountId`s that are permitted to propose `Proposal`s for the treasury's spending. 

**In its current state, this runtime is terrifyingly insecure, but we can learn a lot by improving it, one step at a time.**

It allows every member to submit `Proposal`s for the treasury's spending. To vote on `Proposal`s, members need to call the `stupid_vote` runtime method with the `AccountId` of the member that made the `Proposal`. 

```rust
fn stupid_vote(
    origin,
    vote: T::AccountId,
) -> Result {
    let voter = ensure_signed(origin)?;
    ensure!(Self::is_on_council(&voter), "the voter is on the council");
    if let Some(mut proposal) = <Proposals<T>>::get(vote) {
        proposal.support += 1;
    } else {
        return Err("proposal associated with vote does not exist")
    }
    Ok(())
}
```

There is no limit on the number of times this method can be called by a member. So this voting algorithm is vulnerable to a sybil attack. It also doesn't check that the voter doesn't vote for themselves. Adding the latter check would only require placing another `ensure` statement at the top of the method body,

```rust
ensure!(&voter != &vote, "cannot vote on their own proposal");
```

There is a lot of research on anti-sybil mechanisms. In this case, the most straightforward solution is probably to separate things out into a `ProposalPeriod` and `VotingPeriod` and store some shared state on-chain which indicates which period it is in and how much time is left until the next period. These windows should not overlap and member actions should be restricted within each period.

In `ProposalPeriod`, members should be able to make proposals. Each member should only be able to make one proposal. There should be a simple way of pointing at another member's proposal such that the pointer is invalidated (and there is a default) if the proposal (being pointed at) is changed.

During the transition between the `ProposalPeriod` and `VotePeriod`, it would be convenient if duplicate proposals or the pointing construct described above could be merged to minimize required on-chain state bloat and complexity.

In `VotePeriod`, each member should be able to make only one vote. There should be a configurable role regarding whether or not a candidate can vote for their proposal. Each member should be able to change their vote as many times as requested before the end of the `VotePeriod`. 

## Other Possible Extensions
* design a system for electing a rotating council instead of the static vector of `AccountId`s; this would be a better model of how representative democracy is supposed to work for funding public infrastructure
* prioritize certain proposals/requests based on increased `support` levels or something else (using `sort` with an `ord` implementation that might weight certain parameters a certain way)
* implement quadratic signalling to separate the weighting algorithm from the voting process
* swap about `Currency` with `GenericAsset` such that this system exists to control the transfer of a specific asset `=>` consider use cases in which this might be useful like ticket issuance according to some on-chain defined set of rules; maybe posting collateral like kickback but how should this organize in terms of objects and relationships
