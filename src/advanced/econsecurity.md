# Economic Security

Substrate's runtime is different in many ways from other conventional development environments. Rather than opting for safety by limiting the flexibility of the underlying implementation, Substrate grants developer's low-level, fine-grain control over implementation details. Although this increased flexibility allows for efficient, modular, and extensible applications, it also places more responsibility on developers to verify the safety of their logic.

With this in mind, there are a few informal rules that developers should follow when building with Substrate. These rules do not hold for every situation; Substrate's bare metal abstractions are designed to foster optimization in context.

1. Modules should be independent pieces of code without unnecessary, significant external dependencies.
2. [First Verify, Then Write](#fw)
3. [Robust In-Module Fee Structure](#robust)
4. [History](#history)

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

## Robust In-Module Fee Structure <a name = "robust"></a>

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

## History <a name = "history"></a>

In late June 2016, Gavin Wood published a post on [Condition-Oriented Programming (COP)](https://www.parity.io/condition-oriented-programming/), a hybrid approach between functional and imperative programming. Put simply, COP aims to ensure that function bodies have no conditional paths or, alternatively, never mix transitions with conditions. By discouraging conditional paths from state-transitions, this approach limits the complexity of state-transitions, thereby allowing for facilitated auditability and better testing. 

More than two years later, James Prestwich published [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts) reiterating the necessity of a *functional* approach to smart contract code patterns. In this post, Prestwich cites that "declarative contracts align the structure of the contract implementation with the reality of the chain by defining exactly what state modifications are permissible, and letting the user modify state directly. Declarative contracts prevent unintended state changes." 

As we continue to explore the environment for developing, we will hopefully continue to extract useful patterns for efficiently managing conditional paths in the context of Substrate development.