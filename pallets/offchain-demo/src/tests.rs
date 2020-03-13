use crate::*;

use codec::Decode;
use support::{
	assert_ok, impl_outer_event, impl_outer_origin, parameter_types,
	weights::{GetDispatchInfo, Weight},
};
use sp_io::TestExternalities;
use sp_core::{H256, sr25519};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

use crate as offchain_demo;

impl_outer_origin! {
	pub enum Origin for TestRuntime where system = system {}
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		system<T>,
		offchain_demo<T>,
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1_000_000;
	pub const MaximumBlockLength: u32 = 10 * 1_000_000;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

// The TestRuntime implements two pallet/frame traits: system, and simple_event
impl system::Trait for TestRuntime {
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

// --- mocking offchain-demo trait

type TestExtrinsic = TestXt<Call<TestRuntime>, ()>;
type SubmitTransaction = system::offchain::TransactionSubmitter<
	crypto::Public,
	TestRuntime,
	TestExtrinsic
>;

parameter_types! {
	pub const GracePeriod: u64 = 2;
}

impl Trait for TestRuntime {
	type Call = Call<TestRuntime>;
	type Event = TestEvent;
	type GracePeriod = GracePeriod;
	type SubmitSignedTransaction = SubmitTransaction;
	type SubmitUnsignedTransaction = SubmitTransaction;
}

impl system::offchain::CreateTransaction<TestRuntime, TestExtrinsic> for TestRuntime {
	type Public = sr25519::Public;
	type Signature = sr25519::Signature;

	fn create_transaction<TSigner: system::offchain::Signer<Self::Public, Self::Signature>> (
		call: Call<TestRuntime>,
		public: Self::Public,
		_account: <TestRuntime as system::Trait>::AccountId,
		index: <TestRuntime as system::Trait>::Index,
	) -> Option<(Call<TestRuntime>, <TestExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		Some((call, (index, ())))
	}
}

pub type System = system::Module<TestRuntime>;
pub type OffchainDemo = Module<TestRuntime>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		TestExternalities::from(storage)
	}
}

#[test]
fn submit_number_signed_works() {
	ExtBuilder::build().execute_with(|| {
		// call submit_number_signed
		let num = 32;
		let acct: <TestRuntime as system::Trait>::AccountId = Default::default();
		assert_ok!(OffchainDemo::submit_number_signed(Origin::signed(acct), num));
		// A number is inserted to <Numbers> vec
		assert_eq!(<Numbers>::get(), vec![num]);
		// storage <NextTx> is incremented
		assert_eq!(<NextTx<TestRuntime>>::get(), <TestRuntime as Trait>::GracePeriod::get());
		// AddSeq is incremented
		assert_eq!(<AddSeq>::get(), 1);
		// An event is emitted
		assert!(System::events().iter().any(|er| er.event ==
			TestEvent::offchain_demo(RawEvent::NewNumber(Some(acct), num))));
	})
}

#[test]
fn offchain_send_signed_tx() {

}

#[test]
fn offchain_send_unsigned_tx() {

}
