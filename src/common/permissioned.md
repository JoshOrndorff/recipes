# Permissioned Methods

## Single Owner Access Control

This recipe contains a permissioned function which can only be called by the *Owner*. An event is emitted when the function is successfully executed.

The imports are similar to previous event recipes with the additional import of the `support::StorageValue`.
```rust
// other imports
use support::{StorageValue};
```

In the [`decl_storage`](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html) block, designate the `AccountId` of the owner that can invoke the permissioned function.

```rust
decl_storage! {
	trait Store for Module<T: Trait> as RuntimeExampleStorage {
		Owner get(owner): T::AccountId;
    }
}
```

When this `AccountId` is changed, it is useful to emit an event to notify any relevant actors off-chain.

```rust
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		OwnershipTransferred(AccountId, AccountId),
	}
);
```

The main logic is contained in the runtime methods. Our first runtime method initiates the ownership. Before doing so, it verifies that no current owner exists.

```rust
/// in decl_module
fn init_ownership(origin) -> Result {
    ensure!(!<Owner<T>>::exists(), "Owner already exists");
    let sender = ensure_signed(origin)?;
    <Owner<T>>::put(&sender);
    Self::deposit_event(RawEvent::OwnershipTransferred(sender.clone(), sender));
    Ok(())
}
```


The second runtime method transfers ownership. Before doing so, it checks that the invocation is made by the current owner.
```rust
fn transfer_ownership(origin, newOwner: T::AccountId) -> Result {
    let sender = ensure_signed(origin)?;
    ensure!(sender == Self::owner(), "This function can only be called by the owner");
    <Owner<T>>::put(&newOwner);
    Self::deposit_event(RawEvent::OwnershipTransferred(sender, newOwner));
    Ok(())
}
```

## Group Membership Authentication

This recipe is extended to define permissioned functions which limit invocations to members of a group. The group's membership is managed in runtime storage:

```rust
// decl_storage block
Members get(members): Vec<T::AccountId>;
```

Runtime methods `add_member` demonstrates how members can be added. In other projects, existing members might vote on new member applications instead of automatic admission.

```rust
fn add_member(origin) -> Result {
    let new_member = ensure_signed(origin)?;
    ensure!(!Self::is_member(&new_member), "already a member");

    <Members<T>>::mutate(|mem| mem.push(new_member.clone())); // change to append after 3071 merged
    Self::deposit_event(RawEvent::AddMember(new_member));
    Ok(())
}
```

The `remove_member` method is similar. The only difference is that we are removing the member rather than pushing a new member to the method.

```rust
fn remove_member(origin) -> Result {
    let old_member = ensure_signed(origin)?;

    ensure!(Self::is_member(&old_member), "not a member");
    // keep all members except for the member in question
    <Members<T>>::mutate(|mem| mem.retain(|m| m != &old_member));
    Self::deposit_event(RawEvent::RemoveMember(old_member));
    Ok(())
}
```

The `ensure` checks are symmetric in the sense that `add_member` requires that the member in question is not already a member, while the `remove_member` method requires membership. To check membership within the runtime, we define the helper `is_member` method:

```rust
// impl<T: Trait> Module<T> block
pub fn is_member(who: &T::AccountId) -> bool {
    Self::members().contains(who)
}
```

This example can easily be extended to define criteria for adding and removing members. A well-written example can be found in [`srml/collective`](https://github.com/paritytech/substrate/blob/master/srml/collective/src/lib.rs), which also uses a `Vec<AccountId>` to manage membership.
