#![cfg_attr(not(feature = "std"), no_std)]

// Simple Storage Map
// https://crates.parity.io/sr_primitives/weights/index.html
use support::{ensure, decl_module, decl_storage, StorageMap, dispatch::{Result, WeighData}};
use system::ensure_signed;
use runtime_primitives::weights::{ DispatchClass, Weight, ClassifyDispatch, SimpleDispatchInfo };

pub trait Trait: system::Trait {}

decl_storage! {
	trait Store for Module<T: Trait> as SimpleMap {
		StoredValue get(stored_value): u32;
	}
}

// This struct is the "scale" that will weigh transactions
pub struct Linear(u32);

// Implement WeighData for a single u32 parameter
impl WeighData<(&u32,)> for Linear {
	fn weigh_data(&self, (x,): (&u32,)) -> Weight {
		// For now just hardcode a constant
		// TODO actually make it linear and return c*x with saturation
		5
	}
}

// We implement ClassifyDispatch a single time for all types
// That may need to be classified, because the result is
// always normal regardless of number or type of params.
impl<T> ClassifyDispatch<T> for Linear {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		// Classify all calls as Normal (which is the default)
		Default::default()
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// Store value does not loop at all so a fixed weight is appropriate
		#[weight = SimpleDispatchInfo::FixedNormal(100)]
		fn store_value(origin, entry: u32) -> Result {

			StoredValue::put(entry);

			Ok(())
		}

		/// This one sets the storage value n times, so it should cost n times as much.
		/// Because it performs both a read and a write, the multiplier is set to 200
		/// instead of 100 as before
		#[weight = Linear(200)]
		fn add_n(origin, n: u32) -> Result {

			let mut old : u32;
			for _i in 1..=n {
				old = StoredValue::get();
				StoredValue::put(old + 1);
			}
			Ok(())
		}
		/// This one is proportional to a storage value
		/// Dispatch weightings can't use storage values directly,
		/// So we have the caller pass in the expected storage value
		/// and we ensure it is correct
		#[weight = Linear(200)]
		fn double(origin, initial_value: u32) -> Result {
			let initial = StoredValue::get();
			ensure!(initial == initial_value, "Storage value did not match parameter");

			for _i in 1..=initial {
				let old = StoredValue::get();
				StoredValue::put(old + 1);
			}
			Ok(())
		}

		// This one is polynomial in the second argument so we use a wrapper
		//TODO
	}
}
