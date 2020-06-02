#![cfg_attr(not(feature = "std"), no_std)]

//! A FRAME pallet that handles storage migrations for the migrations station runtime

use frame_support::{decl_module, decl_event, Hashable, traits::Len};
use frame_system::{self as system};
use frame_support::storage::migration::{take_storage_value, put_storage_value};
use sp_std::vec::Vec;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

// The pallet's events
decl_event!(
	pub enum Event {
		Started,
		Read,
		Migrated,
		Finished,
	}
);

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		fn on_runtime_upgrade() -> frame_support::weights::Weight {

			Self::deposit_event(Event::Started);

			// Read the current members out of the old storage location storage
			// Take removes them from the old location
			let vec_of_members: Option<Vec<T::AccountId>> = take_storage_value(b"VecMap", b"Members", &[]);
			Self::deposit_event(Event::Read);

			// Insert the size of the map
			put_storage_value(b"MapSet", b"MemberCount", &[], vec_of_members.len() as u32);
			Self::deposit_event(Event::Finished);

			// Iterate over the existing members, writing them to
			for member in vec_of_members {
				let map_key = member.blake2_128_concat();
				put_storage_value(b"MapSet", b"Members", &map_key, true);
				Self::deposit_event(Event::Migrated);
			}

			1_000 // In reality the weight of a migration should be determined by benchmarking
		}
	}
}
