#![cfg_attr(not(feature = "std"), no_std)]

/// Single Value Storage
use support::{decl_module, decl_event, decl_storage, ensure, dispatch::Result, StorageValue};
use system::{self, ensure_signed};

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as SingleValue {
        MyValue: u32;
        MyAccount: T::AccountId;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        // value set at Set, BlockNumber at time
        ValueSet(u32, BlockNumber),
        // requested public value get
        ValueGet(u32, BlockNumber),
        // requested public account get
        AccountGet(AccountId, BlockNumber),
        // account set at Set, BlockNumber at time
        AccountSet(AccountId, BlockNumber),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn set_value(origin, value: u32) -> Result {
            let _ = ensure_signed(origin)?;
            let now = <system::Module<T>>::block_number();
            <MyValue>::put(value.clone());
            Self::deposit_event(RawEvent::ValueSet(value, now));
            Ok(())
        }

        fn get_value(origin) -> Result {
            let _ = ensure_signed(origin)?;
            let now = <system::Module<T>>::block_number();
            ensure!(<MyValue>::exists(), "value does not exist");
            let val = <MyValue>::get();
            Self::deposit_event(RawEvent::ValueGet(val, now));
            Ok(())
        }

        fn set_account(origin, account_to_set: T::AccountId) -> Result {
            let _ = ensure_signed(origin)?;
            let now = <system::Module<T>>::block_number();
            <MyAccount<T>>::put(account_to_set.clone());
            Self::deposit_event(RawEvent::AccountSet(account_to_set, now));
            Ok(())
        }

        fn get_account(origin) -> Result {
            let _ = ensure_signed(origin)?;
            let now = <system::Module<T>>::block_number();
            ensure!(<MyAccount<T>>::exists(), "account dne");
            let got_account = <MyAccount<T>>::get();
            Self::deposit_event(RawEvent::AccountGet(got_account, now));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
	use support::{assert_err, impl_outer_origin, impl_outer_event, parameter_types, traits::Get};
	use runtime_primitives::{Perbill, traits::{IdentityLookup, BlakeTwo256}, testing::Header};
    use system::{EventRecord, Phase};
    use super::RawEvent;
    use runtime_io;
	use primitives::H256;
	use crate::{Module, Trait};

	impl_outer_origin!{
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

    mod single_value {
        pub use super::super::*;
    }
    
    impl_outer_event! {
        pub enum TestEvent for Runtime {
            single_value<T>,
        }
    }

	impl Trait for Runtime {
		type Event = TestEvent;
    }

	pub type System = system::Module<Runtime>;
	pub type SingleValue = Module<Runtime>;

	pub struct ExtBuilder;

	impl ExtBuilder {
		pub fn build() -> runtime_io::TestExternalities {
			let mut storage = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
			runtime_io::TestExternalities::from(storage)
		}
    }

    #[test]
    fn set_value_works() {
        ExtBuilder::build().execute_with(|| {
            System::set_block_number(2);
            SingleValue::set_value(Origin::signed(1), 10);

            let expected_event = TestEvent::single_value(
                RawEvent::ValueSet(10, 2),
            );

            assert!(System::events().iter().any(|a| a.event == expected_event));
            
            System::set_block_number(15);
            SingleValue::set_value(Origin::signed(1), 11);

            let expected_event = TestEvent::single_value(
                RawEvent::ValueSet(11, 15),
            );

			assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn set_account_works() {
        // NOTE: could probably be combined into `set_works()`
        ExtBuilder::build().execute_with(|| {
            System::set_block_number(2);
            SingleValue::set_account(Origin::signed(1), 10);

            let expected_event = TestEvent::single_value(
                RawEvent::AccountSet(10, 2),
            );

            assert!(System::events().iter().any(|a| a.event == expected_event));
            
            System::set_block_number(15);
            SingleValue::set_account(Origin::signed(1), 11);

            let expected_event = TestEvent::single_value(
                RawEvent::AccountSet(11, 15),
            );

			assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

    #[test]
    fn get_works() {
        ExtBuilder::build().execute_with(|| {

                assert_err!(
                    SingleValue::get_value(Origin::signed(1)),
                    "value does not exist"
                );
                assert_err!(
                    SingleValue::get_account(Origin::signed(2)),
                    "account dne"
                );

                // set value and account
                System::set_block_number(2);
                SingleValue::set_value(Origin::signed(2), 5);
                SingleValue::set_account(Origin::signed(1), 10);

                // get value and account
                SingleValue::get_value(Origin::signed(1));

                let expected_event = TestEvent::single_value(
                    RawEvent::ValueGet(5, 2),
                );
    
                assert!(System::events().iter().any(|a| a.event == expected_event));

                SingleValue::get_account(Origin::signed(1));

                let expected_event2 = TestEvent::single_value(
                    RawEvent::AccountGet(10, 2),
                );

                assert!(System::events().iter().any(|a| a.event == expected_event2));

                // reset value and account
                System::set_block_number(12);
                SingleValue::set_value(Origin::signed(2), 27);
                SingleValue::set_account(Origin::signed(1), 13);

                // reget value and account
                SingleValue::get_value(Origin::signed(1));

                let expected_event3 = TestEvent::single_value(
                    RawEvent::ValueGet(27, 12),
                );
    
                assert!(System::events().iter().any(|a| a.event == expected_event3));

                SingleValue::get_account(Origin::signed(1));

                let expected_event4 = TestEvent::single_value(
                    RawEvent::AccountGet(13, 12),
                );

                assert!(System::events().iter().any(|a| a.event == expected_event4));
            }
        )
    }
}
