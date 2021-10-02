//! A pallet that demonstrates the Transient Storage Adapter pattern through
//! the concrete example of a ringbuffer queue

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
use sp_std::prelude::*;

mod ringbuffer;

use ringbuffer::{RingBufferTrait, RingBufferTransient};

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	type BufferIndex = u8;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct ValueStruct {
		pub integer: i32,
		pub boolean: bool,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_value)]
	pub(super) type BufferMap<T> =
		StorageMap<_, Blake2_128Concat, BufferIndex, ValueStruct, ValueQuery>;

	#[pallet::type_value]
	pub(super) fn BufferIndexDefaultValue() -> (BufferIndex, BufferIndex) {
		(0, 0)
	}

	#[pallet::storage]
	#[pallet::getter(fn range)]
	pub(super) type BufferRange<T: Config> =
		StorageValue<_, (BufferIndex, BufferIndex), ValueQuery, BufferIndexDefaultValue>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Popped(i32, bool),
		DummyEvent(T::AccountId),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add an item to the queue
		#[pallet::weight(10_000)]
		pub fn add_to_queue(
			origin: OriginFor<T>,
			integer: i32,
			boolean: bool,
		) -> DispatchResultWithPostInfo {
			// only a user can push into the queue
			let _user = ensure_signed(origin)?;

			let mut queue = Self::queue_transient();
			queue.push(ValueStruct { integer, boolean });

			Ok(().into())
		}

		/// Add several items to the queue
		#[pallet::weight(10_000)]
		pub fn add_multiple(
			origin: OriginFor<T>,
			integers: Vec<i32>,
			boolean: bool,
		) -> DispatchResultWithPostInfo {
			// only a user can push into the queue
			let _user = ensure_signed(origin)?;

			let mut queue = Self::queue_transient();
			for integer in integers {
				queue.push(ValueStruct { integer, boolean });
			}

			Ok(().into())
		}

		/// Remove and return an item from the queue
		#[pallet::weight(10_000)]
		pub fn pop_from_queue(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// only a user can pop from the queue
			let _user = ensure_signed(origin)?;

			let mut queue = Self::queue_transient();
			if let Some(ValueStruct { integer, boolean }) = queue.pop() {
				Self::deposit_event(Event::Popped(integer, boolean));
			}

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Constructor function so we don't have to specify the types every time.
	///
	/// Constructs a ringbuffer transient and returns it as a boxed trait object.
	/// See [this part of the Rust book](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)
	fn queue_transient() -> Box<dyn RingBufferTrait<ValueStruct>> {
		Box::new(RingBufferTransient::<
			ValueStruct,
			<Self as Store>::BufferRange,
			<Self as Store>::BufferMap,
			u8,
		>::new())
	}
}
