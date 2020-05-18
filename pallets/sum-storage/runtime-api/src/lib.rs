#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait SumStorageApi {
		fn get_sum() -> u32;
	}
}
