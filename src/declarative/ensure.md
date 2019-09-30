# Condition Oriented Programming

Within each runtime module function, it is important to perform requisite checks prior to any storage changes. Unlike existing smart contract platforms, Substrate requires greater attention to detail because mid-function panics will persist any prior changes made to storage.

**Place [`ensure!`](https://crates.parity.io/srml_support/macro.ensure.html) checks at the top of each runtime function's logic to verify that all requisite conditions are met before performing any storage changes.** *Note that this is similar to [`require()`](https://ethereum.stackexchange.com/questions/15166/difference-between-require-and-assert-and-the-difference-between-revert-and-thro) checks at the top of function bodies in Solidity contracts.*

In the [set storage and iteration](../storage/iterate.md), a vector was stored in the runtime to allow for simple membership checks for methods only available to members. 

```rust
decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
        Members get(members): Vec<T::AccountId>;
	}
}
...
impl<T: Trait> Module<T> {
    fn is_member(who: &T::AccountId) -> bool {
        <Members<T>>::get().contains(who)
    }
}
```


"*By returning `bool`, we can easily use these methods in `ensure!` statements to verify relevant state conditions before making requests in the main runtime methods.*" 

```rust
fn member_action(origin) -> Result {
    let member = ensure_signed(origin)?;
    ensure!(Self::is_member(&member), "not a member => cannot do action");
    // <action && || storage change>
    Ok(())
}
```

Indeed, this pattern of extracting runtime checks into separate functions and invoking the `ensure` macro in their place is useful. It produces readable code and encourages targeted testing to more easily identify the source of logic errors.

*This [github comment](https://github.com/shawntabrizi/substrate-collectables-workshop/pull/55#discussion_r258147961) might help when visualizing declarative patterns in practice.*

**Bonus Reading**
* [Design for Testability](https://blog.nelhage.com/2016/03/design-for-testability/)
* [Condition-Oriented Programming](https://www.parity.io/condition-oriented-programming/)
* [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts)