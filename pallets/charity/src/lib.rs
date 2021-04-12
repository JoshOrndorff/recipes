//! A Simple Charity which holds and governs a pot of funds.
//!
//! The Charity has a pot of funds. The Pot is unique because unlike other token-holding accounts,
//! it is not controlled by a cryptographic keypair. Rather it belongs to the pallet itself.
//! Funds can be added to the pot in two ways:
//! * Anyone can make a donation through the `donate` extrinsic.
//! * An imablance can be absorbed from somewhere else in the runtime.
//! Funds can only be allocated by a root call to the `allocate` extrinsic/
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::AccountIdConversion, ModuleId};
use sp_std::prelude::*;

use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult},
	traits::{Currency, ExistenceRequirement::AllowDeath, Imbalance, OnUnbalanced},
};
use frame_system::{ensure_root, ensure_signed};

#[cfg(test)]
mod tests;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Hardcoded pallet ID; used to create the special Pot Account
/// Must be exactly 8 characters long
const PALLET_ID: ModuleId = ModuleId(*b"Charity!");

pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	/// The currency type that the charity deals in
	type Currency: Currency<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Config> as SimpleTreasury {
		// No storage items of our own, but we still need decl_storage to initialize the pot
	}
	add_extra_genesis {
		build(|_config| {
			// Create the charity's pot of funds, and ensure it has the minimum required deposit
			let _ = T::Currency::make_free_balance_be(
				&<Module<T>>::account_id(),
				T::Currency::minimum_balance(),
			);
		});
	}
}

decl_event!(
	pub enum Event<T>
	where
		Balance = BalanceOf<T>,
		<T as frame_system::Config>::AccountId,
	{
		/// Donor has made a charitable donation to the charity
		DonationReceived(AccountId, Balance, Balance),
		/// An imbalance from elsewhere in the runtime has been absorbed by the Charity
		ImbalanceAbsorbed(Balance, Balance),
		/// Charity has allocated funds to a cause
		FundsAllocated(AccountId, Balance, Balance),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Donate some funds to the charity
		#[weight = 10_000]
		fn donate(
			origin,
			amount: BalanceOf<T>
		) -> DispatchResult {
			let donor = ensure_signed(origin)?;

			T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

			Self::deposit_event(RawEvent::DonationReceived(donor, amount, Self::pot()));
			Ok(())
		}

		/// Allocate the Charity's funds
		///
		/// Take funds from the Charity's pot and send them somewhere. This call requires root origin,
		/// which means it must come from a governance mechanism such as Substrate's Democracy pallet.
		#[weight = 10_000]
		fn allocate(
			origin,
			dest: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Make the transfer requested
			T::Currency::transfer(
				&Self::account_id(),
				&dest,
				amount,
				AllowDeath,
			).map_err(|_| DispatchError::Other("Can't make allocation"))?;

			//TODO what about errors here??

			Self::deposit_event(RawEvent::FundsAllocated(dest, amount, Self::pot()));
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	/// The account ID that holds the Charity's funds
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	/// The Charity's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}
}

// This implementation allows the charity to be the recipient of funds that are burned elsewhere in
// the runtime. For eample, it could be transaction fees, consensus-related slashing, or burns that
// align incentives in other pallets.
impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
		let numeric_amount = amount.peek();

		// Must resolve into existing but better to be safe.
		let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

		Self::deposit_event(RawEvent::ImbalanceAbsorbed(numeric_amount, Self::pot()));
	}
}
