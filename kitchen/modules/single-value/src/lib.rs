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
