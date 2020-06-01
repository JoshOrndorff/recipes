#![cfg_attr(not(feature = "std"), no_std)]

//! A FRAME pallet that handles storage migrations for the migrations station runtime

use frame_support::{decl_module, decl_event, dispatch};
use frame_system::{self as system, ensure_signed};
use frame_support::storage::migration::{take_storage_value, put_storage_value};

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		Started,
		Read,
		Migrated(AccountId),
		Finished,
	}
);

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		fn on_runtime_upgrade() -> frame_support::weights::Weight {

			Self::deposit_event(RawEvent::Started);

			// Read the current members out of the old storage location storage
			// Take removes them from the old location
			let vec_of_members: Option<Vec<T::AccountId>> = take_storage_value(b"VecMap", b"Members", &[]);
			Self::deposit_event(RawEvent::Read);

			// Iterate over the existing members, writing them to
			for member in vec_of_members {
				let map_key = member.blake2_128_concat();
				put_storage_value(b"MapSet", b"Members", &map_key, true);
				Self::deposit_event(RawEvent::Migrated(&member));
			}

			/// Insert the size of the map
			put_storage_value(b"MapSet", b"MemberCount", &member.encode(), vec_of_members.len() as u32);
			Self::deposit_event(RawEvent::Finished);

			1_000 // In reality the weight of a migration should be determined by benchmarking
		}
	}
}
