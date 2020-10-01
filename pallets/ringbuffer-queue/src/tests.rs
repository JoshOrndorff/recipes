use super::{RawEvent, ValueStruct};
use crate::{Module, Trait};
use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for TestRuntime {
	type BaseCallFilter = ();
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
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

mod ringbuffer {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		ringbuffer<T>,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type RingBuffer = Module<TestRuntime>;

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn add_to_queue_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(RingBuffer::add_to_queue(Origin::signed(1), 1, true));
		assert_eq!(
			RingBuffer::get_value(0),
			ValueStruct {
				integer: 1,
				boolean: true
			}
		);
		assert_eq!(RingBuffer::range(), (0, 1));
	})
}

#[test]
fn add_multiple_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(RingBuffer::add_multiple(
			Origin::signed(1),
			vec![1, 2, 3],
			true
		));
		assert_eq!(
			RingBuffer::get_value(0),
			ValueStruct {
				integer: 1,
				boolean: true
			}
		);
		assert_eq!(RingBuffer::range(), (0, 3));
	})
}

#[test]
fn pop_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(RingBuffer::add_to_queue(Origin::signed(1), 1, true));
		assert_eq!(
			RingBuffer::get_value(0),
			ValueStruct {
				integer: 1,
				boolean: true
			}
		);
		assert_eq!(RingBuffer::range(), (0, 1));

		assert_ok!(RingBuffer::pop_from_queue(Origin::signed(1)));
		assert_eq!(RingBuffer::range(), (1, 1));

		let expected_event = TestEvent::ringbuffer(RawEvent::Popped(1, true));

		assert_eq!(
			System::events()[0].event,
			expected_event,
		);
	})
}
