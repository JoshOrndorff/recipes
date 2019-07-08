# Permissioned Function with Generic Event

This recipe contains a permissioned function which can only be called by the *Owner*. An event is emitted when the function is successfully executed.

The imports are the same as previous event recipes and our `Trait` inherits from `system::Trait`, which is relatively standard.
```rust
use srml_support::{StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

In our [`decl_storage`](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html) block, designate the `AccountId` of the owner that can invoke the permissioned function.

```rust
decl_storage! {
	trait Store for Module<T: Trait> as RuntimeExampleStorage {
		Owner get(owner): T::AccountId;
    }
}
```

When this `AccountId` is changed, it would be nice to emit an event to notify any relevant actors off-chain.

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

This recipe can be extended to create permissioned functions that limit invocations to members of specified groups.
<!-- TODO: add link to the DAO tutorial for this... -->