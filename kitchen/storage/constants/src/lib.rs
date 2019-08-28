#![cfg_attr(not(feature = "std"), no_std)]

/// configurable module constants in substrate
use runtime_primitives::traits::Zero;
use support::{ensure, decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use support::traits::{Currency, Get, ReservableCurrency};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;

    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    // maximum amount added per invocation
    type MaxAddend: Get<u32>;

    // frequency with which the this value is deleted
    type ClearFrequency: Get<Self::BlockNumber>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
        SingleValue get(single_value): u32;
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

            // previous value
            let c_val = <SingleValue>::get();

			// checks for overflow
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
                <SingleValue>::put(0u32); // is this cheaper than killing?
                Self::deposit_event(Event::Cleared(c_val));
            }
        }
	}
}