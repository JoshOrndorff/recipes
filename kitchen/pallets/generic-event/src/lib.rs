#![cfg_attr(not(feature = "std"), no_std)]

/// Event uses types from the pallet's configuration trait
use support::{decl_event, decl_module, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn do_something(origin, input: u32) -> Result {
            let user = ensure_signed(origin)?;

            // could do something with the input here instead
            let new_number = input;

            Self::deposit_event(RawEvent::EmitInput(user, new_number));
            Ok(())
        }
    }
}

// AccountId, u32 both are inputs `=>` declaration with `<T>`
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        EmitInput(AccountId, u32),
    }
);

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
    use support::{impl_outer_event, impl_outer_origin, parameter_types};
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
    }

    mod generic_event {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            generic_event<T>,
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
    }

    pub type System = system::Module<TestRuntime>;
    pub type GenericEvent = Module<TestRuntime>;

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
    fn test() {
        ExtBuilder::build().execute_with(|| {
            GenericEvent::do_something(Origin::signed(1), 32);

            // construct event that should be emitted in the method call directly above
            use system::ensure_signed;
            let caller_id = ensure_signed(Origin::signed(1)).unwrap();
            let expected_event = TestEvent::generic_event(RawEvent::EmitInput(caller_id, 32));

            // iterate through array of `EventRecord`s
            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }
}
