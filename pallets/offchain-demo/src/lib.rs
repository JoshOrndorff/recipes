#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

use support::{
	debug,
	dispatch::DispatchResult, decl_module, decl_storage, decl_event,
	traits::Get,
	weights::SimpleDispatchInfo,
};

use core::convert::{TryInto};

use system::{self as system, ensure_signed, ensure_none, offchain};
// use serde_json as json;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	offchain::{http},
	traits::Zero,
	transaction_validity::{InvalidTransaction, ValidTransaction, TransactionValidity},
};
use sp_std::prelude::*;

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub const NUM_VEC_LEN: usize = 10;

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod crypto {
	use crate::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, KEY_TYPE);
}

/// This is the pallet's configuration trait
pub trait Trait: system::Trait {
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// The type to sign and submit transactions.
	type SubmitSignedTransaction:
		offchain::SubmitSignedTransaction<Self, <Self as Trait>::Call>;
	/// The type to submit unsigned transactions.
	type SubmitUnsignedTransaction:
		offchain::SubmitUnsignedTransaction<Self, <Self as Trait>::Call>;
	type GracePeriod: Get<Self::BlockNumber>;
}

// Custom data type
#[derive(Debug)]
enum TransactionType {
	Signed,
	Unsigned,
	None,
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		/// A vector of recently submitted prices. Should be bounded
		Numbers get(fn numbers): Vec<u32>;

		/// Defines the block when next unsigned transaction will be accepted.
		NextTx get(fn next_tx): T::BlockNumber;
		AddSeq get(fn add_seq): u32;
	}
}

decl_event!(
	/// Events generated by the module.
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Event generated when new price is accepted to contribute to the average.
		NewNumber(Option<AccountId>, u32),
	}
);

decl_module! {

	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = SimpleDispatchInfo::FixedNormal(1000)]
		pub fn submit_number_signed(origin, number: u32) -> DispatchResult {
			debug::native::info!("submit_number_signed: {:?}", number);
			let who = ensure_signed(origin)?;
			Self::append_or_replace_number(Some(who), number)
		}

		#[weight = SimpleDispatchInfo::FixedNormal(1000)]
		pub fn submit_number_unsigned(origin, _block: T::BlockNumber, number: u32) -> DispatchResult {
			debug::native::info!("submit_number_unsigned: {:?}", number);
			let _ = ensure_none(origin)?;
			Self::append_or_replace_number(None, number)
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::native::info!("Entering off-chain workers");

			let res = match Self::choose_tx_type(block_number) {
				TransactionType::Signed => Self::send_signed(block_number),
				TransactionType::Unsigned => Self::send_unsigned(block_number),
				TransactionType::None => Ok(())
			};

			if let Err(e) = res { debug::native::error!("Error: {}", e); }
		}
	}
}

impl<T: Trait> Module<T> {
	/// Add new price to the list.
	fn append_or_replace_number(who: Option<T::AccountId>, number: u32) -> DispatchResult {
		let current_seq = Self::add_seq();
		Numbers::mutate(|numbers| {
			if (current_seq as usize) < NUM_VEC_LEN {
				numbers.push(number);
			} else {
				numbers[current_seq as usize % NUM_VEC_LEN] = number;
			}

			// displaying the average
			let average = numbers.iter().fold(0, {|acc, num| acc + num}) / (numbers.len() as u32);
			debug::native::info!("Current average of numbers is: {}", average);
		});


		// Update the storage & seq for next update block
		<NextTx<T>>::mutate(|block| *block += T::GracePeriod::get());
		<AddSeq>::mutate(|seq| *seq += 1);

		// Raise the NewPrice event
		Self::deposit_event(RawEvent::NewNumber(who, number));
		Ok(())
	}

	fn choose_tx_type(block_number: T::BlockNumber) -> TransactionType {
		let next_tx = Self::next_tx();
		// Do not perform transaction if still within grace period
		if next_tx > block_number { return TransactionType::None; }

		if Self::add_seq() % 2 == 0 {
			TransactionType::Signed
		} else {
			TransactionType::Unsigned
		}
	}

	fn send_signed(block_number: T::BlockNumber) -> Result<(), &'static str> {
		use system::offchain::SubmitSignedTransaction;
		if !T::SubmitSignedTransaction::can_sign() {
			return Err("No local account available");
		}

		let submission: u32 = block_number.try_into().ok().unwrap() as u32;
		let call = Call::submit_number_signed(submission);

		// Using `SubmitSignedTransaction` associated type we create and submit a transaction
		// representing the call, we've just created.
		// Submit signed will return a vector of results for all accounts that were found in the
		// local keystore with expected `KEY_TYPE`.
		let results = T::SubmitSignedTransaction::submit_signed(call);
		for (acc, res) in &results {
			match res {
				Ok(()) => { debug::native::info!("off-chain send_signed: acc: {}| number: {}", acc, submission); },
				Err(e) => { debug::native::error!("[{:?}] Failed to submit signed tx: {:?}", acc, e); }
			};
		}
		Ok(())
	}

	fn send_unsigned(block_number: T::BlockNumber) -> Result<(), &'static str> {
		use system::offchain::SubmitUnsignedTransaction;

		let submission: u32 = block_number.try_into().ok().unwrap() as u32;
		let call = Call::submit_number_unsigned(block_number, submission);

		T::SubmitUnsignedTransaction::submit_unsigned(call).map_err(|e| {
			debug::native::error!("Failed to submit unsigned tx: {:?}", e);
			"Unable to submit unsigned transaction".into()
		})
	}
}

#[allow(deprecated)] // ValidateUnsigned
impl<T: Trait> support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(call: &Self::Call) -> TransactionValidity {
		if let Call::submit_number_unsigned(block_num, number) = call {
			debug::native::info!("off-chain send_unsigned: block_num: {}| number: {}", block_num, number);

			Ok(ValidTransaction {
				priority: 1 << 20,
				requires: vec![],
				provides: vec![codec::Encode::encode(&(KEY_TYPE.0, block_num))],
				longevity: 3,
				propagate: false,
			})
		} else {
			InvalidTransaction::Call.into()
		}
	}
}
