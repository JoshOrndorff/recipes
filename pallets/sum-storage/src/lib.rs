#![cfg_attr(not(feature = "std"), no_std)]

/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, dispatch};
use frame_system::{self as system, ensure_signed};

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Thing1 get(fn thing1): u32;
		Thing2 get(fn thing2): u32;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn set_thing_1(origin, val: u32) -> dispatch::DispatchResult {
			let _ = ensure_signed(origin)?;

			Thing1::put(val);

			Self::deposit_event(Event::ValueSet(1, val));
			Ok(())
		}

		pub fn set_thing_2(origin, val: u32) -> dispatch::DispatchResult {
			let _ = ensure_signed(origin)?;

			Thing2::put(val);

			Self::deposit_event(Event::ValueSet(2, val));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn get_sum() -> u32 {
		Thing1::get() + Thing2::get()
	}
}

decl_event!(
	pub enum Event {
		ValueSet(u32, u32),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn default_sum_zero() {
		new_test_ext().execute_with(|| {
			assert_eq!(TemplateModule::get_sum(), 0);
		});
	}

	#[test]
	fn sums_thing_one() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::set_thing_1(Origin::signed(1), 42));
			assert_eq!(TemplateModule::get_sum(), 42);
		});
	}

	#[test]
	fn sums_thing_two() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::set_thing_2(Origin::signed(1), 42));
			assert_eq!(TemplateModule::get_sum(), 42);
		});
	}

	#[test]
	fn sums_both_values() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::set_thing_1(Origin::signed(1), 42));
			assert_ok!(TemplateModule::set_thing_2(Origin::signed(1), 43));
			assert_eq!(TemplateModule::get_sum(), 85);
		});
	}
}
