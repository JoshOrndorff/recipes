//! Generating (insecure) randomness
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
pub use pallet::*;
use parity_scale_codec::Encode;
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_support::{sp_runtime::app_crypto::sp_core::H256, traits::Randomness};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The pallet doesn't know what the source of randomness is; it can be anything that
		/// implements the trait. When installing this pallet in a runtime, you
		/// must make sure to give it a randomness source that suits its needs.
		type RandomnessSource: Randomness<H256>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// First element is raw seed, second is using nonce.
		RandomnessConsumed(H256, H256),
	}

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Grab a random seed and random value from the randomness collective flip pallet
		#[pallet::weight(10_000)]
		pub fn consume_randomness(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			// Using a subject is recommended to prevent accidental re-use of the seed
			// (This does not add security or entropy)
			let subject = Self::encode_and_update_nonce();

			let random_seed = T::RandomnessSource::random_seed();
			let random_result = T::RandomnessSource::random(&subject);

			Self::deposit_event(Event::RandomnessConsumed(random_seed, random_result));
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Reads the nonce from storage, increments the stored nonce, and returns
	/// the encoded nonce to the caller.
	fn encode_and_update_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		Nonce::<T>::put(nonce.wrapping_add(1));
		nonce.encode()
	}
}
