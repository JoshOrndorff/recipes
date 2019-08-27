#![cfg_attr(not(feature = "std"), no_std)]

/// Permissioned Function with Generic Event
/// a permissioned funtion which can only be called by the "owner". An event is emitted 
/// when the function is successfully executed.

use support::{ensure, decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PGeneric {
		Owner get(owner): T::AccountId;

        Members get(members): Vec<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		OwnershipTransferred(AccountId, AccountId),
        AddMember(AccountId),
        RemoveMember(AccountId),
	}
);

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

        fn transfer_ownership(origin, new_owner: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::owner(), "This function can only be called by the owner");
            <Owner<T>>::put(&new_owner);
            Self::deposit_event(RawEvent::OwnershipTransferred(sender, new_owner));
            Ok(())
        }

        fn add_member(origin) -> Result {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_member(&new_member), "already a member");

            <Members<T>>::mutate(|mem| mem.push(new_member.clone())); // change to append after 3071 merged
            Self::deposit_event(RawEvent::AddMember(new_member));
            Ok(())
        }

        fn remove_member(origin) -> Result {
            let old_member = ensure_signed(origin)?;

            ensure!(Self::is_member(&old_member), "not a member");
            // keep all members except for the member in question
            <Members<T>>::mutate(|mem| mem.retain(|m| m != &old_member));
            Self::deposit_event(RawEvent::RemoveMember(old_member));
            Ok(())
        }
	}
}

impl<T: Trait> Module<T> {
    pub fn is_member(who: &T::AccountId) -> bool {
        Self::members().contains(who)
    }
}