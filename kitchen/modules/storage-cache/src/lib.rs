#![cfg_attr(not(feature = "std"), no_std)]

// storage cache example
// takeaway: minimize calls to runtime storage
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as StorageCache {
        // copy type
        SomeCopyValue get(fn some_copy_value): u32;

        // clone type
        KingMember get(fn king_member): T::AccountId;
        GroupMembers get(fn group_members): Vec<T::AccountId>;
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

        ///  (Copy) inefficient way of updating value in storage
        ///
        /// storage value -> storage_value * 2 + input_val
        fn increase_value_no_cache(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <SomeCopyValue>::get();
            let some_calculation = original_call.checked_add(some_val).ok_or("addition overflowed1")?; // doesn't check for overflow!
            // this next storage call is unnecessary and is wasteful
            let unnecessary_call = <SomeCopyValue>::get();
            // should've just used first_call here because u32 is copy
            let another_calculation = some_calculation.checked_add(unnecessary_call).ok_or("addition overflowed2")?;
            <SomeCopyValue>::put(another_calculation);
            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
            Ok(())
        }

        /// (Copy) more efficient value change
        ///
        /// storage value -> storage_value * 2 + input_val
        fn increase_value_w_copy(origin, some_val: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let original_call = <SomeCopyValue>::get();
            let some_calculation = original_call.checked_add(some_val).ok_or("addition overflowed1")?; // doesn't check for overflow either!
            // uses the original_call because u32 is copy
            let another_calculation = some_calculation.checked_add(original_call).ok_or("addition overflowed2")?;
            <SomeCopyValue>::put(another_calculation);
            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
            Ok(())
        }

        ///  (Clone) inefficient implementation
        /// swaps the king account with Origin::signed() if
        /// (1) other account is member &&
        /// (2) existing king isn't
        fn swap_king_no_cache(origin) -> Result {
            let new_king = ensure_signed(origin)?;
            let existing_king = <KingMember<T>>::get();

            // only places a new account if
            // (1) the existing account is not a member &&
            // (2) the new account is a member
            ensure!(!Self::is_member(&existing_king), "current king is a member so maintains priority");
            ensure!(Self::is_member(&new_king), "new king is not a member so doesn't get priority");

            // BAD (unnecessary) storage call
            let old_king = <KingMember<T>>::get();
            // place new king
            <KingMember<T>>::put(new_king.clone());

            Self::deposit_event(RawEvent::InefficientKingSwap(old_king, new_king));
            Ok(())
        }

        ///  (Clone) better implementation
        /// swaps the king account with Origin::signed() if
        /// (1) other account is member &&
        /// (2) existing king isn't
        fn swap_king_with_cache(origin) -> Result {
            let new_king = ensure_signed(origin)?;
            let existing_king = <KingMember<T>>::get();
            // prefer to clone previous call rather than repeat call unnecessarily
            let old_king = existing_king.clone();

            // only places a new account if
            // (1) the existing account is not a member &&
            // (2) the new account is a member
            ensure!(!Self::is_member(&existing_king), "current king is a member so maintains priority");
            ensure!(Self::is_member(&new_king), "new king is not a member so doesn't get priority");

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
            ensure!(!Self::is_member(&added), "member already in group");
            <GroupMembers<T>>::append(&mut vec![added])?;
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn is_member(who: &T::AccountId) -> bool {
        <GroupMembers<T>>::get().contains(who)
    }
}

#[cfg(test)]
mod tests {
    use super::RawEvent;
    use crate::{Module, Trait};
    use primitives::H256;
    use runtime_io;
    use runtime_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use support::{assert_err, impl_outer_event, impl_outer_origin, parameter_types, traits::Get};
    use system::{ensure_signed, EventRecord, Phase};

    impl_outer_origin! {
        pub enum Origin for Runtime {}
    }

    // Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Runtime;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: u32 = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::one();
    }
    impl system::Trait for Runtime {
        type Origin = Origin;
        type Index = u64;
        type Call = ();
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = TestEvent;
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
    }

    mod storage_cache {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for Runtime {
            storage_cache<T>,
        }
    }

    impl Trait for Runtime {
        type Event = TestEvent;
    }

    pub type System = system::Module<Runtime>;
    pub type StorageCache = Module<Runtime>;

    pub struct ExtBuilder;

    impl ExtBuilder {
        pub fn build() -> runtime_io::TestExternalities {
            let mut storage = system::GenesisConfig::default()
                .build_storage::<Runtime>()
                .unwrap();
            runtime_io::TestExternalities::from(storage)
        }
    }

    #[test]
    fn init_storage() {
        ExtBuilder::build().execute_with(|| {
            StorageCache::set_copy(Origin::signed(1), 10);
            assert_eq!(StorageCache::some_copy_value(), 10);

            StorageCache::set_king(Origin::signed(2));
            let second_account = ensure_signed(Origin::signed(2)).unwrap();
            assert_eq!(StorageCache::king_member(), second_account);

            StorageCache::mock_add_member(Origin::signed(1));
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            assert_err!(
                StorageCache::mock_add_member(Origin::signed(1)),
                "member already in group"
            );
            assert!(StorageCache::group_members().contains(&first_account));
        })
    }

    #[test]
    fn increase_value_errs_on_overflow() {
        ExtBuilder::build().execute_with(|| {
            let num1: u32 = u32::max_value() - 9;
            StorageCache::set_copy(Origin::signed(1), num1);
            // test first overflow panic for both methods
            assert_err!(
                StorageCache::increase_value_no_cache(Origin::signed(1), 10),
                "addition overflowed1"
            );
            assert_err!(
                StorageCache::increase_value_w_copy(Origin::signed(1), 10),
                "addition overflowed1"
            );

            let num2: u32 = 2147483643;
            StorageCache::set_copy(Origin::signed(1), num2);
            // test second overflow panic for both methods
            assert_err!(
                StorageCache::increase_value_no_cache(Origin::signed(1), 10),
                "addition overflowed2"
            );
            assert_err!(
                StorageCache::increase_value_w_copy(Origin::signed(1), 10),
                "addition overflowed2"
            );
        })
    }

    #[test]
    fn increase_value_works() {
        ExtBuilder::build().execute_with(|| {
            System::set_block_number(5);
            StorageCache::set_copy(Origin::signed(1), 25);
            StorageCache::increase_value_no_cache(Origin::signed(1), 10);
            // proof: x = 25, 2x + 10 = 60 qed
            let expected_event1 = TestEvent::storage_cache(RawEvent::InefficientValueChange(60, 5));
            assert!(System::events().iter().any(|a| a.event == expected_event1));

            StorageCache::increase_value_w_copy(Origin::signed(1), 10);
            // proof: x = 60, 2x + 10 = 130
            let expected_event2 = TestEvent::storage_cache(RawEvent::BetterValueChange(130, 5));
            assert!(System::events().iter().any(|a| a.event == expected_event1));

            // check storage
            assert_eq!(StorageCache::some_copy_value(), 130);
        })
    }

    #[test]
    fn swap_king_errs_as_intended() {
        ExtBuilder::build().execute_with(|| {
            StorageCache::mock_add_member(Origin::signed(1));
            StorageCache::set_king(Origin::signed(1));
            assert_err!(
                StorageCache::swap_king_no_cache(Origin::signed(3)),
                "current king is a member so maintains priority"
            );
            assert_err!(
                StorageCache::swap_king_with_cache(Origin::signed(3)),
                "current king is a member so maintains priority"
            );

            StorageCache::set_king(Origin::signed(2));
            assert_err!(
                StorageCache::swap_king_no_cache(Origin::signed(3)),
                "new king is not a member so doesn't get priority"
            );
            assert_err!(
                StorageCache::swap_king_with_cache(Origin::signed(3)),
                "new king is not a member so doesn't get priority"
            );
        })
    }

    #[test]
    fn swap_king_works() {
        ExtBuilder::build().execute_with(|| {
            StorageCache::mock_add_member(Origin::signed(2));
            StorageCache::mock_add_member(Origin::signed(3));
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let second_account = ensure_signed(Origin::signed(2)).unwrap();
            let third_account = ensure_signed(Origin::signed(3)).unwrap();

            StorageCache::set_king(Origin::signed(1));
            StorageCache::swap_king_no_cache(Origin::signed(2));

            let expected_event = TestEvent::storage_cache(RawEvent::InefficientKingSwap(
                first_account,
                second_account,
            ));
            assert!(System::events().iter().any(|a| a.event == expected_event));
            assert_eq!(StorageCache::king_member(), second_account);

            StorageCache::set_king(Origin::signed(1));
            assert_eq!(StorageCache::king_member(), first_account);
            StorageCache::swap_king_with_cache(Origin::signed(3));

            let expected_event =
                TestEvent::storage_cache(RawEvent::BetterKingSwap(first_account, third_account));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            assert_eq!(StorageCache::king_member(), third_account);
        })
    }
}
