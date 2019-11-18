#![cfg_attr(not(feature = "std"), no_std)]

/// List via Maps
/// Substrate does not natively support a list type since it may encourage
/// dangerous habits. Unless explicitly guarded against, a list will add
/// unbounded `O(n)` complexity to an operation that will only charge `O(1)`
/// fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)).
/// This opens an economic attack vector on your chain.
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageLinkedMap, StorageMap,
    StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as List {
        TheList get(fn the_list): map u32 => T::AccountId;
        TheCounter get(fn the_counter): u32;

        LinkedList get(fn linked_list): linked_map u32 => T::AccountId;
        LinkedCounter get(fn linked_counter): u32;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // member with AccountId added
        MemberAdded(AccountId),
        // member with AccountId removed
        MemberRemoved(AccountId),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // initialize the default event for this module
        fn deposit_event() = default;

        fn add_member(origin) -> Result {
            let who = ensure_signed(origin)?;

            // increment the counter
            let new_count = <TheCounter>::get() + 1;

            // add member at the largest_index
            <TheList<T>>::insert(new_count, who.clone());
            // incremement counter
            <TheCounter>::put(new_count);

            // (keep linked list synced)
            <LinkedList<T>>::insert(new_count, who.clone());
            // increment the counter
            <LinkedCounter>::put(new_count);

            Self::deposit_event(RawEvent::MemberAdded(who));

            Ok(())
        }

        // worst option
        // -- only works if the list is *unbounded*
        fn remove_member_unbounded(origin, index: u32) -> Result {
            let _ = ensure_signed(origin)?;

            // verify existence
            ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
            // for event emission (could be removed to minimize calls)
            let removed_member = <TheList<T>>::get(index);
            <TheList<T>>::remove(index);
            // assumes that we do not need to adjust the list because every add just increments counter

            Self::deposit_event(RawEvent::MemberRemoved(removed_member));

            Ok(())
        }

        // ok option
        // swap and pop
        // -- better than `remove_member_unbounded`
        // -- this pattern becomes unwieldy fast!
        fn remove_member_bounded(origin, index: u32) -> Result {
            let _ = ensure_signed(origin)?;

            ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");

            let largest_index = <TheCounter>::get();
            let member_to_remove = <TheList<T>>::take(index);
            // swap
            if index != largest_index {
                let temp = <TheList<T>>::take(largest_index);
                <TheList<T>>::insert(index, temp);
                <TheList<T>>::insert(largest_index, member_to_remove.clone());
            }
            // pop
            <TheList<T>>::remove(largest_index);
            <TheCounter>::put(largest_index - 1);

            Self::deposit_event(RawEvent::MemberRemoved(member_to_remove.clone()));

            Ok(())
        }

        // best option (atm)
        // this uses the enumerable storage map to simplify `swap and pop`
        // should be generally preferred
        fn remove_member_linked(origin, index: u32) -> Result {
            let _ = ensure_signed(origin)?;

            ensure!(<LinkedList<T>>::exists(index), "A member does not exist at this index");

            let head_index = <LinkedList<T>>::head().unwrap();
            let member_to_remove = <LinkedList<T>>::take(index);
            let head_member = <LinkedList<T>>::take(head_index);
            <LinkedList<T>>::insert(index, head_member);
            <LinkedList<T>>::insert(head_index, member_to_remove);
            <LinkedList<T>>::remove(head_index);

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

    mod linked_map {
        pub use super::super::*;
    }

    impl_outer_event! {
        pub enum TestEvent for Runtime {
            linked_map<T>,
        }
    }

    impl Trait for Runtime {
        type Event = TestEvent;
    }

    pub type System = system::Module<Runtime>;
    pub type LinkedMap = Module<Runtime>;

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
    fn add_member_works() {
        ExtBuilder::build().execute_with(|| {
            LinkedMap::add_member(Origin::signed(1));
            let first_account = ensure_signed(Origin::signed(1)).unwrap();

            let expected_event =
                TestEvent::linked_map(RawEvent::MemberAdded(first_account.clone()));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            let counter = LinkedMap::the_counter();
            assert_eq!(counter, 1);
            assert_eq!(LinkedMap::the_list(counter), first_account.clone());
            let lcounter = LinkedMap::the_counter();
            assert_eq!(lcounter, 1);
            assert_eq!(LinkedMap::linked_list(lcounter), first_account.clone());

            LinkedMap::add_member(Origin::signed(2));
            let second_account = ensure_signed(Origin::signed(2)).unwrap();

            let counter2 = LinkedMap::the_counter();
            assert_eq!(counter2, 2);
            assert_eq!(LinkedMap::the_list(counter2), second_account.clone());
            let lcounter2 = LinkedMap::the_counter();
            assert_eq!(lcounter2, 2);
            assert_eq!(LinkedMap::linked_list(lcounter2), second_account.clone());
        })
    }
}
