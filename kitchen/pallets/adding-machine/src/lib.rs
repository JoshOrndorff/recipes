#![cfg_attr(not(feature = "std"), no_std)]

/// Adding Machine
/// A simple adding machine which checks for overflow and emits an event with
/// the result, without using storage.
use support::{decl_event, decl_module, dispatch::{DispatchResult, DispatchError}};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn add(origin, val1: u32, val2: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            // checks for overflow
            let result = match val1.checked_add(val2) {
                Some(r) => r,
                None => return Err(DispatchError::Other("Addition overflowed")),
            };
            Self::deposit_event(Event::Added(val1, val2, result));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event {
        Added(u32, u32, u32),
    }
);

#[cfg(test)]
mod tests {
    use crate::{Module, Trait};
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use sp_core::H256;
    use support::{assert_err, assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
    use system::{EventRecord, Phase};

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

    mod added {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            added,
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
    }

    pub type System = system::Module<TestRuntime>;
    pub type AddingMachine = Module<TestRuntime>;

    pub struct ExtBuilder;

    impl ExtBuilder {
        pub fn build() -> sp_io::TestExternalities {
            let storage = system::GenesisConfig::default()
                .build_storage::<TestRuntime>()
                .unwrap();
            sp_io::TestExternalities::from(storage)
        }
    }

    #[test]
    fn add_emits_correct_event() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(AddingMachine::add(Origin::signed(1), 6, 9));
            assert_ok!(AddingMachine::add(Origin::signed(1), 235, 431));
            assert_ok!(AddingMachine::add(Origin::signed(1), 1700, 38));
            assert_ok!(AddingMachine::add(Origin::signed(1), 6, 79));
            assert_ok!(AddingMachine::add(Origin::signed(1), 13, 37));

            assert_eq!(
                System::events(),
                vec![
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: TestEvent::added(crate::Event::Added(6, 9, 15)),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: TestEvent::added(crate::Event::Added(235, 431, 666)),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: TestEvent::added(crate::Event::Added(1700, 38, 1738)),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: TestEvent::added(crate::Event::Added(6, 79, 85)),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: TestEvent::added(crate::Event::Added(13, 37, 50)),
                        topics: vec![],
                    },
                ]
            );
        })
    }

    #[test]
    fn overflow_fails() {
        ExtBuilder::build().execute_with(|| {
            assert_err!(
                AddingMachine::add(Origin::signed(3), u32::max_value(), 1),
                "Addition overflowed"
            );
        })
    }
}
