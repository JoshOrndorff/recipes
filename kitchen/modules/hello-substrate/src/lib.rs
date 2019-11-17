/// A very simple substrate runtime
use support::{
	decl_module, decl_event, decl_storage, StorageValue,
	dispatch::Result, ensure
};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!{
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
	{
		ValueSet(AccountId, u64),
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as HelloSubstrate{
		pub LastValue get(fn last_value): u64;
		pub UserValue get(fn user_value): map T::AccountId => u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn set_value(origin, value: u64) -> Result {
			let setter = ensure_signed(origin)?;
			LastValue::put(value);
			UserValue::<T>::insert(&setter, value);
			Self::deposit_event(RawEvent::ValueSet(setter, value));
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
	use support::{impl_outer_origin, parameter_types, traits::Get};
	use runtime_primitives::{Perbill, traits::{IdentityLookup, BlakeTwo256}, testing::Header};
	use runtime_io;
	use core::cell::RefCell;
	use primitives::H256;
	use crate::{Module, Trait};

	impl_outer_origin!{
		pub enum Origin for Runtime {}
	}

	// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
	#[derive(Clone, PartialEq, Eq, Debug)]
	pub struct Runtime;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: u32 = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}
	impl system::Trait for Runtime {
		type Origin = Origin;
		type Index = u64;
		type Call = ();
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
	}

	impl Trait for Runtime {
		type Event = ();
	}

	pub type System = system::Module<Runtime>;
	pub type HelloSubstrate = Module<Runtime>;

	pub struct ExtBuilder;

	impl ExtBuilder {
		pub fn build() -> runtime_io::TestExternalities {
			let mut storage = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
			runtime_io::TestExternalities::from(storage)
		}
	}

	#[test]
	fn last_value_updates() {
		ExtBuilder::build().execute_with(|| {
			HelloSubstrate::set_value(Origin::signed(2), 10u64);
			assert_eq!(HelloSubstrate::last_value(), 10u64);
			HelloSubstrate::set_value(Origin::signed(2), 11u64);
			assert_eq!(HelloSubstrate::last_value(), 11u64);
		});
	}

	#[test]
	fn user_value_works() {
		ExtBuilder::build().execute_with(|| {
			HelloSubstrate::set_value(Origin::signed(1), 10u64);
			assert_eq!(HelloSubstrate::last_value(), 10u64);
			HelloSubstrate::set_value(Origin::signed(2), 11u64);
			assert_eq!(HelloSubstrate::user_value(&2), 11u64);
			assert_eq!(HelloSubstrate::user_value(&1), 10u64);
			// verify again that last_value worked as well
			assert_eq!(HelloSubstrate::last_value(), 11u64);
		});
	}
}