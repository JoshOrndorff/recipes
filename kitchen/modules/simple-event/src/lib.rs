#![cfg_attr(not(feature = "std"), no_std)]

/// Simple Event (not generic over types)
use support::{decl_event, decl_module, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn do_something(origin, input: u32) -> Result {
            let _ = ensure_signed(origin)?;

            // could do something with the input here instead
            let new_number = input;

            // emit event
            Self::deposit_event(Event::EmitInput(new_number));
            Ok(())
        }
    }
}

// uses u32 and not types from Trait so does not require `<T>`
decl_event!(
    pub enum Event {
        EmitInput(u32),
    }
);


#[cfg(test)]
mod tests {
	use support::{impl_outer_origin, impl_outer_event, parameter_types, traits::Get};
	use runtime_primitives::{Perbill, traits::{IdentityLookup, BlakeTwo256}, testing::Header};
    use system::{EventRecord, Phase};
    use runtime_io;
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
		type Event = TestEvent;
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
    }

    mod simple_event {
        pub use crate::Event;
    }
    
    impl_outer_event! {
        pub enum TestEvent for Runtime {
            simple_event,
        }
    }

	impl Trait for Runtime {
		type Event = TestEvent;
    }

	pub type System = system::Module<Runtime>;
	pub type SimpleEvent = Module<Runtime>;

	pub struct ExtBuilder;

	impl ExtBuilder {
		pub fn build() -> runtime_io::TestExternalities {
			let mut storage = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
			runtime_io::TestExternalities::from(storage)
		}
    }

    #[test]
    fn test() {
        ExtBuilder::build().execute_with(|| {
            SimpleEvent::do_something(Origin::signed(1), 32);

            assert_eq!(
                System::events(),
                vec![
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: TestEvent::simple_event(crate::Event::EmitInput(32)),
                        topics: vec![],
                    }
                ]
            );
        })
    }

}