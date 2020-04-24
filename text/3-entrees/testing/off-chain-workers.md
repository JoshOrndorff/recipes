# Off-chain Worker Test Environment

Learn more about how to set up and use offchain-workers in the [offchain-demo entree](/3-entrees/off-chain-workers/index.md).

## Mock Runtime Setup

In addition to everything we need to set up in [Basic Test Environment](./mock.md), we also need to set up the mock for `SubmitTransaction`, and implement the `CreateTransaction` trait for the runtime.

src: [`pallets/offchain-demo/src/tests.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/tests.rs)

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

## Getting the Transaction Pool and Off-chain State

When writing test cases for off-chain workers, we need to look into the transaction pool and current off-chain state to ensure a certain transaction has made its way, and was passed with the right parameters and signature. So in addition to the regular test environment `TestExternalities`, we also need to return references to the transaction pool state and off-chain state for future inspection.

src: [`pallets/offchain-demo/src/tests.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/tests.rs)

```rust
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> (TestExternalities, Arc<RwLock<PoolState>>, Arc<RwLock<OffchainState>>) {
		const PHRASE: &str = "expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		// Getting the transaction pool and off-chain state. Return them for future inspection.
		let (offchain, offchain_state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();

		// Initialize the keystore with a default key
		let keystore = KeyStore::new();
		keystore.write().sr25519_generate_new(
			KEY_TYPE,
			Some(&format!("{}/hunter1", PHRASE))
		).unwrap();

		// Initialize our genesis config
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();

		// Get the TestExternalities, register additional extension we just set up
		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(keystore));

		// Return the externalities and two necessary states
		(t, pool_state, offchain_state)
	}
}
```

## Testing Off-chain Worker

When we write tests for off-chain workers, we should test only what our off-chain workers do. For example, when our off-chain workers will eventually make a signed transaction to dispatch function A, which does B, C, and D, we write our test for the off-chain worker to test only if function A is dispatched. But whether function A actually does B, C, and D should be tested separately in another test case. This way we keep our tests more robust.

This is how we write our test cases.

src: [`pallets/offchain-demo/src/tests.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/tests.rs)

```rust
#[test]
fn offchain_send_signed_tx() {
	let (mut t, pool_state, offchain_state) = ExtBuilder::build();

	t.execute_with(|| {
		// when
		let num = 32;
		OffchainDemo::send_signed(num).unwrap();
		// then

		// Test only one transaction is in the pool.
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());

		let tx = TestExtrinsic::decode(&mut &*tx).unwrap();
		// Test the transaction is signed
		assert_eq!(tx.signature.unwrap().0, 0);
		// Test the transaction is calling the expected extrinsics with expected parameters
		assert_eq!(tx.call, Call::submit_number_signed(num));
	});
}
```

We test that when `OffchainDemo::send_signed(num)` function is being called,

- There is only one transaction made to the transaction pool.
- The transaction is signed.
- The transaction is calling the `Call::submit_number_signed` on-chain function with the parameter `num`.

What's performed by the `Call::submit_number_signed` on-chain function is tested in another test case, which would be similar to how you [test for dispatched extrinsic calls](./common.md).
