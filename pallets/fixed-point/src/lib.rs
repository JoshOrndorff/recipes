#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet that demonstrates Fixed Point arithmetic
use parity_scale_codec::{Encode, Decode};
use sp_runtime::traits::Zero;
use sp_arithmetic::{Percent, traits::AtLeast32Bit};
use frame_support::{
	decl_event,
	decl_module,
	decl_storage,
	dispatch::DispatchResult,
};
use frame_system::{self as system, ensure_signed};
use substrate_fixed::{
	transcendental::exp,
	types::U32F32,
};

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

/// Special Balance type for this fixed point pallet
type FPBalance = u64;

#[derive(Encode, Decode, Default)]
pub struct ContinuousAccountData<BlockNumber> {
	/// The balance of the account after last manual adjustment
	principle: FPBalance,
	/// The time (block height) at which the balance was last adjusted
	deposit_date: BlockNumber,
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		/// Balance for the continuously compounded account
		ContinuousAccount get(fn balance_compound): ContinuousAccountData<T::BlockNumber>;
		/// Balance for the discrete interest account
		DiscreteAccount get(fn discrete_account): FPBalance;
	}
}

decl_event!(
	pub enum Event {
		/// Deposited some balance into the compounding interest account
		DepositedContinuous(FPBalance),
		/// Withdrew some balance from the compounding interest account
		WithdrewContinuous(FPBalance),
		/// Deposited some balance into the discrete interest account
		DepositedDiscrete(FPBalance),
		/// Withdrew some balance from the discrete interest account
		WithdrewDiscrete(FPBalance),
		/// Some interest has been applied to the discrete interest account
		/// The associated data is just the interest amout (not the new or old balance)
		/// This happens every ten blocks
		DiscreteInterestApplied(FPBalance),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Deposit some funds into the compounding interest account
		fn deposit_compounding(origin, val_to_add: FPBalance) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let current_block = system::Module::<T>::block_number();
			let old_value = Self::value_of_continuous_account(&current_block);

			// Update storage for compounding account
			ContinuousAccount::<T>::put(
				ContinuousAccountData {
					principle: old_value + val_to_add,
					deposit_date: current_block,
				}
			);

			// Emit event
			Self::deposit_event(Event::DepositedContinuous(val_to_add));
			Ok(())
		}

		/// Withdraw some funds from the compounding interest account
		fn withdraw_compounding(origin, val_to_take: FPBalance) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let current_block = system::Module::<T>::block_number();
			let old_value = Self::value_of_continuous_account(&current_block);

			// Update storage for compounding account
			ContinuousAccount::<T>::put(
				ContinuousAccountData {
					principle: old_value - val_to_take,
					deposit_date: current_block,
				}
			);

			// Emit event
			Self::deposit_event(Event::WithdrewContinuous(val_to_take));
			Ok(())
		}

		/// Deposit some funds into the discrete interest account
		fn deposit_discrete(origin, val_to_add: FPBalance) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let old_value = DiscreteAccount::get();

			// Update storage for compounding account
			DiscreteAccount::put(old_value + val_to_add);

			// Emit event
			Self::deposit_event(Event::DepositedDiscrete(val_to_add));
			Ok(())
		}

		/// Withdraw some funds from the discrete interest account
		fn withdraw_discrete(origin, val_to_take: FPBalance) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let old_value = DiscreteAccount::get();

			// Update storage for compounding account
			DiscreteAccount::put(old_value - val_to_take);

			// Emit event
			Self::deposit_event(Event::WithdrewDiscrete(val_to_take));
			Ok(())
		}

		fn on_finalize(n: T::BlockNumber) {
			// Apply newly-accrued discrete interest every ten blocks
			if (n % 10.into()).is_zero() {

				// Calculate interest
				// We can use the `*` operator for multiplying a `Percent` by a u64
				// because `Percent` implements the trait Mul<u64>
				let interest = Self::discrete_interest_rate() * DiscreteAccount::get();

				// The following line, although similar, does not work because
				// u64 does not implement the trait Mul<Percent>
				// let interest = DiscreteAccount::get() * Self::discrete_interest_rate();

				// Update the balance
				let old_balance = DiscreteAccount::get();
				DiscreteAccount::put(old_balance + interest);

				// Emit the event
				Self::deposit_event(Event::DiscreteInterestApplied(interest));
			}
		}
	}
}

impl<T: Trait> Module<T> {
	/// A helper function to evaluate the current value of the continuously compounding interest
	/// account
	fn value_of_continuous_account(now: &<T as system::Trait>::BlockNumber) -> FPBalance {
		let ContinuousAccountData{
			principle,
			deposit_date,
		} = ContinuousAccount::<T>::get();


		// let elapsed_time = *now - deposit_date;
		// let elapsed_time : u32 = 5;
		let elapsed_time = 5; // Why can I multiply by integer, but not u32
		// How can I actually multiply by the elapsed time?
		let exponent = Self::continuous_interest_rate() * elapsed_time;


		// principle * exp(Self::continuous_interest_rate() * elapsed_time)
		1 //TODO
	}

	/// A helper function to return the hard-coded 5% interest rate
	fn discrete_interest_rate() -> Percent {
		Percent::from_percent(5)
	}

	/// A helper function to return the hard-coded 5% interest rate
	fn continuous_interest_rate() -> U32F32 {
		// 1 / 20 is 5%. Same interest rate as the discrete account, but in the
		// fancy substrate-fixed format. This U32F32 type represents a 32 bit
		// unsigned integer where all 32 bits are fractional.
		U32F32::from_num(1) / 20
	}
}
