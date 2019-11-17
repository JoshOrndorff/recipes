#![cfg_attr(not(feature = "std"), no_std)]

// Simple Storage Map
// https://crates.parity.io/srml_support/storage/trait.StorageMap.html
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as SimpleMap {
        SimpleMap get(fn simple_map): map T::AccountId => u32;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // insert new entry
        EntrySet(AccountId, u32),
        EntryGot(AccountId, u32),
        EntryTook(AccountId, u32),
        // increase (old_entry, new_entry) (by logic in increase which adds the input param)
        IncreaseEntry(u32, u32),
        // CompareAndSwap (old_entry, new_entry)
        CAS(u32, u32),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn set_single_entry(origin, entry: u32) -> Result {
            // only a user can set their entry
            let user = ensure_signed(origin)?;

            <SimpleMap<T>>::insert(user.clone(), entry);

            Self::deposit_event(RawEvent::EntrySet(user, entry));
            Ok(())
        }

        fn get_single_entry(origin, account: T::AccountId) -> Result {
            // anyone (signed extrinsic) can get an entry
            let getter = ensure_signed(origin)?;

            ensure!(<SimpleMap<T>>::exists(account.clone()), "an entry does not exist for this user");
            let entry = <SimpleMap<T>>::get(account);
            Self::deposit_event(RawEvent::EntryGot(getter, entry));
            Ok(())
        }

        fn take_single_entry(origin) -> Result {
            // only the user can take their own entry
            let user = ensure_signed(origin)?;

            ensure!(<SimpleMap<T>>::exists(user.clone()), "an entry does not exist for this user");
            let entry = <SimpleMap<T>>::take(user.clone());
            Self::deposit_event(RawEvent::EntryTook(user, entry));
            Ok(())
        }

        fn increase_single_entry(origin, add_this_val: u32) -> Result {
            // only the user can mutate their own entry
            let user = ensure_signed(origin)?;
            let original_value = <SimpleMap<T>>::get(&user);
            let new_value = original_value.checked_add(add_this_val).ok_or("value overflowed")?;
            <SimpleMap<T>>::insert(user, new_value);

            Self::deposit_event(RawEvent::IncreaseEntry(original_value, new_value));

            Ok(())
        }

        fn compare_and_swap_single_entry(origin, old_entry: u32, new_entry: u32) -> Result {
            // only a user that knows their previous entry can set the new entry
            let user = ensure_signed(origin)?;

            // compare
            ensure!(old_entry == <SimpleMap<T>>::get(user.clone()), "cas failed bc old_entry inputted by user != existing_entry");
            // and swap
            <SimpleMap<T>>::insert(user, new_entry);
            Self::deposit_event(RawEvent::CAS(old_entry, new_entry));
            Ok(())
        }
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
    use system::{EventRecord, Phase};

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

    mod simple_map {
        pub use super::super::*;
    }

    impl_outer_event! {
        pub enum TestEvent for Runtime {
            simple_map<T>,
        }
    }

    impl Trait for Runtime {
        type Event = TestEvent;
    }

    pub type System = system::Module<Runtime>;
    pub type SimpleMap = Module<Runtime>;

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
    fn set_works() {
        ExtBuilder::build().execute_with(|| {
            SimpleMap::set_single_entry(Origin::signed(1), 19);

            use system::ensure_signed;
            let first_id = ensure_signed(Origin::signed(1)).unwrap();

            let expected_event = TestEvent::simple_map(RawEvent::EntrySet(first_id, 19));

            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn get_works() {
        ExtBuilder::build().execute_with(|| {
            assert_err!(
                SimpleMap::get_single_entry(Origin::signed(1), 2),
                "an entry does not exist for this user"
            );

            SimpleMap::set_single_entry(Origin::signed(2), 19);
            SimpleMap::get_single_entry(Origin::signed(1), 2);

            use system::ensure_signed;
            let first_id = ensure_signed(Origin::signed(1)).unwrap();

            let expected_event = TestEvent::simple_map(RawEvent::EntryGot(first_id, 19));

            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn take_works() {
        ExtBuilder::build().execute_with(|| {
            assert_err!(
                SimpleMap::take_single_entry(Origin::signed(2)),
                "an entry does not exist for this user"
            );

            SimpleMap::set_single_entry(Origin::signed(2), 19);
            SimpleMap::take_single_entry(Origin::signed(2));

            use system::ensure_signed;
            let first_id = ensure_signed(Origin::signed(2)).unwrap();

            let expected_event = TestEvent::simple_map(RawEvent::EntryTook(first_id, 19));

            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn increase_works() {
        ExtBuilder::build().execute_with(|| {
            SimpleMap::set_single_entry(Origin::signed(2), 19);
            SimpleMap::increase_single_entry(Origin::signed(2), 2);

            let expected_event = TestEvent::simple_map(RawEvent::IncreaseEntry(19, 21));

            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn cas_works() {
        ExtBuilder::build().execute_with(|| {
            SimpleMap::set_single_entry(Origin::signed(2), 19);

            assert_err!(
                SimpleMap::compare_and_swap_single_entry(Origin::signed(2), 18, 32),
                "cas failed bc old_entry inputted by user != existing_entry"
            );

            SimpleMap::compare_and_swap_single_entry(Origin::signed(2), 19, 32);

            let expected_event = TestEvent::simple_map(RawEvent::CAS(19, 32));

            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }
}
