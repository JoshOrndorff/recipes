//! Pallet that demonstrates a minimal access control check. When a user calls this pallet's
//! only dispatchable function, `check_membership`, the caller is checked against a set of approved
//! callers. If the caller is a member of the set, the pallet's `IsAMember` event is emitted. Otherwise a `NotAMember` error is returned.
//!
//! The list of approved members is provided by the `vec-set` pallet. In order for this pallet to be
//! used, the `vec-set` pallet must also be present in the runtime.

use frame_support::{decl_error, decl_event, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
/// Notice the explicit tight coupling to the `vec-set` pallet
pub trait Config: frame_system::Config + vec_set::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// The caller is a member.
		IsAMember(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The caller is not a member
		NotAMember,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Checks whether the caller is a member of the set of account IDs provided by the `vec-set`
		/// pallet. Emits an event if they are, and errors if not.
		#[weight = 10_000]
		fn check_membership(origin) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			// Get the members from the `vec-set` pallet
			let members = vec_set::Module::<T>::members();

			// Check whether the caller is a member
			members.binary_search(&caller).map_err(|_| Error::<T>::NotAMember)?;

			// If the previous call didn't error, then the caller is a member, so emit the event
			Self::deposit_event(RawEvent::IsAMember(caller));
			Ok(())
		}
	}
}
