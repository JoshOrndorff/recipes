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

### Custom Origin

* todo