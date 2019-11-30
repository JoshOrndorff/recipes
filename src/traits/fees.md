# Economic Security in Substrate <a name = "sec"></a>

An algorithm is considered to be *efficient* if its running time is polynomial in the size of the input, and *highly efficient* if its running time is linear in the size of the input. **It is important for all on-chain algorithms to be highly efficient, because they must scale linearly as the size of the Polkadot network grows**. In contrast, off-chain algorithms are only required to be efficient. - [Web3 Research](http://research.web3.foundation/en/latest/polkadot/NPoS/1.intro/)

Any resources used by a transaction must explicitly be paid for, and it is a module author's job to ensure that appropriate fees are required. Maintaining the balance between **resources used** and **price paid** is an important design activity for runtime security.

*Indeed, mispriced EVM operations have shown how operations that underestimate cost can open economic DOS attack vectors: [Onwards; Underpriced EVM Operations](https://www.parity.io/onwards/), [Under-Priced DOS Attacks on Ethereum](https://www4.comp.polyu.edu.hk/~csxluo/DoSEVM.pdf)*



Substrate provides several ways to affect the fees charges for executing a transaction. Substrate developer hub contains full details about [fees](https://substrate.dev/docs/en/next/development/module/fees) and [weights](https://substrate.dev/docs/en/next/conceptual/runtime/weight).

* Base fee - Applies a fixed fee to each and every transaction. A parameter in the `transaction_payment` module.

* Length fee - Applies a fee proportional to the transaction's length in bytes. The constant is a parameter in the `transaction_payment` module.

* Transaction weight - Each transaction can declare a weight, either fixed, or calculated from its parameters. This is exemplified briefly below and more thoroughly in the kitchen.

* Weight to Fee - A function to convert weight to fee. It doesn't need to be linear, although it often is. The same conversion function is applied across all transactions from all modules in the runtime. This is exemplified briefly below and more thoroughly in the kitchen.

## Assigning Transaction Weights

For simple transactions a fixed weight will do.
```rust, ignore
decl_module! {
	pub struct Module<T: Trait> for enum Call {

		#[weight = SimpleDispatchInfo::FixedNormal(100)]
		fn store_value(_origin, entry: u32) -> Result {
			// --snip--
		}
```

For more complex transactions, custom weight calculations can be performed.
```rust, ignore
pub struct Conditional(u32);

impl WeighData<(&bool, &u32)> for Conditional {
	fn weigh_data(&self, (switch, val): (&bool, &u32)) -> Weight {

		if *switch {
			val.saturating_mul(self.0)
		}
		else {
			self.0
		}
	}
}
```

These examples, and several others can be compiled in the kitchen.

While you can make reasonable estimates of resource consumption at
design time, it is always best to actually measure the resources
required of your functions through an empirical process. Failure to
perform such rigorous measurement may result in an economically
insecure chain.

## Converting Weight To Fees

In many cases converting weight to fees 1:1 will suffice and be accomplished with [`ConvertInto`](https://crates.parity.io/sr_primitives/traits/struct.ConvertInto.html). This approach is taken in the [node template](https://github.com/substrate-developer-hub/substrate-node-template/blob/43ee95347b6626580b1d9d554c3c8b77dc85bc01/runtime/src/lib.rs#L230) as well as the kitchen's own super runtime.
```rust, ignore
impl transaction_payment::Trait for Runtime {
	// --snip--
	type WeightToFee = ConvertInto;
}
```

This example uses a quadratic conversion and supports custom coefficients
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

These examples, and several others can be compiled in the kitchen.
