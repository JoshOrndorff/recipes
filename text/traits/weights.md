# Computational Resources and Weights
*[`kitchen/pallets/weights`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/weights)*

Any computational resources used by a transaction must be accounted for so that appropriate fees can be applied, and it is a pallet author's job to ensure that this accounting happens. Substrate provides a mechanism known as transaction weighting to quantify the resources consumed while executing a transaction.

*Indeed, mispriced EVM operations have shown how operations that underestimate cost can open economic DOS attack vectors: [Onwards; Underpriced EVM Operations](https://www.parity.io/onwards/), [Under-Priced DOS Attacks on Ethereum](https://www4.comp.polyu.edu.hk/~csxluo/DoSEVM.pdf)*


## Assigning Transaction Weights
Pallet authors can annotate their dispatchable function with a weight using syntax like this,
```rust, ignore
#[weight = <Some Weighting Instance>]
fn some_call(...) -> Result {
	// --snip--
}
```

For simple transactions a fixed weight will do. Substrate provides the [`SimpleDispatchInfo` enum](https://substrate.dev/rustdocs/master/frame_support/weights/enum.SimpleDispatchInfo.html) for situations like this.
```rust, ignore
decl_module! {
	pub struct Module<T: Trait> for enum Call {

		#[weight = SimpleDispatchInfo::FixedNormal(100)]
		fn store_value(_origin, entry: u32) -> Result {
			// --snip--
		}
```

For more complex transactions, custom weight calculations can be performed that consider the parameters passed to the call. This snippet shows a weighting struct that weighs transactions where the first parameter
is a `boo`l. If the first parameter is `true`, then the weight is linear in the second parameter. Otherwise the weight is constant. A transaction where this weighting scheme makes sense is demonstrated in the kitchen.
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

In addition to the [`WeightData`
Trait](https://substrate.dev/rustdocs/master/frame_support/weights/trait.WeighData.html), shown
above, types that are used to calculate transaction weights, must also implement
[`ClassifyDispatch`](https://substrate.dev/rustdocs/master/frame_support/weights/trait.ClassifyDispatch.html),
and [`PaysFee`](https://substrate.dev/rustdocs/master/frame_support/weights/trait.PaysFee.html).


```rust,ignore
impl<T> ClassifyDispatch<T> for Conditional {
    fn classify_dispatch(&self, _: T) -> DispatchClass {
        // Classify all calls as Normal (which is the default)
        Default::default()
    }
}
```

```rust,ignore
impl PaysFee for Conditional {
    fn pays_fee(&self) -> bool {
        true
    }
}
```

The complete code for this example as well as several others can be found in the kitchen.

## Cautions

While you can make reasonable estimates of resource consumption at
design time, it is always best to actually measure the resources
required of your functions through an empirical process. Failure to
perform such rigorous measurement may result in an economically
insecure chain.

While it isn't enforced, calculating a transaction's weight should itself be a cheap operation. If the weight calculation itself is expensive, your chain will be insecure.

## What About Fees?
Weights are used only to describe the computational resources consumed by a transaction, and enable accounting of these resources. To learn how to turn these weights into actual fees charged to transactors, continue to the recipe on [Fees](./fees.md).
