#![cfg_attr(not(feature = "std"), no_std)]

/// configurable module constants in substrate
use runtime_primitives::traits::Zero;
use support::traits::Get;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;

    // maximum amount added per invocation
    type MaxAddend: Get<u32>;

    // frequency with which the this value is deleted
    type ClearFrequency: Get<Self::BlockNumber>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        SingleValue get(fn single_value): u32;
    }
}

decl_event!(
    pub enum Event {
        // initial amount, amount added, final amount
        Added(u32, u32, u32),
        // cleared amount
        Cleared(u32),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        const MaxAddend: u32 = T::MaxAddend::get();

        const ClearFrequency: T::BlockNumber = T::ClearFrequency::get();

        fn add_value(origin, val_to_add: u32) -> Result {
            let _ = ensure_signed(origin)?;
            ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= maximum add amount constant");

            // previous value got
            let c_val = <SingleValue>::get();

            // checks for overflow when new value added
            let result = match c_val.checked_add(val_to_add) {
                Some(r) => r,
                None => return Err("Addition overflowed"),
            };
            <SingleValue>::put(result);
            Self::deposit_event(Event::Added(c_val, val_to_add, result));
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::ClearFrequency::get()).is_zero() {
                let c_val = <SingleValue>::get();
                <SingleValue>::put(0u32);
                Self::deposit_event(Event::Cleared(c_val));
            }
        }

        // for testing purposes
        fn set_value(origin, value: u32) -> Result {
            let _ = ensure_signed(origin)?;
            <SingleValue>::put(value);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Event;
    use crate::{Module, Trait};
    use primitives::H256;
    use runtime_io;
    use runtime_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup, OnFinalize},
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

    mod constant_config {
        pub use super::super::*;
    }

    impl_outer_event! {
        pub enum TestEvent for Runtime {
            constant_config,
        }
    }

    parameter_types! {
        pub const MaxAddend: u32 = 100;
        pub const ClearFrequency: u64 = 10;
    }
    impl Trait for Runtime {
        type Event = TestEvent;
        type MaxAddend = MaxAddend;
        type ClearFrequency = ClearFrequency;
    }

    pub type System = system::Module<Runtime>;
    pub type ConstantConfig = Module<Runtime>;

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
    fn max_added_exceeded_errs() {
        ExtBuilder::build().execute_with(|| {
            assert_err!(
                ConstantConfig::add_value(Origin::signed(1), 101),
                "value must be <= maximum add amount constant"
            );
        })
    }

    #[test]
    fn overflow_checked() {
        ExtBuilder::build().execute_with(|| {
            let test_num: u32 = u32::max_value() - 99;
            ConstantConfig::set_value(Origin::signed(1), test_num);

            assert_err!(
                ConstantConfig::add_value(Origin::signed(1), 100),
                "Addition overflowed"
            );
        })
    }

    #[test]
    fn add_value_works() {
        ExtBuilder::build().execute_with(|| {
            ConstantConfig::set_value(Origin::signed(1), 10);

            ConstantConfig::add_value(Origin::signed(2), 100);
            let expected_event1 = TestEvent::constant_config(Event::Added(10, 100, 110));
            assert!(System::events().iter().any(|a| a.event == expected_event1));

            ConstantConfig::add_value(Origin::signed(3), 100);
            let expected_event2 = TestEvent::constant_config(Event::Added(110, 100, 210));
            assert!(System::events().iter().any(|a| a.event == expected_event2));

            ConstantConfig::add_value(Origin::signed(4), 100);
            let expected_event3 = TestEvent::constant_config(Event::Added(210, 100, 310));
            assert!(System::events().iter().any(|a| a.event == expected_event3));
        })
    }

    #[test]
    fn on_finalize_clears() {
        ExtBuilder::build().execute_with(|| {
            System::set_block_number(5);
            ConstantConfig::set_value(Origin::signed(1), 10);

            ConstantConfig::add_value(Origin::signed(2), 100);

            ConstantConfig::on_finalize(10);
            let expected_event = TestEvent::constant_config(Event::Cleared(110));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            assert_eq!(ConstantConfig::single_value(), 0);
        })
    }
}
