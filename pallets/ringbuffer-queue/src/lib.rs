//! A pallet that demonstrates the Transient Storage Adapter pattern through
//! the concrete example of a ringbuffer queue

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_std::prelude::*;

mod ringbuffer;

use ringbuffer::{RingBufferTrait, RingBufferTransient};

#[cfg(test)]
mod tests;

type BufferIndex = u8;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ValueStruct {
	integer: i32,
	boolean: bool,
}

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as RingBufferQueue {
		BufferMap get(fn get_value): map hasher(twox_64_concat) BufferIndex => ValueStruct;
		BufferRange get(fn range): (BufferIndex, BufferIndex) = (0, 0);
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		Popped(i32, bool),
		DummyEvent(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Add an item to the queue
		#[weight = 10_000]
		pub fn add_to_queue(origin, integer: i32, boolean: bool) -> DispatchResult {
			// only a user can push into the queue
			let _user = ensure_signed(origin)?;

			let mut queue = Self::queue_transient();
			queue.push(ValueStruct{ integer, boolean });

			Ok(())
		}

		/// Add several items to the queue
		#[weight = 10_000]
		pub fn add_multiple(origin, integers: Vec<i32>, boolean: bool) -> DispatchResult {
			// only a user can push into the queue
			let _user = ensure_signed(origin)?;

			let mut queue = Self::queue_transient();
			for integer in integers {
				queue.push(ValueStruct{ integer, boolean });
			}

			Ok(())
		}

		/// Remove and return an item from the queue
		#[weight = 10_000]
		pub fn pop_from_queue(origin) -> DispatchResult {
			// only a user can pop from the queue
			let _user = ensure_signed(origin)?;

			let mut queue = Self::queue_transient();
			if let Some(ValueStruct{ integer, boolean }) = queue.pop() {
				Self::deposit_event(RawEvent::Popped(integer, boolean));
			}

			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	/// Constructor function so we don't have to specify the types every time.
	///
	/// Constructs a ringbuffer transient and returns it as a boxed trait object.
	/// See [this part of the Rust book](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)
	fn queue_transient() -> Box<dyn RingBufferTrait<ValueStruct>> {
		Box::new(RingBufferTransient::<
			ValueStruct,
			<Self as Store>::BufferRange,
			<Self as Store>::BufferMap,
			BufferIndex,
		>::new())
	}
}
