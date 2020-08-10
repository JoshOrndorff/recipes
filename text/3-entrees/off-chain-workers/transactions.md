# Transactions in Off-chain Workers

_[`pallets/offchain-demo`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo)_

## Compiling this Pallet

This `offchain-demo` pallet is included in the
[ocw-runtime](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime).
That runtime can be used in the kitchen node.

In order to use the Offchain worker, the node must inject some keys into its keystore, and that is
enabled with a feature flag.

First, edit `nodes/kitchen-node/Cargo.toml` to enable the `ocw-runtime`.

Then, build the kitchen node with these commands.

```bash
# Switch to kitchen-node directory
cd nodes/kitchen-node

# Compile with OCW feature
cargo build --release --features ocw
```

## Life-cycle of Off-chain Worker

Running the `kitchen-node`, you will see the off-chain worker is run after each block generation
phase, as shown by `Entering off-chain workers` in the node output message:

```
...
2020-03-14 13:30:36 Starting BABE Authorship worker
2020-03-14 13:30:36 Prometheus server started at 127.0.0.1:9615
2020-03-14 13:30:41 Idle (0 peers), best: #0 (0x2658…9a5b), finalized #0 (0x2658…9a5b), ⬇ 0 ⬆ 0
2020-03-14 13:30:42 Starting consensus session on top of parent 0x26582455e63448e8dafe1e70f04d7d74d39358c6b71c306eb7013e2c54069a5b
2020-03-14 13:30:42 Prepared block for proposing at 1 [hash: 0xdc7a76fc89c45a3f318e29df06cbdb097cc3094112b204f10e1e84e0799eba88; parent_hash: 0x2658…9a5b; extrinsics (1): [0xf572…63c0]]
2020-03-14 13:30:42 Pre-sealed block for proposal at 1. Hash now 0x3558accae1325a2ae5569512b8542e90ae11b4f0de6834ba901eb03b97a680aa, previously 0xdc7a76fc89c45a3f318e29df06cbdb097cc3094112b204f10e1e84e0799eba88.
2020-03-14 13:30:42 New epoch 0 launching at block 0x3558…80aa (block slot 264027307 >= start slot 264027307).
2020-03-14 13:30:42 Next epoch starts at slot 264027407
2020-03-14 13:30:42 Imported #1 (0x3558…80aa)
2020-03-14 13:30:42 Entering off-chain workers
2020-03-14 13:30:42 off-chain send_signed: acc: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY| number: 0
...
```

Referring to the code at
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs),
there is an `offchain_worker` function inside `decl_module!`. This is the entry point of the
off-chain worker that is executed once after each block generation, so we put all the off-chain
logic here.

Two kinds of transactions can be sent back on-chain from off-chain workers, **Signed Transactions**
and **Unsigned Transactions**. Signed transactions are used if the transaction requires the sender
to be specified. Unsigned transactions are used when the sender does not need to be known, and
additional logic is written in the code to provide extra data verification. Let's walk through how
to set up each one.

## Signed Transactions

> **Notes**: This example will have account `Alice` submitting signed transactions to the node in
off-chain worker, and these transactions have associated fees. If you run the node in development
mode (with `--dev` flag) using the default sr25519 crypto signature, `Alice` will have initial fund
deposited and this example will run all fine.
>
> If you are running either not in development mode or switched to another crypto signature, please
> beware to have account `Alice` setup and be funded to run this example.

### Setup

For signed transactions, the first thing you will notice is that we have defined another sub-module
here:

src:
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, KEY_TYPE);
}
```

This is the application key to be used as the prefix for this pallet in underlying storage.

Second, we have added an additional associated type `AuthorityId`.

src:
[`pallets/offchain-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo/src/lib.rs)

```rust
pub trait Trait: system::Trait {
	//...snip
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
}
```

This associated type needs to be specified by the runtime when the runtime is to include this pallet
(implement this pallet trait).

Now if we build the `kitchen-node`, we will see the compiler complain that there are three trait
bounds `Runtime: frame_system::offchain::CreateSignedTransaction`,
`frame_system::offchain::SigningTypes`, and `frame_system::offchain::SendTransactionTypes` are not
satisfied. We learn that when using `SubmitSignedTransaction`, we also need to have our runtime
implement the
[`CreateSignedTransaction` trait](https://substrate.dev/rustdocs/v2.0.0-rc5/frame_system/offchain/trait.CreateSignedTransaction.html).
So let's implement this in our runtime.

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

There is a lot happening in the code. But basically we are:

-   Signing the `call` and `extra`, also called signed extension, and
-   Making the call(`call`, which includes the call paramters) and passing the sender `address`,
    signature of the data `signature`, and its signed extension `extra` on-chain as a transaction.

`SignedExtra` data type is defined later in the runtime.

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

You will notice that we run a for loop in the returned result. This implies that this call may make
multiple transactions and return multiple results. It is because this call actually signs and sends
the transaction with each of the accounts that can be found locally under the application crypto
(which we defined earlier in `pub mod crypto {...}`). You can view this as the local accounts that
are managed under this pallet namespace. Right now, we only have one key in the app crypto, so only
one signed transaction is made.

Eventually, the `call` transaction is made on-chain via the `create_transaction` function we defined
earlier when we implemented `CreateTransaction` trait in our runtime.

If you are wondering where we insert the local account in the pallet app crypto, it is actually in
the outer node's [service](https://substrate.dev/rustdocs/v2.0.0-rc5/sc_service/index.html).

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

By default, unsigned transactions are rejected by the Substrate runtime unless they are explicitly
allowed. So we need to write the logic to allow unsigned transactions for certain particular
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

We implement `ValidateUnsigned`, and the allowance logic is added inside the `validate_unsigned`
function. We check if the call is to `Call::submit_number_unsigned` and returns `Ok()` if this is
the case. Otherwise, `InvalidTransaction::Call`.

The `ValidTransaction` object has some fields that touch on concepts that we have not discussed
before:

-   `priority`: Ordering of two transactions, given their dependencies are satisfied.
-   `requires`: List of tags this transaction depends on. Not shown in the above code as it is not
    necessary in this case.
-   `provides`: List of tags provided by this transaction. Successfully importing the transaction
    will enable other transactions that depend on these tags to be included as well. `provides` and
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
this as a parameter when constructing the runtime.

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

As in signed transactions, we prepare a function reference with its parameters and then call
`T::SubmitUnsignedTransaction::submit_unsigned`.

## Testing

For writing test cases for off-chain worker, refer to our
[testing section](../testing/off-chain-workers.md).
