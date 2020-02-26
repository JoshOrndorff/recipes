#![cfg_attr(not(feature = "std"), no_std)]

/// configurable pallet constants in substrate
use runtime_primitives::traits::Zero;
use support::traits::Get;
use support::{
	decl_event,
	decl_module,
	decl_storage,
	dispatch::{DispatchResult, DispatchError},
	ensure,
	StorageValue
};
use system::ensure_signed;

#[cfg(test)]
mod tests;

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

		fn add_value(origin, val_to_add: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= maximum add amount constant");

			// previous value got
			let c_val = <SingleValue>::get();

			// checks for overflow when new value added
			let result = match c_val.checked_add(val_to_add) {
				Some(r) => r,
				None => return Err(DispatchError::Other("Addition overflowed")),
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
		fn set_value(origin, value: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			<SingleValue>::put(value);
			Ok(())
		}
	}
}
