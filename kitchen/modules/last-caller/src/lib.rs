#![cfg_attr(not(feature = "std"), no_std)]

/// An example instantiable module (without default instance)
use support::{decl_module, decl_event, decl_storage, dispatch::Result, StorageValue};
use system::{self, ensure_signed};

// The module's configuration trait takes an instance as a type parameter. The instance type is
// created by the `decl_storage!` macro below.
pub trait Trait<I: Instance>: system::Trait {

    // The ubiquitous event type's From bound needs updated to support the instance.
    type Event: From<Event<Self, I>> + Into<<Self as system::Trait>::Event>;
}

// It is necessary for instantiable modules to call `decl_storage!` so
// the instance type is created.
decl_storage! {
    // The storage trait also takes the Instance parameter
    trait Store for Module<T: Trait<I>, I: Instance> as LastCaller {

        // A single storage item that keeps track of
        // which account last called its only dispatchable call.
        Caller: T::AccountId;
    }
}

decl_event!(
    // The enum trait also takes the Instance as a parameter
    pub enum Event<T, I> where AccountId = <T as system::Trait>::AccountId {
        Called(AccountId),
    }
);

decl_module! {
    // The module struct also takes the instance parameter.
    pub struct Module<T: Trait<I>, I: Instance> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // The only dispatchable call, updates the single storage item,
        // and emits an event.
        fn call(origin) -> Result {
            let caller = ensure_signed(origin)?;

            // When writing to storage, we supply, not only a configuration T, but also an
            // instance, I.
            <Caller<T, I>>::put(&caller);
            Self::deposit_event(RawEvent::Called(caller));
            Ok(())
        }
    }
}
