# Permissioned Function with Generic Event

This recipe contains a permissioned function which can only be called by the "owner". An event is emitted when the function is successfully executed.

```rust
use srml_support::{StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        fn init_ownership(origin) -> Result {
            ensure!(!<Owner<T>>::exists(), "Owner already exists");
            let sender = ensure_signed(origin)?;
            <Owner<T>>::put(&sender);
            Self::deposit_event(RawEvent::OwnershipTransferred(sender.clone(), sender));
            Ok(())
        }

        fn transfer_ownership(origin, newOwner: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::owner(), "This function can only be called by the owner");
            <Owner<T>>::put(&newOwner);
            Self::deposit_event(RawEvent::OwnershipTransferred(sender, newOwner));
            Ok(())
        }
    }
}

decl_storage! {
	trait Store for Module<T: Trait> as RuntimeExampleStorage {
		Owner get(owner): T::AccountId;
    }
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		OwnershipTransferred(AccountId, AccountId),
	}
);
```