//! Pallet that demonstrates a minimal access control check. When a user calls this pallet's
//! only dispatchable function, `check_membership`, the caller is checked against a set of approved
//! callers. If the caller is a member of the set, the pallet's `IsAMember` event is emitted. Otherwise a `NotAMember` error is returned.
//!
//! The list of approved members is provided by an external source and exposed through an associated
//! type in this pallet's configuration trait. Any type that implements the `AccountSet` trait can be
//! used to supply the membership set.

#![allow(clippy::unused_unit)]
pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use account_set::AccountSet;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// The pallet's configuration trait
	/// Notice the loose coupling: any pallet that implements the `AccountSet` behavior works here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The ubiquitous event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// A type that will supply a set of members to check access control against
		type MembershipSource: AccountSet<AccountId = Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The caller is a member.
		IsAMember(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The caller is not a member
		NotAMember,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Checks whether the caller is a member of the set of account IDs provided by the
		/// MembershipSource type. Emits an event if they are, and errors if not.
		#[pallet::weight(10_000)]
		pub fn check_membership(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			// Get the members from the `vec-set` pallet
			let members = T::MembershipSource::accounts();

			// Check whether the caller is a member
			ensure!(members.contains(&caller), Error::<T>::NotAMember);

			// If the previous call didn't error, then the caller is a member, so emit the event
			Self::deposit_event(Event::IsAMember(caller));
			Ok(().into())
		}
	}
}
