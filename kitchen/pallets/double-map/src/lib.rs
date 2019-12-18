#![cfg_attr(not(feature = "std"), no_std)]

// Double Map Example w/ remove_prefix
// https://crates.parity.io/srml_support/storage/trait.StorageDoubleMap.html
// `double_map` maps two keys to a single value.
// the first key might be a group identifier
// the second key might be a unique identifier
// `remove_prefix` enables clean removal of all values with the group identifier

use rstd::prelude::*;
use support::{
    decl_event, decl_module, decl_storage,
    dispatch::Result,
    ensure,
    storage::{StorageDoubleMap, StorageMap, StorageValue},
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub type GroupIndex = u32; // this is Encode (which is necessary for double_map)

decl_storage! {
    trait Store for Module<T: Trait> as Dmap {
        /// Member score (double map)
        MemberScore get(fn member_score): double_map GroupIndex, twox_128(T::AccountId) => u32;
        /// Get group ID for member
        GroupMembership get(fn group_membership): map T::AccountId => GroupIndex;
        /// For fast membership checks, see check-membership recipe for more details
        AllMembers get(fn all_members): Vec<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// New member for `AllMembers` group
        NewMember(AccountId),
        /// Put member score (id, index, score)
        MemberJoinsGroup(AccountId, GroupIndex, u32),
        /// Remove a single member with AccountId
        RemoveMember(AccountId),
        /// Remove all members with GroupId
        RemoveGroup(GroupIndex),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Join the `AllMembers` vec before joining a group
        fn join_all_members(origin) -> Result {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_member(&new_member), "already a member, can't join");
            <AllMembers<T>>::mutate(|v| v.push(new_member.clone()));
            Self::deposit_event(RawEvent::NewMember(new_member));
            Ok(())
        }

        /// Put MemberScore (for testing purposes)
        fn join_a_group(origin, index: GroupIndex, score: u32) -> Result {
            let member = ensure_signed(origin)?;
            ensure!(Self::is_member(&member), "not a member, can't remove");
            <MemberScore<T>>::insert(&index, &member, score);
            <GroupMembership<T>>::insert(&member, &index);
            Self::deposit_event(RawEvent::MemberJoinsGroup(member, index, score));
            Ok(())
        }

        fn remove_member(origin) -> Result {
            let member_to_remove = ensure_signed(origin)?;
            ensure!(Self::is_member(&member_to_remove), "not a member, can't remove");
            let group_id = <GroupMembership<T>>::take(member_to_remove.clone());
            <MemberScore<T>>::remove(&group_id, &member_to_remove);
            Self::deposit_event(RawEvent::RemoveMember(member_to_remove));
            Ok(())
        }

        fn remove_group(origin, group: GroupIndex) -> Result {
            let member = ensure_signed(origin)?;

            let group_id = <GroupMembership<T>>::get(member);
            // check that the member is in the group (at least)
            ensure!(group_id == group, "member isn't in the group, can't remove it");

            // remove all group members from MemberScore at once
            <MemberScore<T>>::remove_prefix(&group_id);

            Self::deposit_event(RawEvent::RemoveGroup(group_id));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // for fast membership checks (see check-membership recipe for more details)
    fn is_member(who: &T::AccountId) -> bool {
        <AllMembers<T>>::get().contains(who)
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
    }

    mod double_map {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            double_map<T>,
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
    }

    pub type System = system::Module<TestRuntime>;
    pub type DoubleMap = Module<TestRuntime>;

    pub struct ExtBuilder;

    impl ExtBuilder {
        pub fn build() -> runtime_io::TestExternalities {
            let mut storage = system::GenesisConfig::default()
                .build_storage::<TestRuntime>()
                .unwrap();
            runtime_io::TestExternalities::from(storage)
        }
    }

    #[test]
    fn join_all_members_works() {
        ExtBuilder::build().execute_with(|| {
            DoubleMap::join_all_members(Origin::signed(1));
            // correct panic upon existing member trying to join
            assert_err!(
                DoubleMap::join_all_members(Origin::signed(1)),
                "already a member, can't join"
            );

            DoubleMap::join_all_members(Origin::signed(1));

            // correct event emission
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let expected_event = TestEvent::double_map(RawEvent::NewMember(first_account.clone()));
            assert!(System::events().iter().any(|a| a.event == expected_event));
            // correct storage changes
            assert_eq!(DoubleMap::all_members(), vec![first_account]);
        })
    }

    #[test]
    fn group_join_works() {
        ExtBuilder::build().execute_with(|| {
            // expected panic
            assert_err!(
                DoubleMap::join_a_group(Origin::signed(1), 3, 5),
                "not a member, can't remove"
            );

            DoubleMap::join_all_members(Origin::signed(1));
            DoubleMap::join_a_group(Origin::signed(1), 3, 5);

            // correct event emission
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let expected_event =
                TestEvent::double_map(RawEvent::MemberJoinsGroup(first_account.clone(), 3, 5));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            // correct storage changes
            assert_eq!(DoubleMap::group_membership(first_account.clone()), 3);
            assert_eq!(DoubleMap::member_score(3, first_account.clone()), 5);
        })
    }

    #[test]
    fn remove_member_works() {
        ExtBuilder::build().execute_with(|| {
            DoubleMap::join_all_members(Origin::signed(1));
            DoubleMap::join_a_group(Origin::signed(1), 3, 5);
            DoubleMap::remove_member(Origin::signed(1));

            // correct event emission
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let expected_event =
                TestEvent::double_map(RawEvent::RemoveMember(first_account.clone()));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            // TODO: test correct changes to storage
        })
    }

    #[test]
    fn remove_group_works() {
        ExtBuilder::build().execute_with(|| {
            DoubleMap::join_all_members(Origin::signed(1));
            DoubleMap::join_all_members(Origin::signed(2));
            DoubleMap::join_all_members(Origin::signed(3));
            DoubleMap::join_a_group(Origin::signed(1), 3, 5);
            DoubleMap::join_a_group(Origin::signed(2), 3, 5);
            DoubleMap::join_a_group(Origin::signed(3), 3, 5);

            assert_err!(
                DoubleMap::remove_group(Origin::signed(4), 3),
                "member isn't in the group, can't remove it"
            );

            assert_err!(
                DoubleMap::remove_group(Origin::signed(1), 2),
                "member isn't in the group, can't remove it"
            );

            DoubleMap::remove_group(Origin::signed(1), 3);

            // correct event emission
            let first_account = ensure_signed(Origin::signed(1)).unwrap();
            let expected_event = TestEvent::double_map(RawEvent::RemoveGroup(3));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            // TODO: test correct changes to storage
        })
    }
}
