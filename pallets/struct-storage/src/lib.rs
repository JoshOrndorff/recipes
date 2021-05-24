#![cfg_attr(not(feature = "std"), no_std)]

//! Struct Storage
//! This pallet demonstrates how to declare and store `structs` that contain types
//! that come from the pallet's configuration trait.

use frame_support::{
	codec::{Decode, Encode},
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
};
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;

#[cfg(test)]
mod tests;

pub trait Config: pallet_balances::Config + frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct InnerThing<Hash, Balance> {
	number: u32,
	hash: Hash,
	balance: Balance,
}

type InnerThingOf<T> =
	InnerThing<<T as frame_system::Config>::Hash, <T as pallet_balances::Config>::Balance>;

#[derive(Encode, Decode, Default, RuntimeDebug)]
pub struct SuperThing<Hash, Balance> {
	super_number: u32,
	inner_thing: InnerThing<Hash, Balance>,
}

decl_storage! {
	trait Store for Module<T: Config> as NestedStructs {
		InnerThingsByNumbers get(fn inner_things_by_numbers):
			map hasher(blake2_128_concat) u32 => InnerThingOf<T>;
		SuperThingsBySuperNumbers get(fn super_things_by_super_numbers):
			map hasher(blake2_128_concat) u32 => SuperThing<T::Hash, T::Balance>;
	}
}

decl_event! (
	pub enum Event<T>
	where
		<T as frame_system::Config>::Hash,
		<T as pallet_balances::Config>::Balance
	{
		// fields of the new inner thing
		NewInnerThing(u32, Hash, Balance),
		// fields of the super_number and the inner_thing fields
		NewSuperThingByExistingInner(u32, u32, Hash, Balance),
		// ""
		NewSuperThingByNewInner(u32, u32, Hash, Balance),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Stores an `InnerThing` struct in the storage map
		#[weight = 10_000]
		fn insert_inner_thing(origin, number: u32, hash: T::Hash, balance: T::Balance) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let thing = InnerThing {
							number,
							hash,
							balance,
						};
			<InnerThingsByNumbers<T>>::insert(number, thing);
			Self::deposit_event(RawEvent::NewInnerThing(number, hash, balance));
			Ok(())
		}

		/// Stores a `SuperThing` struct in the storage map using an `InnerThing` that was already
		/// stored
		#[weight = 10_000]
		fn insert_super_thing_with_existing_inner(origin, inner_number: u32, super_number: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let inner_thing = Self::inner_things_by_numbers(inner_number);
			let super_thing = SuperThing {
				super_number,
				inner_thing: inner_thing.clone(),
			};
			<SuperThingsBySuperNumbers<T>>::insert(super_number, super_thing);
			Self::deposit_event(RawEvent::NewSuperThingByExistingInner(super_number, inner_thing.number, inner_thing.hash, inner_thing.balance));
			Ok(())
		}

		/// Stores a `SuperThing` struct in the storage map using a new `InnerThing`
		#[weight = 10_000]
		fn insert_super_thing_with_new_inner(origin, inner_number: u32, hash: T::Hash, balance: T::Balance, super_number: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			// construct and insert `inner_thing` first
			let inner_thing = InnerThing {
				number: inner_number,
				hash,
				balance,
			};
			// overwrites any existing `InnerThing` with `number: inner_number` by default
			<InnerThingsByNumbers<T>>::insert(inner_number, inner_thing.clone());
			Self::deposit_event(RawEvent::NewInnerThing(inner_number, hash, balance));
			// now construct and insert `super_thing`
			let super_thing = SuperThing {
				super_number,
				inner_thing,
			};
			<SuperThingsBySuperNumbers<T>>::insert(super_number, super_thing);
			Self::deposit_event(RawEvent::NewSuperThingByNewInner(super_number, inner_number, hash, balance));
			Ok(())
		}
	}
}
