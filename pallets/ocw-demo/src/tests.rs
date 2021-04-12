use crate::{self as ocw_demo, *};
use frame_support::{assert_ok, construct_runtime, parameter_types};
use frame_system::{limits, mocking};
use parity_scale_codec::alloc::sync::Arc;
use parking_lot::RwLock;
use sp_core::{
	offchain::{
		testing::{self, OffchainState, PoolState},
		OffchainExt, TransactionPoolExt,
	},
	sr25519::{self, Signature},
	H256,
};
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
};

type Extrinsic = TestXt<Call, ()>;
type UncheckedExtrinsic = mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = mocking::MockBlock<TestRuntime>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// For testing the module, we construct a mock runtime.
construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		OcwDemo: ocw_demo::{Module, Call, Storage, Event<T>, ValidateUnsigned},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: limits::BlockWeights = limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const UnsignedPriority: u64 = 100;
}

impl Config for TestRuntime {
	type AuthorityId = crypto::TestAuthId;
	type Call = Call;
	type Event = Event;
}

impl frame_system::offchain::SigningTypes for TestRuntime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for TestRuntime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for TestRuntime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> (
		TestExternalities,
		Arc<RwLock<PoolState>>,
		Arc<RwLock<OffchainState>>,
	) {
		const PHRASE: &str =
			"expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		let (offchain, offchain_state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();
		let keystore = KeyStore::new();
		keystore
			.sr25519_generate_new(KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
			.unwrap();

		let storage = frame_system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();

		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(Arc::new(keystore)));
		t.execute_with(|| System::set_block_number(1));
		(t, pool_state, offchain_state)
	}
}

#[test]
fn submit_number_signed_works() {
	let (mut t, _, _) = ExternalityBuilder::build();

	t.execute_with(|| {
		// call submit_number_signed
		let num = 32;
		let acct: <TestRuntime as frame_system::Config>::AccountId = Default::default();
		assert_ok!(OcwDemo::submit_number_signed(Origin::signed(acct), num));
		// A number is inserted to <Numbers> vec
		assert_eq!(<Numbers>::get(), vec![num]);
		// An event is emitted
		assert!(System::events()
			.iter()
			.any(|er| er.event == Event::ocw_demo(RawEvent::NewNumber(Some(acct), num))));

		// Insert another number
		let num2 = num * 2;
		assert_ok!(OcwDemo::submit_number_signed(Origin::signed(acct), num2));
		// A number is inserted to <Numbers> vec
		assert_eq!(<Numbers>::get(), vec![num, num2]);
	});
}

#[test]
fn test_offchain_signed_tx() {
	let (mut t, pool_state, _offchain_state) = ExternalityBuilder::build();

	t.execute_with(|| {
		// Setup
		let num = 32;
		OcwDemo::offchain_signed_tx(num).unwrap();

		// Verify
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(tx.signature.unwrap().0, 0);
		assert_eq!(
			tx.call,
			Call::OcwDemo(ocw_demo::Call::submit_number_signed(num))
		);
	});
}

#[test]
fn test_offchain_unsigned_tx() {
	let (mut t, pool_state, _offchain_state) = ExternalityBuilder::build();

	t.execute_with(|| {
		// when
		let num = 32;
		OcwDemo::offchain_unsigned_tx(num).unwrap();
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(tx.signature, None);
		assert_eq!(
			tx.call,
			Call::OcwDemo(ocw_demo::Call::submit_number_unsigned(num))
		);
	});
}
