//! An example instantiable pallet (without default instance)

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;

// The pallet's configuration trait takes an instance as a type parameter. The instance type is
// created by the `decl_storage!` macro below.
pub trait Config<I: Instance>: frame_system::Config {
	// The ubiquitous event type's From bound needs updated to support the instance.
	type Event: From<Event<Self, I>> + Into<<Self as frame_system::Config>::Event>;
}

// It is necessary for instantiable pallets to call `decl_storage!` even if no storage items
// are used so the instance type is created.
decl_storage! {
	// The storage trait also takes the Instance parameter
	trait Store for Module<T: Config<I>, I: Instance> as LastCaller {

		// A single storage item that keeps track of
		// which account last called its only dispatchable call.
		Caller: T::AccountId;
	}
}

decl_event!(
	// The enum trait also takes the Instance as a parameter
	pub enum Event<T, I>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		Called(AccountId),
	}
);

decl_module! {
	// The `Module` struct also takes the instance parameter.
	pub struct Module<T: Config<I>, I: Instance> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		// The only dispatchable call, updates the single storage item,
		// and emits an event.
		#[weight = 10_000]
		fn call(origin) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			// When writing to storage, we supply, not only a configuration T, but also an
			// instance, I.
			<Caller<T, I>>::put(&caller);
			Self::deposit_event(RawEvent::Called(caller));
			Ok(())
		}
	}
}
