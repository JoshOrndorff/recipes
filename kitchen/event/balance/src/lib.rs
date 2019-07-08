#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as IncBalance {
		BalanceVal get(balance_val): Option<T::Balance>;
	}
}

decl_event!(
	pub enum Event<T> where B = <T as balances::Trait>::Balance {
		NewBalance(B),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn accumulate_dummy(origin, increase_by: T::Balance) -> Result {
			// This is a public call, so we ensure that the origin is some signed account.
			let _sender = ensure_signed(origin)?;

			// use the `::get` on the storage item type itself
			let balance_val = <BalanceVal<T>>::get();

			// Calculate the new value.
			let new_balance = balance_val.map_or(increase_by, |val| val + increase_by);

			// Put the new value into storage.
			<BalanceVal<T>>::put(new_balance);

			// Deposit an event to let the outside world know this happened.
			Self::deposit_event(RawEvent::NewBalance(increase_by));

			// All good.
			Ok(())
		}
	}
}