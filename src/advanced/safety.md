# Safety First

Unlike conventional software development kits that abstract away low-level decisions, Substrate grants developers fine-grain control over the underlying implementation. This approach fosters high-performance, modular applications. At the same time, it also demands increased attention from developers. To quote the [late Uncle Ben](https://knowyourmeme.com/memes/with-great-power-comes-great-responsibility), **with great power comes great responsibility**.

Indeed, Substrate developers have to exercise incredible caution. The bare-metal control that they maintain over the runtime logic introduces new attack vectors. In the context of blockchains, the cost of bugs scale with the amount of capital secured by the application. Likewise, developers should *generally* abide by a few rules when building with Substrate. These rules may not hold in every situation; Substrate offers optimization in context.

1. [Module Development Criteria](#criteria)
2. [Condition-Oriented Programming](#condition)
3. [Common Necessary Checks](#check)
4. [Logic Proofs](#qed)

## Module Development Criteria <a name = "criteria"></a>

1. Modules should be independent pieces of code; if your module is tied to many other modules, it should be a smart contract. See the [substrate-contracts-workshop](https://github.com/shawntabrizi/substrate-contracts-workshop) for more details with respect to smart contract programming on Substrate.

2. It should not be possible for your code to *panic* after storage changes. Poor error handling in Substrate can *brick* the blockchain, rendering it useless thereafter. With this in mind, developers need to follow a [declarative design pattern](https://www.tokendaily.co/blog/declarative-smart-contracts) in which checks are made at the top of function bodies before storage changes. This approach discourages unintended state changes, thereby facilitating auditability and better testing. In documentation, we refer to this pattern as declarative programming `<=>` [condition-oriented programming](#condition) `<=>` verify first, write last.

**Bonus Reading**
* [Condition-Oriented Programming](https://www.parity.io/condition-oriented-programming/)
* [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts)

## Condition-Oriented Programming <a name = "condition"></a>

Within each runtime module function, it is important to perform all checks prior to any storage changes. When coding on most smart contract platforms, the stakes are lower because panics on contract calls will revert any storage changes. Conversely, Substrate requires greater attention to detail because mid-function panics will persist any prior changes made to storage. 

**Substrate developers should use [`ensure!`](https://crates.parity.io/srml_support/macro.ensure.html) checks at the top of each runtime function's logic to verify that all of the requisite checks pass before performing any storage changes.** *Note that this is similar to [`require()`](https://ethereum.stackexchange.com/questions/15166/difference-between-require-and-assert-and-the-difference-between-revert-and-thro) checks at the top of function bodies in Solidity contracts.*

```rust
/// from Sunshine/runtime/src/dao.rs propose method
fn propose(origin, applicant: AccountId, shares: u32, tokenTribute: BalanceOf<T>) -> Result {
    let who = ensure_signed(origin)?;
    // check that the sender is a member of the DAO
    ensure!(Self::is_member(&who), "sponsor is not a member of Dao");

    // check that too many shares aren't requsted ( 100% is a high upper bound)
    ensure!(shares <= Self::total_shares(), "too many shares requested");

    // check that applicant doesn't have a pending application
    ensure!(!(Self::applicants::exists(&applicant)), "applicant has pending application");

    // check that the TokenTribute covers at least the `ProposalFee`
    ensure!(Self::proposal_fee() >= tokenTribute, 
    "The token tribute does not cover the applicant's required bond");
    /// storage changes start here...
}
```

When a check needs to be made, but ownership of locally declared variables does not need to be persisted, the developer should create a local scope to test the required variant before proceeding. An example of this pattern would be similar to [membership uniqueness](./unique.md) in which the given check is verified within closed scopes to minimize the persistence of `BTreeMap<T>`.

For more in-depth explanations of this pattern, see the relevant section in the [Shawn's Substrate Collectables tutorial](https://github.com/shawntabrizi/substrate-collectables-workshop/blob/master/3/buying-a-kitty.md#remember-verify-first-write-last) as well as [Gautam's Substrate Best Practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices). *This [github comment](https://github.com/shawntabrizi/substrate-collectables-workshop/pull/55#discussion_r258147961) is also very useful for visualizing the `verify first, write last` pattern in practice.*

## Common Necessary Checks <a name = "check"></a>

Here, we identify a few of the common checks made at the top of module function bodies.

* [Overflows/Underflows](#overunder)
* [Collision in Key-Value Maps](#collision)
* [Verifying Signed Messages](#signed)

### Checking for Overflows/Underflows <a name = "overunder"></a>

We can use the `checked` traits in [substrate-primitives](https://crates.parity.io/sr_primitives/traits/index.html) to protect against [overflow/underflow](https://medium.com/@taabishm2/integer-overflow-underflow-and-floating-point-imprecision-6ba869a99033) when we increment/decrement objects in our runtime. To follow the [Substrat collectable tutorial example](https://shawntabrizi.com/substrate-collectables-workshop/#/2/tracking-all-kitties?id=checking-for-overflowunderflow), we use [`checked_add()`](https://crates.parity.io/sr_primitives/traits/trait.CheckedAdd.html) to safely handle the possibility of overflow when incremementing a global counter. *Note that this is similar to [`SafeMath`](https://ethereumdev.io/safemath-protect-overflows/) in Solidity*. 

```rust
use runtime_primitives::traits::CheckedAdd;

let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;
```

[`ok_or()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) transforms an `Option` from `Some(value)` to `Ok(value)` or `None` to `Err(error)`. The [`?` operator](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html) facilitates error propagation. In this case, using `ok_or()` is the same as writing

```rust
let new_all_people_count = match all_people_count.checked_add(1) {
    Some (c) => c,
    None => return Err("Overflow adding a new person"),
};
```

### Collision in Key-Value Maps <a name = "collision"></a>

Often times we may intend for keys to be unique identifiers that map to a specific storage item. In this case, it is necessary to check for collisions before adding new entries. Before adding a new item to the mapping, we can check if the unique id already has an associated storage item.

```rust
ensure!(!<Value<T>>::exists(new_id), "This new id already exists");
```

### Verifying Signed Messages <a name = "signed"></a>

* [Checking for a Signed Message](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/storing-a-value?id=checking-for-a-signed-message)

Sometimes, we use this to verify set membership. In the context of SunshineDAO, 

```rust
/// going to add soon
```

## Logic Proofs <a name = "qed"></a>

Because Substrate grants bare-metal control to developers, certain code patterns can expose potential panics at runtime. Panics at runtime could

* prove with `.except()`
* look for examples in Substrate with the codefinder...
* look at Shawn's cryptokitties...
* [Adding Proofs for Unsafe and Panics](https://forum.parity.io/t/usage-of-unsafe-code/240)