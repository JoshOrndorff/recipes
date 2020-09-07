# Transactions in Off-chain Workers

`pallets/ocw-demo`
<a href="https://playground-staging.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%ocw-demo%2Fsrc%2Flib.rs" target="_blank">![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)</a>
<a href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs" target="_blank">![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)</a>

## Compiling this Pallet

This `ocw-demo` pallet is included in the
[ocw-runtime](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime).
In order to use this runtime in the kitchen node, we open the `nodes/kitchen-node/Cargo.toml` file,
enable the `ocw-runtime` package and comment out the `super-runtime` package.

Then we build the kitchen node with `ocw` feature flag:

```bash
# Switch to kitchen-node directory
cd nodes/kitchen-node

# Compile with OCW feature
cargo build --release --features ocw
```

With this feature flag, an account key is injected into the Substrate node keystore.

src:
[`nodes/kitchen-node/src/service.rs`](https://github.com/substrate-developer-hub/recipes/blob/master/nodes/kitchen-node/src/service.rs)

```rust
// Initialize seed for signing transaction using off-chain workers
#[cfg(feature = "ocw")]
{
	keystore.write().insert_ephemeral_from_seed_by_type::<runtime::offchain_demo::crypto::Pair>(
		"//Alice", runtime::offchain_demo::KEY_TYPE
	).expect("Creating key with account Alice should succeed.");
}
```

## Life-cycle of Off-chain Worker

Running the `kitchen-node` you will see log messages similar to the following:

```
2020-09-02 11:09:33.780 main WARN sc_cli::commands::run_cmd  Running in --dev mode, RPC CORS has been disabled.
2020-09-02 11:09:33.780 main INFO sc_cli::runner  Kitchen Node
2020-09-02 11:09:33.781 main INFO sc_cli::runner  ✌️  version 2.0.0-rc6-unknown-x86_64-linux-gnu
2020-09-02 11:09:33.781 main INFO sc_cli::runner  ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2019-2020
2020-09-02 11:09:33.781 main INFO sc_cli::runner  📋 Chain specification: Development
2020-09-02 11:09:33.781 main INFO sc_cli::runner  🏷  Node name: precious-angle-3060
2020-09-02 11:09:33.781 main INFO sc_cli::runner  👤 Role: AUTHORITY
2020-09-02 11:09:33.781 main INFO sc_cli::runner  💾 Database: RocksDb at /home/jimmychu/.local/share/kitchen-node/chains/dev/db
2020-09-02 11:09:33.781 main INFO sc_cli::runner  ⛓  Native runtime: ocw-runtime-1 (ocw-runtime-1.tx1.au1)
2020-09-02 11:09:34.881 main INFO sc_service::client::client  🔨 Initializing Genesis block/state (state: 0x2b24…4bf9, header-hash: 0xde55…8fed)
2020-09-02 11:09:35.081 main WARN sc_service::builder  Using default protocol ID "sup" because none is configured in the chain specs
2020-09-02 11:09:35.083 main INFO sub-libp2p  🏷  Local node identity is: 12D3KooWC8iNnJqM64qiurVSA3mRFGE4LPj99QPVtUE6whyxFAJy (legacy representation: QmZPmiuc4DAmM7Fo6GdChmxF4pTaDc8brgUKVXLhxKjq62)
2020-09-02 11:09:35.517 main INFO sc_service::builder  📦 Highest known block at #0
2020-09-02 11:09:35.519 tokio-runtime-worker INFO substrate_prometheus_endpoint::known_os  〽️ Prometheus server started at 127.0.0.1:9615
2020-09-02 11:09:40.527 tokio-runtime-worker INFO substrate  💤 Idle (0 peers), best: #0 (0xde55…8fed), finalized #0 (0xde55…8fed), ⬇ 0 ⬆ 0
...
```

First, pay attention the line `⛓  Native runtime: ocw-runtime-1 (ocw-runtime-1.tx1.au1)`
to ensure we are running the kitchen-node with the `ocw-runtime`.

Other than that, you will realized the chain is just sitting idled. This is because currently off-chain
worker is only run after a block is imported. Our kitchen node is configured to use
[instant-seal consensus](../kitchen-node.md), meaning that we need to send a transaction to trigger a
block to be imported.

Once a transaction is sent, such as using [Polkadot-JS App](https://polkadot.js.org/apps?rpc=ws://localhost:9944)
to perform a balance transfer, the following more interesting logs are shown.

```
2020-09-03 23:47:24.656 tokio-runtime-worker INFO sc_consensus_manual_seal::rpc  Instant Seal success: CreatedBlock { hash: 0x02f2fd8e06bd8138040813f18c4b2df41404c289c3418142f613ae5c72abe6ce, aux: ImportedAux { header_only: false, clear_justification_requests: false, needs_justification: false, bad_justification: false, needs_finality_proof: false, is_new_best: true } }
2020-09-03 23:47:24.658  INFO ocw_demo  Entering off-chain worker
2020-09-03 23:47:24.866 tokio-runtime-worker INFO sc_basic_authorship::basic_authorship  🙌 Starting consensus session on top of parent 0x02f2fd8e06bd8138040813f18c4b2df41404c289c3418142f613ae5c72abe6ce
2020-09-03 23:47:24.874 tokio-blocking-driver INFO ocw_demo  submit_number_signed: (0, d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...))
2020-09-03 23:47:24.874 tokio-blocking-driver INFO ocw_demo  Number vector: [0]
...
```

Let's take a deeper look at what's happening here. Referring to the code at
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs),
there is an `fn offchain_worker()` function inside `decl_module!`. This is the entry point of the
off-chain worker logic which is executed once per block import.

As off-chain workers, by definition, run computation off-chain, they cannot alter the block state
directly. In order to do so, they need to send transactions back on-chain. Three kinds of transaction
can be sent here, **signed transactions**, **unsigned transactions**, and **unsigned transactions
with signed payload**.

- [Signed transactions](#signed-transactions) are used if the transaction requires the sender to be
specified.
- [Unsigned transactions](#unsigned-transactions) are used when the sender does not need to be known.
- [Unsigned transactions with signed payloads](#unsigned-transactions-with-signed-payloads) are used
if the transaction requires the sender to be specified but the sender account not be charged for the transaction fee.

We will walk through each of them in the following.

## Signed Transactions

> **Notes**: This example will have account `Alice` submitting signed transactions to the node in
> the off-chain worker, and these transactions have associated fees. If you run the node in development
> mode (with `--dev` flag) using the default sr25519 crypto signature, `Alice` will have sufficient funds
> initialized in the chain and this example will run fine. Otherwise, please be aware `Alice` account
> must be funded to run this example.

### Setup

For signed transactions, we have to define a crypto signature sub-module:

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	// -- snip --
	app_crypto!(sr25519, KEY_TYPE);
}
```

`KEY_TYPE` is the application key prefix for the pallet in the underlying storage. This is to be used
for signing transactions.

Second, we have our pallet configration trait be additionally bounded by `CreateSignedTransaction`
and add an additional associated type `AuthorityId`. This tell the runtime that this pallet can
create signed transactions.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	// -- snip --
}
```

Now if we [build the `kitchen-node`](#compiling-this-pallet), we will see compiler errors saying
three trait bounds are not satisfied: `Runtime: frame_system::offchain::CreateSignedTransaction`,
`frame_system::offchain::SigningTypes`, and `frame_system::offchain::SendTransactionTypes`. So let's
implement these traits in our runtime.

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		index: Index,
	) -> Option<(
		Call,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		let period = BlockHashCount::get() as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			frame_system::CheckNonce::<Runtime>::from(index),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);

		#[cfg_attr(not(feature = "std"), allow(unused_variables))]
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				debug::native::warn!("SignedPayload error: {:?}", e);
			})
			.ok()?;

		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;

		let address = account;
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}
```

The overall goal here is to perform the following:

- Signing the on-chain `call` and `extra` payload of the call. This together is called the signature.
- Finally returning the on-chain `call`, the account/address making the signature, the signature
itself, and the `extra` payload.

The `SignedExtra` data type used above is defined later in our runtime.

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
```

Next the remaining two traits are also implemented by specifying the concrete types of their respective
trait associated types.

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = UncheckedExtrinsic;
}
```

By now, we have completed the setup of implementing the necessary trait bounds for our runtime to
create signed transactions.

### Sending Signed Transactions

A signed transaction is sent with `frame_system::offchain::SendSignedTransaction::send_signed_transaction`,
as shown below:

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	// We retrieve a signer and check if it is valid.
	//   Since this pallet only has one key in the keystore. We use `any_account()1 to
	//   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
	//   ref: https://substrate.dev/rustdocs/v2.0.0-rc6/frame_system/offchain/struct.Signer.html
	let signer = Signer::<T, T::AuthorityId>::any_account();

	// Translating the current block number to number and submit it on-chain
	let number: u64 = block_number.try_into().unwrap_or(0) as u64;

	// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
	//   - `None`: no account is available for sending transaction
	//   - `Some((account, Err(())))`: error occured when sending the transaction
	//   - `Some((account, Ok(())))`: transaction is successfully sent
	let result = signer.send_signed_transaction(|_acct|
		// This is the on-chain function
		Call::submit_number_signed(number)
	);

	// Display error if the signed tx fails.
	if let Some((acc, res)) = result {
		if res.is_err() {
			debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
			return Err(<Error<T>>::OffchainSignedTxError);
		}
		// Transaction is sent successfully
		return Ok(());
	}

	// The case of `None`: no account is available for sending
	debug::error!("No local account available");
	Err(<Error<T>>::NoLocalAcctForSignedTx)
}
```

On the above code, we first retrieve a signer. Then we send a signed transaction on-chain by calling
`send_signed_transaction` with a closure returning the on-chain call,
`Call::submit_number_signed(submission)`.

Then we use the signer to send signed transaction, and the result is in the type of
`Option<(Account<T>, Result<(), ()>)>`. So we handle each of the following cases:

- `None`: when no account is available for sending transaction
- `Some((account, Err(())))`: when an error occured when sending the transaction
- `Some((account, Ok(())))`: when transaction is successfully sent

Eventually, the `call` transaction is made on-chain via the
`frame_system::offchain::CreateSignedTransaction::create_transaction` function we defined in our
runtime.

## Unsigned Transactions

### Setup

By default unsigned transactions are rejected by the runtime unless they are explicitly
allowed. So we write the logic to allow unsigned transactions to be validated:

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		let valid_tx = |provide| ValidTransaction::with_tag_prefix("ocw-demo")
			.priority(T::UnsignedPriority::get())
			.and_provides([&provide])
			.longevity(3)
			.propagate(true)
			.build();

		match call {
			Call::submit_number_unsigned(_number) => valid_tx(b"submit_number_unsigned".to_vec()),
			// -- snip --
			_ => InvalidTransaction::Call.into(),
		}
	}
}
```

We implement the `ValidateUnsigned` trait for `Module`, and add the allowance logic inside the
`validate_unsigned` function. We verify that if the call is `Call::submit_number_unsigned` we return
a [`ValidTransaction`](https://substrate.dev/rustdocs/v2.0.0-rc6/sp_runtime/transaction_validity/struct.ValidTransaction.html) object using the [builder pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md).

The `ValidTransaction` object contain some fields we have not seen before:

- `priority`: determine the ordering of two transactions, given their dependencies are satisfied.
- `requires`: contain a list of tags the transaction depends on.
- `provides`: contain a list of tags provided by this transaction. Successfully importing the
	transaction will enable other transactions that depend on these tags be included. Both`provides`
  and `requires` tags allow Substrate to build a dependency graph of transactions and import them in
  the right order.
- `longevity`: this transaction longevity describes the minimum number of blocks the transaction
  has to be valid for. After this period the transaction should be removed from the pool or revalidated.
- `propagate`: indicate if the transaction should be propagated to other peers. By setting to
  `false` the transaction will still be considered for inclusion in blocks on
  the current node but will never be sent to other peers.

Finally, to tell the runtime that we have our own `ValidateUnsigned` logic, we need to pass
this as a parameter when constructing the runtime:

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		//...snip
		OcwDemo: ocw_demo::{Module, Call, Storage, Event<T>, ValidateUnsigned},
	}
);
```

### Sending Unsigned Transactions

We can now send an unsigned transaction from offchain worker with the
`T::SubmitUnsignedTransaction::submit_unsigned` function, as shown in the code.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	let number: u64 = block_number.try_into().unwrap_or(0) as u64;
	let call = Call::submit_number_unsigned(number);

	// `submit_unsigned_transaction` returns a type of `Result<(), ()>`
	//   ref: https://substrate.dev/rustdocs/v2.0.0-rc6/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction
	SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
		.map_err(|_| {
			debug::error!("Failed in offchain_unsigned_tx");
			<Error<T>>::OffchainUnsignedTxError
		})
}
```

As in signed transactions, we prepare a function reference with its parameters and call
[`frame_system::offchain::SubmitTransaction::submit_unsigned_transaction`](https://substrate.dev/rustdocs/v2.0.0-rc6/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction).

## Unsigned Transactions with Signed Payloads

With this type of transaction, we need to first specify a signer, sign the transaction, and then send
it back on-chain as unsigned transaction. The main difference with signed transactions is that the signer
account will not be charged for the transaction fee. This is not the case for signed transaction normally.

But this could potentially be an attack vector, so extra precaution should be added as to what counted
as a valid (unsigned) transaction.

Since we are still sending unsigned transactions, we need to add extra code to validate them. [See above](#setup-1).

### Sending Unsigned Transactions with Signed Payloads

We send unsigned transactions with signed payloads as followed.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_unsigned_tx_signed_payload(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	// Retrieve the signer to sign the payload
	let signer = Signer::<T, T::AuthorityId>::any_account();

	let number: u64 = block_number.try_into().unwrap_or(0) as u64;

	// `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
	//   Similar to `send_signed_transaction`, they account for:
	//   - `None`: no account is available for sending transaction
	//   - `Some((account, Ok(())))`: transaction is successfully sent
	//   - `Some((account, Err(())))`: error occured when sending the transaction
	if let Some((_, res)) = signer.send_unsigned_transaction(
		|acct| Payload { number, public: acct.public.clone() },
		Call::submit_number_unsigned_with_signed_payload
	) {
		return res.map_err(|_| {
			debug::error!("Failed in offchain_unsigned_tx_signed_payload");
			<Error<T>>::OffchainUnsignedTxSignedPayloadError
		});
	}

	// The case of `None`: no account is available for sending
	debug::error!("No local account available");
	Err(<Error<T>>::NoLocalAcctForSigning)
}
```

What is unique here is that
[`send_unsigned_transaction` function](https://substrate.dev/rustdocs/v2.0.0-rc6/frame_system/offchain/trait.SendUnsignedTransaction.html#tymethod.send_unsigned_transaction) takes two functions. The first, expressed as a closure,
returns a `SignedPayload` object, and the second returns an on-chain call to be made.

We have defined our `SignedPayload` object earlier in the pallet.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Payload<Public> {
	number: u64,
	public: Public
}

impl <T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
	fn public(&self) -> T::Public {
		self.public.clone()
	}
}
```

## Conclusion

By now, you should be able to code your own off-chain workers that send signed transactions, unsigned
transactions, and unsigned transactions with signed payloads back on chain.
