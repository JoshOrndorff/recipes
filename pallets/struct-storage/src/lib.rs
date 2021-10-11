#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

//! Struct Storage
//! This pallet demonstrates how to declare and store `structs` that contain types
//! that come from the pallet's configuration trait.

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
	pub struct InnerThing<Hash, Balance> {
		pub number: u32,
		pub hash: Hash,
		pub balance: Balance,
	}

	type InnerThingOf<T> =
		InnerThing<<T as frame_system::Config>::Hash, <T as pallet_balances::Config>::Balance>;

	#[derive(Encode, Decode, Default, RuntimeDebug)]
	pub struct SuperThing<Hash, Balance> {
		pub super_number: u32,
		pub inner_thing: InnerThing<Hash, Balance>,
	}

	#[pallet::storage]
	#[pallet::getter(fn inner_things_by_numbers)]
	pub(super) type InnerThingsByNumbers<T> =
		StorageMap<_, Blake2_128Concat, u32, InnerThingOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn super_things_by_super_numbers)]
	pub(super) type SuperThingsBySuperNumbers<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, SuperThing<T::Hash, T::Balance>, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// fields of the new inner thing
		NewInnerThing(u32, T::Hash, T::Balance),
		// fields of the super_number and the inner_thing fields
		NewSuperThingByExistingInner(u32, u32, T::Hash, T::Balance),
		// ""
		NewSuperThingByNewInner(u32, u32, T::Hash, T::Balance),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Stores an `InnerThing` struct in the storage map
		#[pallet::weight(10_000)]
		pub fn insert_inner_thing(
			origin: OriginFor<T>,
			number: u32,
			hash: T::Hash,
			balance: T::Balance,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			let thing = InnerThing {
				number,
				hash,
				balance,
			};
			<InnerThingsByNumbers<T>>::insert(number, thing);
			Self::deposit_event(Event::NewInnerThing(number, hash, balance));
			Ok(().into())
		}

		/// Stores a `SuperThing` struct in the storage map using an `InnerThing` that was already
		/// stored
		#[pallet::weight(10_000)]
		pub fn insert_super_thing_with_existing_inner(
			origin: OriginFor<T>,
			inner_number: u32,
			super_number: u32,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			let inner_thing = Self::inner_things_by_numbers(inner_number);
			let super_thing = SuperThing {
				super_number,
				inner_thing: inner_thing.clone(),
			};
			<SuperThingsBySuperNumbers<T>>::insert(super_number, super_thing);
			Self::deposit_event(Event::NewSuperThingByExistingInner(
				super_number,
				inner_thing.number,
				inner_thing.hash,
				inner_thing.balance,
			));
			Ok(().into())
		}

		/// Stores a `SuperThing` struct in the storage map using a new `InnerThing`
		#[pallet::weight(10_000)]
		pub fn insert_super_thing_with_new_inner(
			origin: OriginFor<T>,
			inner_number: u32,
			hash: T::Hash,
			balance: T::Balance,
			super_number: u32,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			// construct and insert `inner_thing` first
			let inner_thing = InnerThing {
				number: inner_number,
				hash,
				balance,
			};
			// overwrites any existing `InnerThing` with `number: inner_number` by default
			<InnerThingsByNumbers<T>>::insert(inner_number, inner_thing.clone());
			Self::deposit_event(Event::NewInnerThing(inner_number, hash, balance));
			// now construct and insert `super_thing`
			let super_thing = SuperThing {
				super_number,
				inner_thing,
			};
			<SuperThingsBySuperNumbers<T>>::insert(super_number, super_thing);
			Self::deposit_event(Event::NewSuperThingByNewInner(
				super_number,
				inner_number,
				hash,
				balance,
			));
			Ok(().into())
		}
	}
}
