# Mock Runtime Setup

In addition to everything we need to do to setup [Basic Test Environment](./mock.md), we also need to have extra setup details below.

Refer to our [offchain-demo test](TK), the additional associated type wt need to define is the `SubmitTransaction` associated type, and have our runtime implements the `CreateTransaction` trait.

So we have the following code:

src: [pallets/offchain-demo/src/tests.rs](TK)

```rust
type TestExtrinsic = TestXt<Call<TestRuntime>, ()>;
type SubmitTransaction = system::offchain::TransactionSubmitter<
	crypto::Public,
	TestRuntime,
	TestExtrinsic
>;

impl Trait for TestRuntime {
	// ...snip
	// For signed transaction
	type SubmitSignedTransaction = SubmitTransaction;
	// For unsigned transaction
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
		// This is the simplest setup we can do
		Some((call, (index, ())))
	}
}
```

# Testing on Off-chain Worker

## Getting the Transaction Pool and Off-chain State

When writing test cases, we usually need to look into the transaction pool and current off-chain state. This is how we build up and retrieve them, shown in our offchain-demo test.

src: [pallets/offchain-demo/src/tests.rs](TK)

```rust
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> (TestExternalities, Arc<RwLock<PoolState>>, Arc<RwLock<OffchainState>>) {
		const PHRASE: &str = "expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		// getting the transaction pool and off-chain state. Return them for future inspection.
		let (offchain, offchain_state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();

		// Initialize the keystore with a default key
		let keystore = KeyStore::new();
		keystore.write().sr25519_generate_new(
			KEY_TYPE,
			Some(&format!("{}/hunter1", PHRASE))
		).unwrap();

		// initialize our genesis config
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();

		// Get the TestExternalities, register additional extension we just setup
		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(keystore));

		// Return the externalities and two necessary states
		(t, pool_state, offchain_state)
	}
}
```

## Testing in Off-chain Worker

When we write tests on off-chain worker, we should test only what our off-chain workers do. For example, when our off-chain worker eventually will make a signed transaction to dispatched function A, which does B, C, and D. We write our test for off-chain worker to test if function A is eventually dispatched. But whether function A actually does B, C, and D, it should be tested separately in another test case.

This is what happen exactly in our test cases.

src: [pallets/offchain-demo/src/tests.rs](TK)

```rust
#[test]
fn offchain_send_signed_tx() {
	let (mut t, pool_state, offchain_state) = ExtBuilder::build();

	t.execute_with(|| {
		// when
		let num = 32;
		OffchainDemo::send_signed(num).unwrap();
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
		let tx = TestExtrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(tx.signature.unwrap().0, 0);
		assert_eq!(tx.call, Call::submit_number_signed(num));
	});
}
```

We test that when `OffchainDemo::send_signed(num)` function is being called,

- There is only one transaction is made into the transaction pool
- The transaction has a signature associated with it
- The transaction is calling `Call::submit_number_signed` on-chain function with the parameter `num`.

What's performed by the `Call::submit_number_signed` on-chain function is tested in another test case, which would similar to how you [test for on-chain function](./common.md).
