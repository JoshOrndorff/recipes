# Transaction Fees
*[runtimes/weight-fee-runtime](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/weight-fee-runtime)*

Substrate provides the [`transaction_payment` pallet](https://substrate.dev/rustdocs/master/pallet_transaction_payment/index.html) for calculating and collecting fees for executing transactions. Fees are broken down into several components:

* Base fee - A fixed fee applied to each transaction. A parameter in the `transaction_payment` pallet.
* Length fee - A fee proportional to the transaction's length in bytes. The proportionality constant is a parameter in the `transaction_payment` pallet.
* Weight fee - A fee calculated from the transaction's weight. Weights are intended to capture the actual resources consumed by the transaction. Learn more in the [recipe on weights](./weights.md). It doesn't need to be linear, although it often is. The same conversion function is applied across all transactions from all pallets in the runtime.
* Fee Multiplier - A multiplier for the computed fee, that can change as the chain progresses. This topic is not (yet) covered further in the recipes.

```
total_fee = base_fee + transaction_length * length_fee + weight_to_fee(total_weight)
```

## Setting the Constants

Each of the parameters described above is set in the `transaction_payment` pallet's configuration trait. For example, the `super-runtime` sets these parameters as follows.

src: [`runtimes/super-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/super-runtime/src/lib.rs)

```rust,ignore
parameter_types! {
	pub const TransactionBaseFee: u128 = 0;
	pub const TransactionByteFee: u128 = 1;
}

impl transaction_payment::Trait for Runtime {
	type Currency = balances::Module<Runtime>;
	type OnTransactionPayment = ();
	type TransactionBaseFee = TransactionBaseFee;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = ConvertInto;
	type FeeMultiplierUpdate = ();
}
```

## Converting Weight To Fees

In many cases converting weight to fees in a one-to-one fashion, as shown above, will suffice and can be accomplished with [`ConvertInto`](https://substrate.dev/rustdocs/master/sp_runtime/traits/struct.ConvertInto.html). This approach is also taken in the [node template](https://github.com/substrate-developer-hub/substrate-node-template/blob/43ee95347b6626580b1d9d554c3c8b77dc85bc01/runtime/src/lib.rs#L230). It is also possible to provide a type that makes a more complex calculation. Any type that implements `Convert<Weight, Balance>` will suffice.

This example uses a quadratic conversion and supports custom coefficients.

src: [`runtimes/weight-fee-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/weight-fee-runtime/src/lib.rs)

```rust, ignore
pub struct QuadraticWeightToFee<C0, C1, C2>(C0, C1, C2);

impl<C0, C1, C2> Convert<Weight, Balance> for QuadraticWeightToFee<C0, C1, C2>
	where C0: Get<Balance>, C1: Get<Balance>, C2: Get<Balance> {

	fn convert(w: Weight) -> Balance {
		let c0 = C0::get();
		let c1 = C1::get();
		let c2 = C2::get();
		let w = Balance::from(w);

		// TODO use safe math
		c0 + c1 * w + c2 * w * w
	}
}
```

## Collecting Fees

Having calculated the amount of fees due, runtime authors must decide which asset the fees should be paid in. A common choice is the use the [`Ballances` pallet](https://substrate.dev/rustdocs/master/pallet_balances/index.html), but any type that implements the [`Currency` trait](https://substrate.dev/rustdocs/master/frame_support/traits/trait.Currency.html) can be used. The weight-fee-runtime demonstrates how to use an asset provided by the [`Generic Asset` pallet](https://substrate.dev/rustdocs/master/pallet_generic_asset/index.html).

src: [`runtimes/weight-fee-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/weight-fee-runtime/src/lib.rs)

```rust,ignore
impl transaction_payment::Trait for Runtime {

	// A generic asset whose ID is stored in the generic_asset pallet's runtime storage
	type Currency = SpendingAssetCurrency<Self>;

	// --snip--
}
```
