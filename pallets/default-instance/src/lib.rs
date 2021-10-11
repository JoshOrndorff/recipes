//! An example instantiable pallet (with default instance)
//! TODO combine this and last caller into a singe crate (see check membership for an example)

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	// The pallet's configuration trait takes an instance as a type parameter. The instance type is
	// created by the `decl_storage!` macro below. Giving it a value of `DefaultInstance` allows us
	// to use the pallet in a runtime where only a single instance is desired without the extra syntax
	// that is otherwise needed to use instantiable pallets.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	pub(super) type Caller<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Called(T::AccountId),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The only dispatchable call, updates the single storage item,
		/// and emits an event.
		#[pallet::weight(10_000)]
		fn call(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			// When writing to storage, we supply, not only a configuration T, but also an
			// instance, I.
			<Caller<T>>::put(&caller);
			Self::deposit_event(Event::Called(caller));
			Ok(().into())
		}
	}
}
