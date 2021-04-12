#![cfg_attr(not(feature = "std"), no_std)]

//! A pallet that demonstrates Fixed Point arithmetic in the context of two simple bank accounts
//! that accrue compounding interest.
//!
//! The discrete account accrues interest every ten blocks and is implemented using
//! Substrate's `Percent` implementation of fixed point.
//!
//! The continuous account accrues interest continuously and is implemented using
//! Substrate-fixed's `I32F32` implementation of fixed point.

use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;
use parity_scale_codec::{Decode, Encode};
use sp_arithmetic::Percent;
use sp_runtime::traits::Zero;
use sp_std::convert::TryInto;
use substrate_fixed::{transcendental::exp, types::I32F32};

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default)]
pub struct ContinuousAccountData<BlockNumber> {
	/// The balance of the account after last manual adjustment
	principal: I32F32,
	/// The time (block height) at which the balance was last adjusted
	deposit_date: BlockNumber,
}

decl_storage! {
	trait Store for Module<T: Config> as CompoundingInterest {
		/// Balance for the continuously compounded account
		ContinuousAccount get(fn balance_compound): ContinuousAccountData<T::BlockNumber>;
		/// Balance for the discrete interest account
		DiscreteAccount get(fn discrete_account): u64;
	}
}

decl_event!(
	pub enum Event {
		/// Deposited some balance into the compounding interest account
		DepositedContinuous(u64),
		/// Withdrew some balance from the compounding interest account
		WithdrewContinuous(u64),
		/// Deposited some balance into the discrete interest account
		DepositedDiscrete(u64),
		/// Withdrew some balance from the discrete interest account
		WithdrewDiscrete(u64),
		/// Some interest has been applied to the discrete interest account
		/// The associated data is just the interest amout (not the new or old balance)
		/// This happens every ten blocks
		DiscreteInterestApplied(u64),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Deposit some funds into the compounding interest account
		#[weight = 10_000]
		fn deposit_continuous(origin, val_to_add: u64) -> DispatchResult {
			ensure_signed(origin)?;

			let current_block = frame_system::Module::<T>::block_number();
			let old_value = Self::value_of_continuous_account(&current_block);

			// Update storage for compounding account
			ContinuousAccount::<T>::put(
				ContinuousAccountData {
					principal: old_value + I32F32::from_num(val_to_add),
					deposit_date: current_block,
				}
			);

			// Emit event
			Self::deposit_event(Event::DepositedContinuous(val_to_add));
			Ok(())
		}

		/// Withdraw some funds from the compounding interest account
		#[weight = 10_000]
		fn withdraw_continuous(origin, val_to_take: u64) -> DispatchResult {
			ensure_signed(origin)?;

			let current_block = frame_system::Module::<T>::block_number();
			let old_value = Self::value_of_continuous_account(&current_block);

			// Update storage for compounding account
			ContinuousAccount::<T>::put(
				ContinuousAccountData {
					principal: old_value - I32F32::from_num(val_to_take),
					deposit_date: current_block,
				}
			);

			// Emit event
			Self::deposit_event(Event::WithdrewContinuous(val_to_take));
			Ok(())
		}

		/// Deposit some funds into the discrete interest account
		#[weight = 10_000]
		fn deposit_discrete(origin, val_to_add: u64) -> DispatchResult {
			ensure_signed(origin)?;

			let old_value = DiscreteAccount::get();

			// Update storage for discrete account
			DiscreteAccount::put(old_value + val_to_add);

			// Emit event
			Self::deposit_event(Event::DepositedDiscrete(val_to_add));
			Ok(())
		}

		/// Withdraw some funds from the discrete interest account
		#[weight = 10_000]
		fn withdraw_discrete(origin, val_to_take: u64) -> DispatchResult {
			ensure_signed(origin)?;

			let old_value = DiscreteAccount::get();

			// Update storage for discrete account
			DiscreteAccount::put(old_value - val_to_take);

			// Emit event
			Self::deposit_event(Event::WithdrewDiscrete(val_to_take));
			Ok(())
		}

		fn on_finalize(n: T::BlockNumber) {
			// Apply newly-accrued discrete interest every ten blocks
			if (n % 10u32.into()).is_zero() {

				// Calculate interest Interest = principal * rate * time
				// We can use the `*` operator for multiplying a `Percent` by a u64
				// because `Percent` implements the trait Mul<u64>
				let interest = Self::discrete_interest_rate() * DiscreteAccount::get() * 10;

				// The following line, although similar, does not work because
				// u64 does not implement the trait Mul<Percent>
				// let interest = DiscreteAccount::get() * Self::discrete_interest_rate() * 10;

				// Update the balance
				let old_balance = DiscreteAccount::get();
				DiscreteAccount::put(old_balance + interest);

				// Emit the event
				Self::deposit_event(Event::DiscreteInterestApplied(interest));
			}
		}
	}
}

impl<T: Config> Module<T> {
	/// A helper function to evaluate the current value of the continuously compounding interest
	/// account
	fn value_of_continuous_account(now: &<T as frame_system::Config>::BlockNumber) -> I32F32 {
		// Get the old state of the accout
		let ContinuousAccountData {
			principal,
			deposit_date,
		} = ContinuousAccount::<T>::get();

		// Calculate the exponential function (lots of type conversion)
		let elapsed_time_block_number = *now - deposit_date;
		let elapsed_time_u32: u32 = TryInto::try_into(elapsed_time_block_number)
			.ok()
			.expect("blockchain will not exceed 2^32 blocks; qed");
		let elapsed_time_i32f32 = I32F32::from_num(elapsed_time_u32);
		let exponent: I32F32 = Self::continuous_interest_rate() * elapsed_time_i32f32;
		let exp_result : I32F32 = exp(exponent)
			.expect("Interest will not overflow account (at least not until the learner has learned enough about fixed point :)");

		// Return the result interest = principal * e ^ (rate * time)
		principal * exp_result
	}

	/// A helper function to return the hard-coded 5% interest rate
	fn discrete_interest_rate() -> Percent {
		Percent::from_percent(5)
	}

	/// A helper function to return the hard-coded 5% interest rate
	fn continuous_interest_rate() -> I32F32 {
		// 1 / 20 is 5%. Same interest rate as the discrete account, but in the
		// fancy substrate-fixed format. This I32F32 type represents a 32 bit
		// signed integer where all 32 bits are fractional.
		I32F32::from_num(1) / 20
	}
}
