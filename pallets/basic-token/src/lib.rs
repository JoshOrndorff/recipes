#![cfg_attr(not(feature = "std"), no_std)]

//! Simple Token Transfer
//! 1. set total supply
//! 2. establish ownership upon configuration of circulating tokens
//! 3. coordinate token transfers with the runtime functions
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as Token {
		pub Balances get(fn get_balance): map hasher(blake2_128_concat) T::AccountId => u64;

		pub TotalSupply get(fn total_supply): u64 = 21000000;

		Init get(fn is_init): bool;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// Token was initialized by user
		Initialized(AccountId),
		/// Tokens successfully transferred between users
		Transfer(AccountId, AccountId, u64), // (from, to, value)
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Attempted to initialize the token after it had already been initialized.
		AlreadyInitialized,
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Initialize the token
		/// transfers the total_supply amout to the caller
		#[weight = 10_000]
		fn init(origin) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Self::is_init(), <Error<T>>::AlreadyInitialized);

			<Balances<T>>::insert(sender, Self::total_supply());

			Init::put(true);
			Ok(())
		}

		/// Transfer tokens from one account to another
		#[weight = 10_000]
		fn transfer(_origin, to: T::AccountId, value: u64) -> DispatchResult {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances
			let updated_from_balance = sender_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance.checked_add(value).expect("Entire supply fits in u64; qed");

			// Write new balances to storage
			<Balances<T>>::insert(&sender, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(RawEvent::Transfer(sender, to, value));
			Ok(())
		}
	}
}
