#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet that demonstrates Fixed Point arithmetic
use runtime_primitives::traits::Zero;
use ap_arithmetic::Percent;
use support::traits::Get;
use support::{
	decl_event,
	decl_module,
	decl_storage,
	dispatch::{DispatchResult, DispatchError},
	ensure,
	StorageValue
};
use system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
	//TODO if this becomes a financial example, maybe import a Currency
}

type Balance = u64;
type BlockNumber = u64;

pub struct CompoundingAccountData {
	/// The balance of the account after last manual adjustment
	principle: Balance,
	/// The time (block height) at which the balance was last adjusted
	deposit_date: BlockNumber,
}

impl CompoundingAccountData {
	/// Compute the current value of the account by applying the interest accrued since
	/// the last time the acocunt was touched.
	fn apply_interest(self, now: &BlockNumber) -> Balance {
		let elapsed_time = now - self.deposit_date;

		//TODO calculate the interest as self.principle * e^???
		let interest = 0;

		self.principle + interest
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		/// The interest rate for the accounts
		InterestRate get(fn rate): Percent = Percent::from_percent(5);
		/// Balance for the continuously compounded account
		CompoundingAccount get(fn balance_compound): AccountData;
		/// Balance for the discrete interest account
		DiscreteAccount get(fn discrete_account): Balance;
	}
}

decl_event!(
	pub enum Event {
		/// Deposited some balance into the compounding interest account
		DepositedCompounding(Balance),
		/// Withdrew some balance from the compounding interest account
		WithdrewCompounding(Balance),
		/// Deposited some balance into the discrete interest account
		DepositedDiscrete(Balance),
		/// Withdrew some balance from the discrete interest account
		WithdrewDiscrete(Balance),
		/// Some interest has been applied to the discrete interest account
		/// This happens every ten blocks
		DiscreteInterestApplied(Balance),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Deposit some funds into the compounding interest account
		fn deposit_compounding(origin, val_to_add: Balance) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let current_block = system::current_block();

			let old_value = CompoundingAccount::get().apply_interest(&current_block);
			let new_data = AccountData {
				balance: old_value + val_to_add,
				deposit_date: current_block,
			};

			CompoundingInterest::put(new_data);

			Self::deposit_event(Event::DepositedCompounding(val_to_add));
			Ok(())
		}

		/// Withdraw some funds from the compounding interest account
		fn withdraw_compounding(origin, val_to_take: Balance) -> DispatchResult {
			//TODO
			Ok(())
		}

		/// Deposit some funds into the discrete interest account
		fn deposit_discrete(origin, val_to_add: Balance) -> DidpatchResult {
			//TODO
			Ok(())
		}

		/// Withdraw some funds from the discrete interest account
		fn withdraw_discrete(origin, val_to_take: Balance) -> DispatchResult {
			//TODO
			Ok(())
		}

		fn on_finalize(n: T::BlockNumber) {
			//TODO apply new interest every ten blocks
			if (n % 10).is_zero() {
				let interest = DiscreteAccount::get() * InterestRate::get();
				Self::deposit_event(Event::InterestApplied(&interest));
				Disc
			}
		}
	}
}
