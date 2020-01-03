#![cfg_attr(not(feature = "std"), no_std)]

// demonstrates how to use append instead of mutate
// https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageValue.html#tymethod.append
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as VecMap {
        Members get(fn members): Vec<T::AccountId>;
        CurrentValues get(fn current_values): Vec<u32>;
        NewValues get(fn new_values): Vec<u32>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // added member
        MemberAdded(AccountId),
        // removed member
        MemberRemoved(AccountId),
        // mutate to append
        MutateToAppend(AccountId),
        // append
        AppendVec(AccountId),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // don't do this
        // (unless appending new entries AND mutating existing entries)
        fn mutate_to_append(origin) -> Result {
            let user = ensure_signed(origin)?;

            // this decodes the existing vec, appends the new values, and re-encodes the whole thing
            <CurrentValues>::mutate(|v| v.extend_from_slice(&Self::new_values()));
            Self::deposit_event(RawEvent::MutateToAppend(user));
            Ok(())
        }

        // do this instead
        fn append_new_entries(origin) -> Result {
            let user = ensure_signed(origin)?;

            // this encodes the new values and appends them to the already encoded existing evc
            <CurrentValues>::append(&mut Self::new_values())?;
            Self::deposit_event(RawEvent::AppendVec(user));
            Ok(())
        }

        fn add_member(origin) -> Result {
            let new_member = ensure_signed(origin)?;
            ensure!(!Self::is_member(&new_member), "must not be a member to be added");
            <Members<T>>::append(&mut vec![new_member.clone()])?;
            Self::deposit_event(RawEvent::MemberAdded(new_member));
            Ok(())
        }

        fn remove_member(origin) -> Result {
            let old_member = ensure_signed(origin)?;
            ensure!(Self::is_member(&old_member), "must be a member in order to leave");
            <Members<T>>::mutate(|v| v.retain(|i| i != &old_member));
            Self::deposit_event(RawEvent::MemberRemoved(old_member));
            Ok(())
        }
        // also see `append_or_insert`, `append_or_put` in pallet-elections/phragmen, democracy
    }
}

impl<T: Trait> Module<T> {
    pub fn is_member(who: &T::AccountId) -> bool {
        <Members<T>>::get().contains(who)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Module, RawEvent, Trait};
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

    mod vec_set {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            vec_set<T>,
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
    }

    pub type System = system::Module<TestRuntime>;
    pub type VecSet = Module<TestRuntime>;

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
    fn add_member_err_works() {
        ExtBuilder::build().execute_with(|| {
            VecSet::add_member(Origin::signed(1));

            assert_err!(
                VecSet::add_member(Origin::signed(1)),
                "must not be a member to be added"
            );
        })
    }

    #[test]
    fn add_member_works() {
        ExtBuilder::build().execute_with(|| {
            VecSet::add_member(Origin::signed(1));

            let new_member = ensure_signed(Origin::signed(1)).unwrap();
            let expected_event = TestEvent::vec_set(RawEvent::MemberAdded(new_member));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            assert_eq!(VecSet::members(), vec![new_member]);
        })
    }

    #[test]
    fn remove_member_err_works() {
        ExtBuilder::build().execute_with(|| {
            // 2 is NOT previously added as a member
            assert_err!(
                VecSet::remove_member(Origin::signed(2)),
                "must be a member in order to leave"
            );
        })
    }

    #[test]
    fn remove_member_works() {
        ExtBuilder::build().execute_with(|| {
            VecSet::add_member(Origin::signed(1));
            VecSet::remove_member(Origin::signed(1));
            VecSet::add_member(Origin::signed(2));

            // check correct event emission
            let old_member = ensure_signed(Origin::signed(1)).unwrap();
            let new_member = ensure_signed(Origin::signed(2)).unwrap();
            let expected_event = TestEvent::vec_set(RawEvent::MemberRemoved(old_member));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            // check storage changes
            assert_eq!(VecSet::members(), vec![new_member]);
        })
    }
}
