# Declarative Programming

Within each runtime module function, it is important to perform all checks prior to any storage changes. When coding on most smart contract platforms, the stakes are lower because panics on contract calls will revert any storage changes. Conversely, Substrate requires greater attention to detail because mid-function panics will persist any prior changes made to storage.

* [Using the Ensure Macro](#ensure)
* [Verifying Signed Messages](#verify)
* [Checking for Collisions](#collide)

## Using the Ensure Macro <a name = "ensure"></a>

**Substrate developers should use [`ensure!`](https://crates.parity.io/srml_support/macro.ensure.html) checks at the top of each runtime function's logic to verify that all of the requisite checks pass before performing any storage changes.** *Note that this is similar to [`require()`](https://ethereum.stackexchange.com/questions/15166/difference-between-require-and-assert-and-the-difference-between-revert-and-thro) checks at the top of function bodies in Solidity contracts.*

The [Social Network](../storage/social.md#naive) recipe demonstrated how we can create separate runtime methods to verify necessary conditions in the main methods.

```rust
impl<T: Trait> Module<T> {
  pub fn friend_exists(current: T::AccountId, friend: T::AccountId) -> bool {
    // search for friend in AllFriends vector
    <AllFriends<T>>::get(current).iter()
		  .any(|&ref a| a == &friend)
  }

  pub fn is_blocked(current: T::AccountId, other_user: T::AccountId) -> bool {
    // search for friend in Blocked vector
    <Blocked<T>>::get(current).iter()
		  .any(|&ref a| a == &other_user)
  }
}
```

"*By returning `bool`, we can easily use these methods in `ensure!` statements to verify relevant state conditions before making requests in the main runtime methods.*"

```rust
// in the remove_friend method
ensure!(Self::friend_exists(user.clone(), old_friend.clone()), "old friend is not a friend");

...
// in the block method
ensure!(!Self::is_blocked(user.clone(), blocked_user.clone()), "user is already blocked");
```

Indeed, this pattern of extracting runtime checks into separate functions and invoking the `ensure` macro in their place is useful. It produces readable code and encourages targeted testing to more easily identify the source of logic errors.

*For a deeper dive into the "Verify First, Write Last" pattern, see the relevant section in the [Substrate Collectables tutorial](https://github.com/shawntabrizi/substrate-collectables-workshop/blob/master/3/buying-a-kitty.md#remember-verify-first-write-last) as well as [Substrate Best Practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices). This [github comment](https://github.com/shawntabrizi/substrate-collectables-workshop/pull/55#discussion_r258147961) is also very useful for visualizing the declarative pattern in practice.*

**Bonus Reading**
* [Design for Testability](https://blog.nelhage.com/2016/03/design-for-testability/)
* [Condition-Oriented Programming](https://www.parity.io/condition-oriented-programming/)
* [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts)

## Verifying Signed Messages <a name = "verify"></a>

It is often useful to designate some functions as permissioned and, therefore, accessible only to a defined group. In this case, we check that the transaction that invokes the runtime function is signed before verifying that the signature corresponds to a member of the permissioned set.

```rust
let who = ensure_signed(origin)?;
ensure!(Self::is_member(&who), "user is not a member of the group");
```

We can define `is_member` similar to the helper methods in the [Social Network](../storage/social.md#naive) recipe by defining a vector of `AccountId`s (`current_member`) that contains all members. We then search this vector for the `AccountId` in question within the body of the `is_member` method.

```rust
impl<T: Trait> Module<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		Self::current_member().iter()
			.any(|&ref a| a == who)
	}
}
```

*To read more about checking for signed messages, see the relevant section in the [Substrate collectables tutorial](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/storing-a-value?id=checking-for-a-signed-message).*

## Checking for Collisions <a name = "collide"></a>

Often times we may intend for keys to be unique identifiers that map to a specific storage item. In this case, it is necessary to check for collisions before adding new entries.

For example, it is common to use the hash of an object as the unique identifier in a map defined in the `decl_storage` block. Before adding a new value to the map, check that the key (hash) doesn't already have an associated value in the map. If it does, it is necessary to decide between the new item and the existing item to prevent an inadvertent key collision. In most cases, the new value is rejected.

```rust
fn insert_value(origin, hash: Hash, value: u32) {
    // check that key doesn't have an associated value
    ensure!( !(Self::map::exists(&hash)), "key already has an associated value" );

    // add key-value pair
    <Map<T>>::insert(hash, value);
}
```

*See how the [Substrate Collectables Tutorial](https://shawntabrizi.com/substrate-collectables-workshop/#/2/generating-random-data?id=checking-for-collision) covers this pattern.*