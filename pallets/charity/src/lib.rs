//! A Simple Charity which holds and governs a pot of funds.
//!
//! The Charity has a pot of funds. The Pot is unique because unlike other token-holding accounts,
//! it is not controlled by a cryptographic keypair. Rather it belongs to the pallet itself.
//! Funds can be added to the pot in two ways:
//! * Anyone can make a donation through the `donate` extrinsic.
//! * An imablance can be absorbed from somewhere else in the runtime.
//! Funds can only be allocated by a root call to the `allocate` extrinsic/

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;

use sp_runtime::{traits::AccountIdConversion, ModuleId};

use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

/// Hardcoded pallet ID; used to create the special Pot Account
/// Must be exactly 8 characters long
const PALLET_ID: ModuleId = ModuleId(*b"Charity!");

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::BalanceOf;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::AllowDeath},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency type that the charity deals in
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let _ = T::Currency::make_free_balance_be(
				&<Module<T>>::account_id(),
				T::Currency::minimum_balance(),
			);
		}
	}

	#[pallet::event]
	#[pallet::metadata(T::Balance = "Balance")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Donor has made a charitable donation to the charity
		DonationReceived(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// An imbalance from elsewhere in the runtime has been absorbed by the Charity
		ImbalanceAbsorbed(BalanceOf<T>, BalanceOf<T>),
		/// Charity has allocated funds to a cause
		FundsAllocated(T::AccountId, BalanceOf<T>, BalanceOf<T>),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Donate some funds to the charity
		#[pallet::weight(10_000)]
		pub fn donate(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let donor = ensure_signed(origin)?;

			T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

			Self::deposit_event(Event::DonationReceived(donor, amount, Self::pot()));
			Ok(().into())
		}

		/// Allocate the Charity's funds
		///
		/// Take funds from the Charity's pot and send them somewhere. This call requires root origin,
		/// which means it must come from a governance mechanism such as Substrate's Democracy pallet.
		#[pallet::weight(10_000)]
		pub fn allocate(
			origin: OriginFor<T>,
			dest: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			// Make the transfer requested
			T::Currency::transfer(&Self::account_id(), &dest, amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make allocation"))?;

			//TODO what about errors here??

			Self::deposit_event(Event::FundsAllocated(dest, amount, Self::pot()));
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
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

		Self::deposit_event(Event::ImbalanceAbsorbed(numeric_amount, Self::pot()));
	}
}
