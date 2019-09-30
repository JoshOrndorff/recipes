// storage cache example
// -- generally...minimize calls to runtime storage

#![cfg_attr(not(feature = "std"), no_std)]

/// Single Value Storage
use support::{decl_module, decl_event, ensure, decl_storage, dispatch::Result, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as StorageCache {
        // copy type
        SomeCopyValue get(some_copy_value): u32;

        // clone type
        KingMember get(king_member): T::AccountId;
        GroupMembers get(group_members): Vec<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        // swap old value with new value (new_value, time_now)
        InefficientValueChange(u32, BlockNumber),
        // '' (new_value, time_now)
        BetterValueChange(u32, BlockNumber),
        // swap old king with new king (old, new)
        InefficientKingSwap(AccountId, AccountId),
        // '' (old, new)
        BetterKingSwap(AccountId, AccountId),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        //  (Copy) inefficient value change implementation
        fn swap_value_no_cache(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <SomeCopyValue>::get();
            let some_calculation = original_call + some_val; // doesn't check for overflow either!
            // this next storage call is unnecessary and is wasteful
            let unnecessary_call = <SomeCopyValue>::get();
            // should've just used first_call here because u32 is copy
            let another_calculation = some_calculation + unnecessary_call;
            <SomeCopyValue>::put(another_calculation);
            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
            Ok(())
        }
        
        // (Copy) more efficient value change
        fn swap_value_w_copy(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <SomeCopyValue>::get();
            let some_calculation = original_call + some_val; // doesn't check for overflow either!
            // uses the original_call because u32 is copy
            let another_calculation = some_calculation + original_call;
            <SomeCopyValue>::put(another_calculation);
            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
            Ok(())
        }

        //  (Clone) inefficient implementation
        // swaps the king account if 
        // (1) other account is member && 
        // (2) existing king isn't
        fn swap_king_no_cache(origin) -> Result {
            let new_king = ensure_signed(origin)?;
            let existing_king = <KingMember<T>>::get();

            // only places a new account if 
            // (1) the existing account is not a member &&
            // (2) the new account is a member
            ensure!(!Self::is_member(existing_king), "is a member so maintains priority");
            ensure!(Self::is_member(new_king.clone()), "not a member so doesn't get priority");

            // BAD (unnecessary) storage call
            let old_king = <KingMember<T>>::get();
            // place new king
            <KingMember<T>>::put(new_king.clone());

            Self::deposit_event(RawEvent::InefficientKingSwap(old_king, new_king));
            Ok(())
        }

        //  (Clone) better implementation
        // swaps the king account if 
        // (1) other account is member && 
        // (2) existing king isn't
        fn swap_king_with_cache(origin) -> Result {
            let new_king = ensure_signed(origin)?;
            let existing_king = <KingMember<T>>::get();
            // prefer to clone previous call rather than repeat call unnecessarily
            let old_king = existing_king.clone();

            // only places a new account if 
            // (1) the existing account is not a member &&
            // (2) the new account is a member
            ensure!(!Self::is_member(existing_king), "is a member so maintains priority");
            ensure!(Self::is_member(new_king.clone()), "not a member so doesn't get priority");

            // <no (unnecessary) storage call here>
            // place new king
            <KingMember<T>>::put(new_king.clone());

            Self::deposit_event(RawEvent::BetterKingSwap(old_king, new_king));
            Ok(())
        }

        // ---- for testing purposes ----
        fn set_copy(origin, val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            <SomeCopyValue>::put(val);
            Ok(())
        }

        fn set_king(origin) -> Result {
            let user = ensure_signed(origin)?;
            <KingMember<T>>::put(user);
            Ok(())
        }

        fn mock_add_member(origin) -> Result {
            let added = ensure_signed(origin)?;
            // see `append-to-vec` recipe for better way of doing this
            <GroupMembers<T>>::mutate(|v| v.extend_from_slice(&[added]));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // usually should be &T::AccountId
    pub fn is_member(who: T::AccountId) -> bool {
        // should usually use who: &T::AccountId, but showing caching best practices
        <GroupMembers<T>>::get().contains(&who)
    }
}
