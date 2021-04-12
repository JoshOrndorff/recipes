use crate::{self as ringbuffer_queue, Config, RawEvent, ValueStruct};
use frame_support::{assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		RingBuffer: ringbuffer_queue::{Module, Call, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Index = u64;
	type Call = Call;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl Config for TestRuntime {
	type Event = Event;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = frame_system::GenesisConfig::default()
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

		let expected_event = Event::ringbuffer_queue(RawEvent::Popped(1, true));

		assert_eq!(System::events()[0].event, expected_event,);
	})
}
