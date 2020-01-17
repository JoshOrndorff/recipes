#![cfg_attr(not(feature = "std"), no_std)]

/// Permissioned Function with Generic Event
/// a permissioned funtion which can only be called by the "owner". An event is emitted
/// when the function is successfully executed.
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageValue};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as PGeneric {
        Owner get(fn owner): T::AccountId;

        Members get(fn members): Vec<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        OwnershipInitiated(AccountId),
        OwnershipTransferred(AccountId, AccountId),
        AddMember(AccountId),
        RemoveMember(AccountId),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn init_ownership(origin) -> DispatchResult {
            ensure!(!<Owner<T>>::exists(), "Owner already exists");
            let sender = ensure_signed(origin)?;
            <Owner<T>>::put(sender.clone());
            Self::deposit_event(RawEvent::OwnershipInitiated(sender));
            Ok(())
        }

        fn transfer_ownership(origin, new_owner: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::owner(), "This function can only be called by the owner");
            <Owner<T>>::put(new_owner.clone());
            Self::deposit_event(RawEvent::OwnershipTransferred(sender, new_owner));
            Ok(())
        }

        fn add_member(origin) -> DispatchResult {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_member(&new_member), "already a member");

            <Members<T>>::append(&[new_member.clone()])?;
            Self::deposit_event(RawEvent::AddMember(new_member));
            Ok(())
        }

        fn remove_member(origin) -> DispatchResult {
            let old_member = ensure_signed(origin)?;
            ensure!(Self::is_member(&old_member), "not a member so can't be taken out of the set");
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
