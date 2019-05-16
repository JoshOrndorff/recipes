# Safety First

Unlike conventional software development kits that abstract away low-level decisions, Substrate grants developers fine-grain control over the underlying implementation. This approach fosters high-performance, modular applications. At the same time, it also demands increased attention from developers. To quote the [late Uncle Ben](https://knowyourmeme.com/memes/with-great-power-comes-great-responsibility), **with great power comes great responsibility**.

Indeed, Substrate developers have to exercise incredible caution. The bare-metal control that they maintain over the runtime logic introduces new attack vectors. In the context of blockchains, the cost of bugs scale with the amount of capital secured by the application. Likewise, developers should *generally* abide by a few rules when building with Substrate. These rules may not hold in every situation; Substrate offers optimization in context.

1. [Module Development Criteria](#criteria)
2. [Condition-Oriented Programming](#condition)
3. [Common Necessary Checks](#check)
4. [Logic Proofs](#qed)

## Module Development Criteria <a name = "criteria"></a>

1. Modules should be independent pieces of code; if your module is tied to many other modules, it should be a smart contract. See the [substrate-contracts-workshop](https://github.com/shawntabrizi/substrate-contracts-workshop) for more details with respect to smart contract programming on Substrate.

2. It should not be possible for your code to panic after storage changes. Poor error handling in Substrate can *brick* the blockchain, rendering it useless thereafter. With this in mind, developers need to follow a [declarative design pattern](https://www.tokendaily.co/blog/declarative-smart-contracts) in which checks are made at the top of function bodies before storage changes. This approach discourages unintended state changes, thereby facilitating auditability and better testing. In documentation, we refer to this pattern as declarative programming `<=>` [condition-oriented programming](#condition) `<=>` verify first, write last.

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

When a check needs to be made, but ownership of locally declared variables does not need to be persisted, the developer should create a local scope to test the required variant before proceeding. An example of this pattern is how the [membership uniqueness](./unique.md) recipe verifies the nonexistence of duplicate UTXOs within closed scopes to minimize the persistence of `BTreeMap<T>`.

For more in-depth explanations of this pattern, see the relevant section in the [Substrate Collectables tutorial](https://github.com/shawntabrizi/substrate-collectables-workshop/blob/master/3/buying-a-kitty.md#remember-verify-first-write-last) as well as [Substrate Best Practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices). *This [github comment](https://github.com/shawntabrizi/substrate-collectables-workshop/pull/55#discussion_r258147961) is also very useful for visualizing the `verify first, write last` pattern in practice.*

## Common Necessary Checks <a name = "check"></a>

There are a few common checks made at the top of module function bodies.

* [Overflows/Underflows](#overunder)
* [Collision in Key-Value Maps](#collision)
* [Verifying Signed Messages](#signed)

### Checking for Overflows/Underflows <a name = "overunder"></a>

We can use the `checked` traits in [substrate-primitives](https://crates.parity.io/sr_primitives/traits/index.html) to protect against [overflow/underflow](https://medium.com/@taabishm2/integer-overflow-underflow-and-floating-point-imprecision-6ba869a99033) when incrementing/decrementing objects in our runtime. To follow the [Substrat collectable tutorial example](https://shawntabrizi.com/substrate-collectables-workshop/#/2/tracking-all-kitties?id=checking-for-overflowunderflow), we use [`checked_add()`](https://crates.parity.io/sr_primitives/traits/trait.CheckedAdd.html) to safely handle the possibility of overflow when incremementing a global counter. *Note that this check is similar to [`SafeMath`](https://ethereumdev.io/safemath-protect-overflows/) in Solidity*. 

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

In [SunshineDAO](https://github.com/4meta5/SunshineDAO), we use the hash of a proposal as the unique identifier in a `Proposals` map in the `decl_storage` block. Before adding a new proposal to the `Proposals` map, we check that the hash doesn't already have an associated value in the map. If it does, we do not allow subsequent storage changes because this would cause a key collision.

```rust
/// decl_module{} in runtime/src/dao.rs
fn propose(origin, applicant: AccountId, shares: u32, tokenTribute: BalanceOf<T>) -> Result {
    // check that a proposal associated with the given key does not already exist in the map
	ensure!(!(Self::proposals::exists(&prop.base_hash)), "Key collision :(");
    // .. more checks

    //add proposal
	Self::proposals::insert(prop.base_hash, prop);
}
```

For another example, see how the [Substrate collectables tutorial](https://shawntabrizi.com/substrate-collectables-workshop/#/2/generating-random-data?id=checking-for-collision) covers this pattern.

### Verifying Signed Messages <a name = "signed"></a>

It is often useful to designate some functions as permissioned and, therefore, accessible only by a defined group. In this case, we check that the transaction that invokes the runtime function is signed before verifying that the signature corresponds to a member of the permissioned set. In [SunshineDAO](https://github.com/4meta5/SunshineDAO), all of the runtime module functions can only be called by members of the DAO. At the top of every runtime module function, the following check is included.

```rust
let who = ensure_signed(origin)?;
ensure!(Self::is_member(&who), "sponsor is not a member of Dao");
```

To read more about checking for signed messages, see the relevant section in the [Substrate collectables tutorial](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/storing-a-value?id=checking-for-a-signed-message).

## Logic Proofs <a name = "qed"></a>

Because Substrate grants bare-metal control to developers, certain code patterns can expose panics at runtime. As mentioned in (2) of [Module Development Criteria](#criteria), panics can cause irreversible storage changes, possibly even bricking the blockchain and rendering it useless. 

It is the responsibility of Substrate developers to ensure that the code doesn't panics after storage changes. In many cases, safety might be independently verified by the developer while writing the code. To facilitate auditability and better testing, Substrate developers should include a proof in an `.expect()` call that shows why the code's logic is safe and will not panic. Convention dictates formatting the call like so

```rust
<Object<T>>::method_call().expect("<proof of safety>; qed");
```

You can find more examples of this pattern in the [Substrate codebase](https://github.com/paritytech/substrate/search?q=expect). Indeed, including logic proofs is very important for writing readable, well-maintained code. It comes as no surprise that this pattern is also discussed in the [Substrate collectables tutorial](https://shawntabrizi.com/substrate-collectables-workshop/#/3/buying-a-kitty?id=remember-quotverify-first-write-lastquot).

> *QED stands for Quod Erat Demonstrandum which loosely translated means "that which was to be demonstrated"*