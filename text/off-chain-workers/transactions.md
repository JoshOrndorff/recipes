# Transactions in Off-chain Workers

`pallets/offchain-demo`
<a href="https://playground-staging.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Foffchain-demo%2Fsrc%2Flib.rs" target="_blank">![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)</a>
<a href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs" target="_blank">![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)</a>

## Compiling this Pallet

This `offchain-demo` pallet is included in the
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

With this feature flag, an account key is also injected into the Substrate node keystore.

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

Running the `kitchen-node` you will see log messages similar to the following and realize nothing
much is special:

```
2020-09-01 23:43:38 Running in --dev mode, RPC CORS has been disabled.
2020-09-01 23:43:38 Kitchen Node
2020-09-01 23:43:38 ✌️  version 2.0.0-rc5-unknown-x86_64-linux-gnu
2020-09-01 23:43:38 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2019-2020
2020-09-01 23:43:38 📋 Chain specification: Development
2020-09-01 23:43:38 🏷  Node name: pastoral-competition-7996
2020-09-01 23:43:38 👤 Role: AUTHORITY
2020-09-01 23:43:38 💾 Database: RocksDb at /home/jimmychu/.local/share/kitchen-node/chains/dev/db
2020-09-01 23:43:38 ⛓  Native runtime: ocw-runtime-1 (ocw-runtime-1.tx1.au1)
2020-09-01 23:43:39 🔨 Initializing Genesis block/state (state: 0x67aa…4a19, header-hash: 0xc5f0…45a2)
2020-09-01 23:43:40 📦 Highest known block at #0
2020-09-01 23:43:40 Using default protocol ID "sup" because none is configured in the chain specs
2020-09-01 23:43:40 🏷  Local node identity is: 12D3KooWC8iNnJqM64qiurVSA3mRFGE4LPj99QPVtUE6whyxFAJy (legacy representation: QmZPmiuc4DAmM7Fo6GdChmxF4pTaDc8brgUKVXLhxKjq62)
2020-09-01 23:43:40 〽️ Prometheus server started at 127.0.0.1:9615
2020-09-01 23:43:45 💤 Idle (0 peers), best: #0 (0xc5f0…45a2), finalized #0 (0xc5f0…45a2), ⬇ 0 ⬆ 0
2020-09-01 23:43:50 💤 Idle (0 peers), best: #0 (0xc5f0…45a2), finalized #0 (0xc5f0…45a2), ⬇ 0 ⬆ 0
...
```

This is because currently off-chain worker is run after a block is imported. Our kitchen node is
configured to use [instant-seal consensus](/kitchen-node.md), so we need to send a transaction to
trigger a block to be imported.

Once a transaction is sent, such as using [Polkadot-JS App](https://polkadot.js.org/apps) to
perform a balance transfer, the following more interesting logs are shown.

```
2020-09-01 23:55:31 Instant Seal success: CreatedBlock { hash: 0xbbc4f7c4c2a8012857a4cda27747369ff6b5c19892d12f508051bb9af8cf3791, aux: ImportedAux { header_only: false, clear_justification_requests: false, needs_justification: false, bad_justification: false, needs_finality_proof: false, is_new_best: true } }
2020-09-01 23:55:31 ✨ Imported #1 (0xbbc4…3791)
2020-09-01 23:55:31 Entering off-chain workers
2020-09-01 23:55:31 🙌 Starting consensus session on top of parent 0xbbc4f7c4c2a8012857a4cda27747369ff6b5c19892d12f508051bb9af8cf3791
2020-09-01 23:55:31 off-chain send_signed: acc: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)| number: 0
2020-09-01 23:55:31 submit_number_signed: 0
2020-09-01 23:55:31 Current average of numbers is: 0
...
```

Let's take a deeper look at what's happening here. Referring to the code at
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs),
there is an `fn offchain_worker()` function inside `decl_module!`. This is the entry point of the
off-chain worker logic which is executed once per block import.

As off-chain workers, by definition, run computation off-chain, they cannot alter the block state. In
order to do so, they need to send transactions back on-chain. Three kinds of transaction can be sent
here, **signed transactions**, **unsigned transactions**, and **unsigned transactions with signed payload**.

- Signed transactions are used if the transaction requires the sender to be specified.
- Unsigned transactions are used when the sender does not need to be known.
- Unsigned transactions with signed payload are used, [TK]

We will walk through each of them in the following.

## Signed Transactions

> **Notes**: This example will have account `Alice` submitting signed transactions to the node in
> the off-chain worker, and these transactions have associated fees. If you run the node in development
> mode (with `--dev` flag) using the default sr25519 crypto signature, `Alice` will have sufficient funds
> initialized in the chain and this example will run fine. Otherwise, please be aware `Alice` account
> must be funded to run this example.

### Setup

For signed transactions, we have to define a crypto signature sub-module:

[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	// -- snip --
	app_crypto!(sr25519, KEY_TYPE);
}
```

`KEY_TYPE` is the application key prefix for the pallet in the underlying storage. This is to be used for signing transactions.

Second, we have our pallet configration trait be additionally bounded by `CreateSignedTransaction` and add an additional associated type `AuthorityId`. This tell the runtime that this pallet can create signed transactions.

src:
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	// -- snip --
}
```

Now if we [build the `kitchen-node`](#compiling-this-pallet), we will see the compiler return with three trait
bounds that are not satisfied: `Runtime: frame_system::offchain::CreateSignedTransaction`,
`frame_system::offchain::SigningTypes`, and `frame_system::offchain::SendTransactionTypes`.
We also learn that when using `SubmitSignedTransaction`, our runtime need to implement
[`CreateSignedTransaction` trait](https://substrate.dev/rustdocs/v2.0.0-rc6/frame_system/offchain/trait.CreateSignedTransaction.html).

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

// ...snip
```

The overall goal is to execute the following:

-   Sign the `call` and `extra`, also called 'signed extension'
-   Making the call (`call`, which includes the call paramters) and passing the sender `address`,
    signature of the data `signature`, and ensuring its signed extension `extra` on-chain as a transaction.

`SignedExtra` data type will be defined later in the runtime.

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	system::CheckTxVersion<Runtime>,
	system::CheckGenesis<Runtime>,
	system::CheckEra<Runtime>,
	system::CheckNonce<Runtime>,
	system::CheckWeight<Runtime>,
	transaction_payment::ChargeTransactionPayment<Runtime>,
);
```

Next, the remaining two traits are also implemented.

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

### Sending Signed Transactions

A signed transaction is sent with `T::SubmitSignedTransaction::submit_signed`, as shown below:

src:
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
fn send_signed(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	use system::offchain::SubmitSignedTransaction;
	//..snip

	let submission: u64 = block_number.try_into().ok().unwrap() as u64;
	let call = Call::submit_number_signed(submission);

	// Using `SubmitSignedTransaction` associated type we create and submit a transaction
	//   representing the call, we've just created.
	let results = T::SubmitSignedTransaction::submit_signed(call);
	for (acc, res) in &results {
		match res {
			Ok(()) => { debug::native::info!("off-chain send_signed: acc: {}| number: {}", acc, submission); },
			Err(e) => {
				debug::native::error!("[{:?}] Failed to submit signed tx: {:?}", acc, e);
				return Err(<Error<T>>::SendSignedError);
			}
		};
	}
	Ok(())
}
```

We have a function reference to `Call::submit_number_signed(submission)`. This is the function we
are going to submit back to on-chain, and passing it to
`T::SubmitSignedTransaction::submit_signed(call)`.

Notice that we run a loop in the returned result. This implies that the call may make
multiple transactions and return multiple results. The call is both signing and sending
the transaction with each of the accounts that can be found locally under the application crypto
(which we defined earlier in `pub mod crypto {...}`). This can be seen as the local accounts that
are managed under this pallet namespace. As we only have one key in the app crypto, so only
one signed transaction is made.

Eventually, the `call` transaction will be made on-chain via the `create_transaction` function we defined
earlier when we implemented `CreateTransaction` trait in our runtime.

The local account used to sign the transaction is inserted in the pallet app crypto, and lives in
the outer node's [service](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_service/index.html).

src:
[`nodes/kitchen-node/src/service.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/kitchen-node/src/service.rs)

```rust
pub fn new_full(config: Configuration<GenesisConfig>)
	-> Result<impl AbstractService, ServiceError>
{
	// ...snip
	let dev_seed = config.dev_key_seed.clone();

	// ...snip
	// Initialize seed for signing transaction using off-chain workers
	if let Some(seed) = dev_seed {
		service
			.keystore()
			.write()
			.insert_ephemeral_from_seed_by_type::<runtime::offchain_demo::crypto::Pair>(
				&seed,
				runtime::offchain_demo::KEY_TYPE,
			)
			.expect("Dev Seed should always succeed.");
	}
	// ...snip
}
```

## Unsigned Transactions

### Setup

By default, unsigned transactions are rejected by the runtime unless they are explicitly
allowed. So we need to write logic to allow unsigned transactions for certain particular
dispatched functions as follows:

src:
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
impl<T: Trait> support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		if let Call::submit_number_unsigned(number) = call {
			debug::native::info!("off-chain send_unsigned: number: {}", number);

			ValidTransaction::with_tag_prefix("offchain-demo")
				.priority(T::UnsignedPriority::get())
				.and_provides([b"submit_number_unsigned"])
				.longevity(3)
				.propagate(true)
				.build()
		} else {
			InvalidTransaction::Call.into()
		}
	}
}
```

By implementing `ValidateUnsigned`, the allowance logic is added inside the `validate_unsigned`
function. We verify that if the call is `Call::submit_number_unsigned` we return `Ok()`, otherwise `InvalidTransaction::Call`.

Note that the`ValidTransaction` object has some fields that touch on concepts that we have not discussed
before:

-   `priority`: Ordering of two transactions, given their dependencies are satisfied.
-   `requires`: List of tags the transaction depends on.
-   `provides`: List of tags provided by this transaction. Successfully importing the transaction
    will enable other transactions that depend on these tags to be included as well.
-   Both`provides` and
    `requires` tags allow Substrate to build a dependency graph of transactions and import them in
    the right order.
-   `longevity`: Transaction longevity, which describes the minimum number of blocks the transaction
    is valid for. After this period the transaction should be removed from the pool or revalidated.
-   `propagate`: Indication if the transaction should be propagated to other peers. By setting to
    `false` the transaction will still be considered for inclusion in blocks that are authored on
    the current node, but will never be sent to other peers.

We are using the
[builder pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md) to
build up this object.

Finally, to tell the runtime that we have our own `ValidateUnsigned` logic, we also need to pass
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
		OffchainDemo: offchain_demo::{Module, Call, Storage, Event<T>, ValidateUnsigned},
	}
);
```

### Sending Unsigned Transactions

We can now make an unsigned transaction from offchain worker with the
`T::SubmitUnsignedTransaction::submit_unsigned` function, as shown in the code.

src:
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
fn send_unsigned(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	use system::offchain::SubmitUnsignedTransaction;

	let submission: u64 = block_number.try_into().ok().unwrap() as u64;
	// the `block_number` param should be unique within each block generation phase
	let call = Call::submit_number_unsigned(block_number, submission);

	T::SubmitUnsignedTransaction::submit_unsigned(call).map_err(|e| {
		debug::native::error!("Failed to submit unsigned tx: {:?}", e);
		<Error<T>>::SendUnsignedError
	})
}
```

As in signed transactions, we prepare a function reference with its parameters and call
`T::SubmitUnsignedTransaction::submit_unsigned`.
