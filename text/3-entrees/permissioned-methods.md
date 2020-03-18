## Permissioned Methods
*[pallets/check-membership](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/check-membership)*

It is often useful to designate some functions as permissioned and, therefore, accessible only to a defined group. In this case, we check that the transaction that invokes the runtime function is signed before verifying that the signature corresponds to a member of the permissioned set.

To manage the set of members allowed to access the methods in question, we may store a vector in runtime storage. Without access to the standard library, it is necessary to use the [`Vec` struct](https://substrate.dev/rustdocs/master/sp_std/vec/struct.Vec.html) from the `sp-std` crate.


```rust, ignore
use sp_std::vec::Vec;
```

In the runtime, the membership set can be stored as

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as PGeneric {
		Members get(fn members): Vec<&T::AccountId>;
	}
}
```

## Permissionless Membership

If the membership is permissionless such anyone can join, an `add_member` function could be expressed as follows

```rust, ignore
fn add_member(origin) -> DispatchResult {
	let new_member = ensure_signed(origin)?;

	// Ensure that the caller is not already a member
	ensure!(!Self::is_member(&new_member), "already a member");

	<Members<T>>::append(&[new_member.clone()])?;
	Self::deposit_event(RawEvent::AddMember(new_member));
	Ok(())
}
```

Here we've used the [`append` method](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageValue.html#tymethod.append) to add the new member to the list. This allows a quick way to add data to the end of the vector without decoding the entire vector.

To increase the readability of the code, the membership check is extracted into its own auxiliary runtime method.

```rust, ignore
impl<T: Trait> Module<T> {
	pub fn is_member(who: &T::AccountId) -> bool {
		Self::members().contains(who)
	}
}
```
