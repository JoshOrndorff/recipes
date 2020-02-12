#![cfg_attr(not(feature = "std"), no_std)]

/// List via Maps
/// Substrate does not natively support a list type since it may encourage
/// dangerous habits. Unless explicitly guarded against, a list will add
/// unbounded `O(n)` complexity to an operation that will only charge `O(1)`
/// fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)).
/// This opens an economic attack vector on your chain.
use support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageLinkedMap, StorageMap,
    StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as List {
        TheList get(fn the_list): map u32 => T::AccountId;
        LargestIndex get(fn largest_index): u32;

        TheLinkedList get(fn linked_list): linked_map u32 => T::AccountId;
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
        fn deposit_event() = default;

        fn add_member(origin) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Note: We use a 1-based (instead of 0-based) list here
            // Note: Handle overflow here in production code!
            let new_count = <LargestIndex>::get() + 1;
            // insert new member past the end of the list
            <TheList<T>>::insert(new_count, &who);
            // store the incremented count
            <LargestIndex>::put(new_count);

            // (keep linked list synced)
            <TheLinkedList<T>>::insert(new_count, who.clone());

            Self::deposit_event(RawEvent::MemberAdded(who));

            Ok(())
        }

        // worst option
        // -- only works if the list can be *discontiguous*
        fn remove_member_discontiguous(origin, index: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // verify existence
            ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
            // use take for event emission, use remove to drop value
            let removed_member = <TheList<T>>::take(index);
            // assumes that we do not need to adjust the list because every add just increments counter

            Self::deposit_event(RawEvent::MemberRemoved(removed_member));

            Ok(())
        }

        // ok option
        // swap and pop
        // -- better than `remove_member_discontiguous`
        // -- this pattern becomes unwieldy fast!
        fn remove_member_contiguous(origin, index: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");

            let largest_index = <LargestIndex>::get();
            // swap
            if index != largest_index {
                <TheList<T>>::swap(index, largest_index);
            }
            // pop, uses `take` to return the member in the event
            let removed_member = <TheList<T>>::take(largest_index);
            <LargestIndex>::put(largest_index - 1);

            Self::deposit_event(RawEvent::MemberRemoved(removed_member));

            Ok(())
        }

        // best option (atm)
        // this uses the enumerable storage map
        // should be generally preferred
        fn remove_member_linked(origin, index: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            ensure!(<TheLinkedList<T>>::exists(index), "A member does not exist at this index");

            let removed_member = <TheLinkedList<T>>::take(index);

            Self::deposit_event(RawEvent::MemberRemoved(removed_member));

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
    use support::{assert_ok, assert_err, impl_outer_event, impl_outer_origin, parameter_types};
    use system;

    impl_outer_origin! {
        pub enum Origin for TestRuntime {}
    }

    // Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct TestRuntime;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: u32 = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::one();
    }
    impl system::Trait for TestRuntime {
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
        type ModuleToIndex = ();
    }

    mod linked_map {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            linked_map<T>,
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
    }

    pub type System = system::Module<TestRuntime>;
    pub type LinkedMap = Module<TestRuntime>;

    pub struct ExtBuilder;

    impl ExtBuilder {
        pub fn build() -> runtime_io::TestExternalities {
            let storage = system::GenesisConfig::default()
                .build_storage::<TestRuntime>()
                .unwrap();
            runtime_io::TestExternalities::from(storage)
        }
    }

    #[test]
    fn add_member_works() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(LinkedMap::add_member(Origin::signed(1)));

            let expected_event =
                TestEvent::linked_map(RawEvent::MemberAdded(1));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            let counter = LinkedMap::largest_index();
            assert_eq!(counter, 1);
            assert_eq!(LinkedMap::the_list(counter), 1);
            let lcounter = LinkedMap::largest_index();
            assert_eq!(lcounter, 1);
            assert_eq!(LinkedMap::linked_list(lcounter), 1);

            assert_ok!(LinkedMap::add_member(Origin::signed(2)));

            let counter2 = LinkedMap::largest_index();
            assert_eq!(counter2, 2);
            assert_eq!(LinkedMap::the_list(counter2), 2);
            let lcounter2 = LinkedMap::largest_index();
            assert_eq!(lcounter2, 2);
            assert_eq!(LinkedMap::linked_list(lcounter2), 2);
        })
    }

    #[test]
    fn remove_works() {
        ExtBuilder::build().execute_with(|| {
            assert_err!(
                LinkedMap::remove_member_discontiguous(Origin::signed(1), 1),
                "an element doesn't exist at this index"
            );
            assert_ok!(LinkedMap::add_member(Origin::signed(1)));

            let expected_event =
                TestEvent::linked_map(RawEvent::MemberAdded(1));
            assert!(System::events().iter().any(|a| a.event == expected_event));
            // check event is emitted
            let counter = LinkedMap::largest_index();
            assert_eq!(counter, 1);

            // remove unbounded doesn't decrement counter
            assert_ok!(LinkedMap::remove_member_discontiguous(Origin::signed(1), 1));
            let expected_event =
                TestEvent::linked_map(RawEvent::MemberRemoved(1));
            assert!(System::events().iter().any(|a| a.event == expected_event));
            let counter2 = LinkedMap::largest_index();
            // the counter doesn't decrement because the list was unbounded (counter always increases)
            assert_eq!(counter2, 1);

            // add a new member
            assert_ok!(LinkedMap::add_member(Origin::signed(2))); // note: counter increments

            // remove bounded decrements counter
            assert_ok!(LinkedMap::remove_member_contiguous(Origin::signed(1), 2));
            let expected_event2 =
                TestEvent::linked_map(RawEvent::MemberRemoved(2));
            assert!(System::events().iter().any(|a| a.event == expected_event2));
            let counter2 = LinkedMap::largest_index();
            // counter decrements (from 2 to 1)
            assert_eq!(counter2, 1);

            assert_ok!(LinkedMap::remove_member_linked(Origin::signed(1), 1));
            let expected_event3 =
                TestEvent::linked_map(RawEvent::MemberRemoved(1));
            assert!(System::events().iter().any(|a| a.event == expected_event3));
            // no required counter for linked map
        })
    }
}
