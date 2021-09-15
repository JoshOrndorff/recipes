#![cfg_attr(not(feature = "std"), no_std)]

//! Transaction Weight Examples

use frame_support::{
	decl_module, decl_storage,
	dispatch::{DispatchResult, PaysFee, WeighData},
	ensure,
	weights::{ClassifyDispatch, DispatchClass, Pays, Weight},
};

pub trait Config: frame_system::Config {}

decl_storage! {
	trait Store for Module<T: Config> as SimpleMap {
		StoredValue get(fn stored_value): u32;
	}
}

// A "scale" to weigh transactions. This scale can be used with any transactions that take a
// single argument of type u32. The ultimate weight of the transaction is the / product of the
// transaction parameter and the field of this struct.
pub struct Linear(u32);

// The actual weight calculation happens in the `impl WeighData` block
impl WeighData<(&u32,)> for Linear {
	fn weigh_data(&self, (x,): (&u32,)) -> Weight {
		// Use saturation so that an extremely large parameter value
		// Does not cause overflow.
		x.saturating_mul(self.0).into()
	}
}

// The PaysFee trait indicates whether fees should actually be charged from the caller. If not,
// the weights are still applied toward the block maximums.
impl<T> PaysFee<T> for Linear {
	fn pays_fee(&self, _: T) -> Pays {
		Pays::Yes
	}
}

// Any struct that is used to weigh data must also implement ClassifyDispatchInfo. Here we classify
// the transaction as Normal (as opposed to operational.)
impl<T> ClassifyDispatch<T> for Linear {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		// Classify all calls as Normal (which is the default)
		Default::default()
	}
}

// Another scale to weight transactions. This one is more complex. / It computes weight according
// to the formula a*x^2 + b*y + c where / a, b, and c are fields in the struct, and x and y are
// transaction / parameters.
pub struct Quadratic(u32, u32, u32);

impl WeighData<(&u32, &u32)> for Quadratic {
	fn weigh_data(&self, (x, y): (&u32, &u32)) -> Weight {
		let ax2 = x.saturating_mul(*x).saturating_mul(self.0);
		let by = y.saturating_mul(self.1);
		let c = self.2;

		ax2.saturating_add(by).saturating_add(c).into()
	}
}

impl<T> ClassifyDispatch<T> for Quadratic {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		// Classify all calls as Normal (which is the default)
		Default::default()
	}
}

impl<T> PaysFee<T> for Quadratic {
	fn pays_fee(&self, _: T) -> Pays {
		Pays::Yes
	}
}

// A final scale to weight transactions. This one weighs transactions where the first parameter
// is bool. If the bool is true, then the weight is linear in the second parameter. Otherwise
// the weight is constant.
pub struct Conditional(u32);

impl WeighData<(&bool, &u32)> for Conditional {
	fn weigh_data(&self, (switch, val): (&bool, &u32)) -> Weight {
		if *switch {
			val.saturating_mul(self.0)
		} else {
			self.0
		}
		.into()
	}
}

impl<T> PaysFee<T> for Conditional {
	fn pays_fee(&self, _: T) -> Pays {
		Pays::Yes
	}
}

impl<T> ClassifyDispatch<T> for Conditional {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		// Classify all calls as Normal (which is the default)
		Default::default()
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Store value does not loop at all so a fixed weight is appropriate. Fixed weights can
		// be assigned using integer constants. No custom coding is
		// necessary.
		#[weight = 10_000]
		fn store_value(_origin, entry: u32) -> DispatchResult {

			StoredValue::put(entry);

			Ok(())
		}

		// WARNING: The functions that follow, allow the caller to control the
		// amount of computation being performed. This is ONLY SAFE when using
		// custom weighting structs as shown here.

		// add_n sets the storage value n times, so it should cost n times as much as
		// store_value. Because it performs both a read and a write, the multiplier is set to 200
		// instead of 100 as before.
		#[weight = Linear(200)]
		fn add_n(_origin, n: u32) -> DispatchResult {

			let mut old : u32;
			for _i in 1..=n {
				old = StoredValue::get();
				StoredValue::put(old + 1);
			}
			Ok(())
		}

		// The actual expense of `double` is proportional to a storage value. Dispatch
		// weightings can't use storage values directly, because the weight should be computable
		// ahead of time. Instead we have the caller pass in the expected storage value and we
		// ensure it is correct.
		#[weight = Linear(200)]
		fn double(_origin, initial_value: u32) -> DispatchResult {

			// Ensure the value passed by the caller actually matches storage If this condition
			// were not true, the caller would be able to avoid paying appropriate fees.
			let initial = StoredValue::get();
			ensure!(initial == initial_value, "Storage value did not match parameter");

			for _i in 1..=initial {
				let old = StoredValue::get();
				StoredValue::put(old + 1);
			}
			Ok(())
		}

		// This one is quadratic in the first argument plus linear in the second plus a constant.
		// This calculation is not meant to do something really useful or common other than
		// demonstrate that weights should grow by the same order as the compute required by the
		// transaction.
		#[weight = Quadratic(200, 30, 100)]
		fn complex_calculations(_origin, x: u32, y: u32) -> DispatchResult {
			// This first part performs a relatively cheap (hence 30)
			// in-memory calculations.
			let mut part1 = 0;
			for _i in 1..=y {
				part1 += 2
			}

			// The second part performs x^2 storage read-writes (hence 200)
			for _j in 1..=x {
				for _k in 1..=x {
					StoredValue::put(StoredValue::get() + 1);
				}
			}

			// One final storage write (hence 100)
			StoredValue::put(part1);

			Ok(())
		}

		// Here the first parameter, a boolean has a significant effect on the computational
		// intensity of the call.
		#[weight = Conditional(200)]
		fn add_or_set(_origin, add_flag: bool, val: u32) -> DispatchResult {
			if add_flag {
				for _i in 1..=val {
					StoredValue::put(StoredValue::get());
				}
			}
			else {
				StoredValue::put(&val);
			}

			Ok(())
		}
	}
}
