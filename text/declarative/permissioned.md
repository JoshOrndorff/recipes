## Permissioned Methods
*[pallets/check-membership](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/check-membership)*

It is often useful to designate some functions as permissioned and, therefore, accessible only to a defined group. In this case, we check that the transaction that invokes the runtime function is signed before verifying that the signature corresponds to a member of the permissioned set.

To manage the set of members allowed to access the methods in question, we may store a vector in runtime storage. Without access to the standard library, it is necessary to add the `sr-std` dependency to the `Cargo.toml` file and import its prelude:

```
[dependencies.rstd]
default_features = false
git = 'https://github.com/paritytech/substrate.git'
package = 'sr-std'
rev = '3e651110aa06aa835790df63410a29676243fc54'
```

The alias for `sr-std` used is `rstd` which follows substrate's conventions. To import a vector type that can be stored in the runtime:

```rust, ignore
use rstd::prelude::*;
```

In the runtime, the membership set can be stored as

```rust, ignore
decl_storage! {
    trait Store for Module<T: Trait> as PGeneric {
        Members get(fn members): Vec<&T::AccountId>;
    }
}
```

If the set was determined to be permissionless, we could express this in the runtime as

```rust, ignore
fn add_member(origin) -> Result {
	// unwrap signed extrinsic into AccountId
	let new_member = ensure_signed(origin)?;
	// check that the AccountId is contained in the `Members` vector
	ensure!(!Self::members().contains(&new_member), "already a member, don't add duplicates");
	// append the new member to the vec storage value
	<Members<T>>::append(&[new_member.clone()])?;
	Self::deposit_event(RawEvent::AddMember(new_member));
	Ok(())
}
```

To increase the readability of the code, the membership check can be extracted into its own auxiliary runtime method.

```rust, ignore
impl<T: Trait> Module<T> {
    pub fn is_member(who: &T::AccountId) -> bool {
        Self::members().contains(who)
    }
}
```

The `add_member` method now reads

```rust, ignore
fn add_member(origin) -> Result {
	let new_member = ensure_signed(origin)?;
	// the membership check is now shorter
	ensure!(!Self::is_member(&new_member), "already a member");

	<Members<T>>::append(&[new_member.clone()])?;
	Self::deposit_event(RawEvent::AddMember(new_member));
	Ok(())
}
```

## sudo

## custom origin
