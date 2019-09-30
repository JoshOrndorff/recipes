## Verifying Signed Messages

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

## Custom Origin Type
*[`kitchen/modules/custom-origin`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/custom-origin)*

[This recipe](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/custom-origin) uses special origin type to specify conditions for dispatching special extrinsics called `Proposal`s. 

In the `Trait` declaration, define the `Origin` type with special origin type and define the bounds on the outer call dispatch type known as `Proposal`.

```rust
pub trait Trait: system::Trait {
	type Origin: From<RawOrigin<Self::AccountId>>;

	type Proposal: Parameter + Dispatchable<Origin=<Self as Trait>::Origin>;

	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

Use the special origin type to verify the threshold of members that have approved a proposal (the first field). In addition, dispatch proposals by verifying that the participant that makes this request is a member.

```rust
/// special origin type
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RawOrigin<AccountId> {
	Members(MemberCount, MemberCount),
	// the main address for the DAO
	Member(AccountId),
}

// origin type for this module
pub type Origin<T> = RawOrigin<<T as system::Trait>::AccountId>; 
```

A simple example of how this works can be found in the `propose` runtime method. If the threshold defined by the member making a proposal is less than 2, then the member making the proposal already satisfies the threshold requirements (we assume their tacit support).

```rust
// fn propose in decl_module
if threshold < 2 {
	let total_members = Self::current_member().len() as MemberCount;
	let ok = proposal.dispatch(RawOrigin::Members(1, total_members).into()).is_ok();
}
```

In the above code, we set the second field of `RawOrigin::Members` to the total number of members in the DAO and the first field to the required threshold. The proposal is accordingly dispatched from the `RawOrigin` and any error is safely handled with [`.is_ok()`](https://doc.rust-lang.org/std/result/).