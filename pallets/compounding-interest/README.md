---
slug: compounding-interest
lan: en
title: Compounding Interest
public_url: https://github.com/substrate-developer-hub/recipes/tree/master/pallets/compounding-interest
---

Many financial agreements involve interest for loaned or borrowed money. [Compounding interest](https://en.wikipedia.org/wiki/Compound_interest) is when new interest is paid on top of not only the original loan amount, the so-called "principal", but also any interest that has been previously paid. In this recipe we will use [fixed point arithmetic](../fixed-point/README.md) to track non-integer account balances in the Substrate runtime.

### Discrete Compounding

Our first example will look at discrete compounding interest. This is when interest is paid at a fixed interval. In our case, interest will be paid every ten blocks.

For this implementation we've chosen to use Substrate's [`Percent` type](https://substrate.dev/rustdocs/master/sp_arithmetic/struct.Percent.html). It works nearly the same as `Permill`, but it represents numbers as "parts per hundred" rather than "parts per million". We could also have used Substrate-fixed for this implementation, but chose to save it for the next example.

The only storage item needed is a tracker of the account's balance. In order to focus on the fixed-point- and interest-related topics, this pallet does not actually interface with a `Currency`. Instead we just allow anyone to "deposit" or "withdraw" funds with no source or destination.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		// --snip--

		/// Balance for the discrete interest account
		DiscreteAccount get(fn discrete_account): u64;
	}
}
```

There are two extrinsics associated with the discrete interest account. The `deposit_discrete` extrinsic is shown here, and the `withdraw_discrete` extrinsic is nearly identical. Check it out in the kitchen.

```rust, ignore
fn deposit_discrete(origin, val_to_add: u64) -> DispatchResult {
	ensure_signed(origin)?;

	let old_value = DiscreteAccount::get();

	// Update storage for discrete account
	DiscreteAccount::put(old_value + val_to_add);

	// Emit event
	Self::deposit_event(Event::DepositedDiscrete(val_to_add));
	Ok(())
}
```

The flow of these deposit and withdraw extrinsics is entirely straight-forward. They each perform a simple addition or substraction from the stored value, and they have nothing to do with interest.

Because the interest is paid discretely every ten blocks it can be handled independently of deposits and withdrawals. The interest calculation happens automatically in the `on_finalize` block.

```rust, ignore
fn on_finalize(n: T::BlockNumber) {
	// Apply newly-accrued discrete interest every ten blocks
	if (n % 10.into()).is_zero() {

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
```

`on_finalize` is called at the end of every block, but we only want to pay interest every ten blocks, so the first thing we do is check whether this block is a multiple of ten. If it is we calculate the interest due by the formula `interest = principal * rate * time`. As the comments explain, there is some subtlety in the order of the multiplication. You can multiply `PerCent * u64` but not `u64 * PerCent`.


### Continuously Compounding

You can imagine increasing the frequency at which the interest is paid out. Increasing the frequency enough approaches [continuously compounding interest](https://en.wikipedia.org/wiki/Compound_interest#Continuous_compounding). Calculating continuously compounding interest requires the [exponential function](https://en.wikipedia.org/wiki/Exponential_function) which is not available using Substrate's `PerThing` types. Luckily exponential and other [transcendental functions](https://en.wikipedia.org/wiki/Transcendental_function) are available in substrate-fixed, which is why we've chosen to use it for this example.

With continuously compounded interest, we _could_ update the interest in `on_finalize` as we did before, but it would need to be updated every single block. Instead we wait until a user tries to use the account (to deposit or withdraw funds), and then calculate the account's current value "just in time".

To facilitate this implementation, we represent the state of the account not only as a balance, but as a balance, paired with the time when that balance was last updated.

```rust, ignore
#[derive(Encode, Decode, Default)]
pub struct ContinuousAccountData<BlockNumber> {
	/// The balance of the account after last manual adjustment
	principal: I32F32,
	/// The time (block height) at which the balance was last adjusted
	deposit_date: BlockNumber,
}
```

You can see we've chosen substrate-fixed's `I32F32` as our balance type this time. While we don't intend to handle negative balances, there is currently a limitation in the transcendental functions that requires using signed types.

With the struct to represent the account's state defined, we can initialize the storage value.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		// --snip--

		/// Balance for the continuously compounded account
		ContinuousAccount get(fn balance_compound): ContinuousAccountData<T::BlockNumber>;
	}
}
```

As before, there are two relevant extrinsics, `deposit_continuous` and `withdraw_continuous`. They are nearly identical so we'll only show one.

```rust, ignore
fn deposit_continuous(origin, val_to_add: u64) -> DispatchResult {
	ensure_signed(origin)?;

	let current_block = system::Module::<T>::block_number();
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
```

This function itself isn't too insightful. It does the same basic things as the discrete variant: look up the old value and the deposit, update storage, and emit an event. The one interesting part is that it calls a helper function to get the account's previous value. This helper function calculates the value of the account considering all the interest that has accrued since the last time the account was touched. Let's take a closer look.

```rust, ignore
fn value_of_continuous_account(now: &<T as system::Trait>::BlockNumber) -> I32F32 {
	// Get the old state of the accout
	let ContinuousAccountData{
		principal,
		deposit_date,
	} = ContinuousAccount::<T>::get();

	// Calculate the exponential function (lots of type conversion)
	let elapsed_time_block_number = *now - deposit_date;
	let elapsed_time_u32 = TryInto::try_into(elapsed_time_block_number)
		.expect("blockchain will not exceed 2^32 blocks; qed");
	let elapsed_time_i32f32 = I32F32::from_num(elapsed_time_u32);
	let exponent : I32F32 = Self::continuous_interest_rate() * elapsed_time_i32f32;
	let exp_result : I32F32 = exp(exponent)
		.expect("Interest will not overflow account (at least not until the learner has learned enough about fixed point :)");

	// Return the result interest = principal * e ^ (rate * time)
	principal * exp_result
}
```

This function gets the previous state of the account, makes the interest calculation and returns the result. The reality of making these fixed point calculations is that type conversion will likely be your biggest pain point. Most of the lines are doing type conversion between the `BlockNumber`, `u32`, and `I32F32` types.

We've already seen that this helper function is used within the runtime for calculating the current balance "just in time" to make adjustments. In a real-world scenario, chain users would also want to check their balance at any given time. Because the current balance is not stored in runtime storage, it would be wise to [implement a runtime API](./runtime-api.md) so this helper can be called from outside the runtime.
