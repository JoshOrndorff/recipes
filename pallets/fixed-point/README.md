# Fixed Point Arithmetic
*[`pallets/fixed-point`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/fixed-point)*

When programmers learn to use non-integer numbers in their programs, they are usually taught to use [floating point](https://en.wikipedia.org/wiki/Floating-point_arithmetic)s. In blockchain, we use an alternative representation of fractional numbers called [fixed point](https://en.wikipedia.org/wiki/Fixed-point_arithmetic). There are several ways to use fixed point numbers, and this recipe will introduce three of them. In particular we'll see:

* Substrate's own fixed point structs and traits
* The [substrate-fixed](https://github.com/encointer/substrate-fixed/) library
* A manual fixed point implementation (and why it's nicer to use a library)
* A comparison of the two libraries in a compounding interest example

## What's Wrong with Floats?

Floats are cool for all kinds of reasons, but they also have one important drawback. Floating point arithmetic is **nondeterministic** which means that different processors compute (slightly) different results for the same operation. Although there is an [IEEE spec](https://en.wikipedia.org/wiki/IEEE_754), nondeterminism can come from specific libraries used, or even hardware. In order for the nodes in a blockchain network to reach agreement on the state of the chain, all operations must be completely deterministic. Luckily fixed point arithmetic is deterministic, and is often not much harder to use once you get the hang of it.

## Multiplicative Accumulators

The first pallet covered in this recipe contains three implementations of a multiplicative accumulator. That's a fancy way to say the pallet lets users submit fractional numbers and keeps track of the product from multiplying them all together. The value starts out at one (the [multiplicative identity](https://en.wikipedia.org/wiki/Identity_element)), and it gets multiplied by whatever values the users submit. These three independent implementations compare and contrast the features of each.

### Permill Accumulator

We'll using the most common approach which takes its fixed point implementation from Substrate itself. There are a few fixed-point structs available in Substrate, all of which implement the [`PerThing` trait](https://substrate.dev/rustdocs/master/sp_arithmetic/trait.PerThing.html), that cover different amounts of precision. For this accumulator example, we'll use the [`PerMill` struct](https://substrate.dev/rustdocs/master/sp_arithmetic/struct.Permill.html) which represents fractions as parts per million. There are also [`Perbill`](https://substrate.dev/rustdocs/master/sp_arithmetic/struct.Perbill.html), [`PerCent`](https://substrate.dev/rustdocs/master/sp_arithmetic/struct.Percent.html), and [`PerU16`](https://substrate.dev/rustdocs/master/sp_arithmetic/struct.PerU16.html), which all provide the same interface (because it comes from the trait). Substrate's fixed-point structs are somewhat unique because they represent _only_ fractional parts of numbers. That means they can represent numbers between 0 and 1 inclusive, but _not_ numbers with whole parts like 2.718 or 3.14.

To begin we declare the storage item that will hold our accumulated product. You can see that the trait provides a handy function for getting the identity value which we use to set the default storage value to `1`.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		// --snip--

		/// Permill accumulator, value starts at 1 (multiplicative identity)
		PermillAccumulator get(fn permill_value): Permill = Permill::one();
	}
}
```

The only extrinsic for this Permill accumulator is the one that allows users to submit new `Permill` values to get multiplied into the accumulator.

```rust, ignore
fn update_permill(origin, new_factor: Permill) -> DispatchResult {
	ensure_signed(origin)?;

	let old_accumulated = Self::permill_value();

	// There is no need to check for overflow here. Permill holds values in the range
	// [0, 1] so it is impossible to ever overflow.
	let new_product = old_accumulated.saturating_mul(new_factor);

	// Write the new value to storage
	PermillAccumulator::put(new_product);

	// Emit event
	Self::deposit_event(Event::PermillUpdated(new_factor, new_product));
	Ok(())
}
```

The code of this extrinsic largely speaks for itself. One thing to take particular note of is that we _don't_ check for overflow on the multiplication. If you've read many of the recipes you know that a Substrate runtime must never panic, and a developer must be extremely diligent in always checking for and gracefully handling error conditions. Because `Permill` only holds values between 0 and 1, we know that their product will always be in that same range. Thus it is impossible to overflow or saturate. So we can happily use `saturating_mul` and move on.

### Substrate-fixed Accumulator

[Substrate-fixed](https://github.com/encointer/substrate-fixed/) takes a more traditional approach in that their types represent numbers with both whole _and_ fractional parts. For this implementation, we'll use the `U16F16` type. This type contains an unsigned number (indicated by the `U` at the beginning) and has 32 _total_ bits of precision - 16 for the integer part, and 16 for the fractional part. There are several other types provided that follow the same naming convention. Some examples include `U32F32` and `I32F32` where the `I` indicates a signed number, just like in Rust primitive types.

As in the `Permill` example, we begin by declaring the storage item. With substrate-fixed, there is not a `one` function, but there is a `from_num` function that we use to set the storage item's default value. This `from_num` method and its counterpart `to-num` are your primary ways of converting between substrate-fixed types and Rust primitive types. If your use case does a lot of fixed-point arithmetic, like ours does, it is advisable to keep your data in substrate-fixed types.

> We're able to use `U16F16` as a storage item type because it, and the other substrate-fixed types, implements the parity scale codec.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		// --snip--

		/// Substrate-fixed accumulator, value starts at 1 (multiplicative identity)
		FixedAccumulator get(fn fixed_value): U16F16 = U16F16::from_num(1);
	}
}
```

Next we implement the extrinsic that allows users to update the accumulator by multiplying in a new value.

```rust, ignore
fn update_fixed(origin, new_factor: U16F16) -> DispatchResult {
	ensure_signed(origin)?;

	let old_accumulated = Self::fixed_value();

	// Multiply, handling overflow
	let new_product = old_accumulated.checked_mul(new_factor)
		.ok_or(Error::<T>::Overflow)?;

	// Write the new value to storage
	FixedAccumulator::put(new_product);

	// Emit event
	Self::deposit_event(Event::FixedUpdated(new_factor, new_product));
	Ok(())
}
```

This extrinsic is quite similar to the `Permill` version with one notable difference. Because `U16F16` handles numbers greater than one, overflow is possible, and we need to handle it. The error handling here is straightforward, the important part is just that you remember to do it.

This example has shown the fundamentals of substrate-fixed, but this library has much more to offer as we'll see in the compounding interest example.

### Manual Accumulator

In this final accumulator implementation, we manually track fixed point numbers using Rust's native `u32` as the underlying data type. This example is educational, but is only practical in the simplest scenarios. Generally you will have a ~~more fun~~ less error-prone time coding if you use one of the previous two fixed-point types in your real-world applications.

Fixed point is not very complex conceptually. We represent fractional numbers as regular old integers, and we decide in advance to consider some of the place values fractional. It's just like saying we'll omit the decimal point when talking about money and all agree that "1995" actually _means_ 19.95 €. This is exactly how Substrate's [Balances pallet](https://substrate.dev/rustdocs/master/pallet_balances/index.html) works, a tradition that's been in blockchain since Bitcon. In our example we will treat 16 bits as integer values, and 16 as fractional, just as substrate-fixed's `U16F16` did.

If you're rusty or unfamiliar with place values in the [binary number system](https://en.wikipedia.org/wiki/Binary_number), it may be useful to brush up. (Or skip this detailed section and proceed to the compounding interest example.)

```
Normal interpretation of u32 place values
... ___ ___ ___ ___ ___ ___ ___ .
...  64  32  16  8   4   2   1

Fixed interpretation of u32 place values
... ___ ___ ___ ___ . ___ ___ ___ ___ ...
...  8   4   2   1    1/2 1/4 1/8 1/16...
```

Although the concepts are straight-forward, you'll see that manually implementing operations like multiplication is quite error prone. Therefore, when writing your own blockchain applications, it is often best to use on of the provided libraries covered in the other two implementations of the accumulator.

As before, we begin by declaring the storage value. This time around it is just a simple u32. But the default value, `1 << 16` looks quite funny. If you haven't encountered it before `<<` is Rust's [bit shift operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators). It takes a value and moves all the bits to the left. In this case we start with the value `1` and move it 16 bits to the left. This is because Rust interprets `1` as a regular `u32` value and puts the `1` in the far right place value. But because we're treating this `u32` specially, we need to shift that bit to the middle just left of the imaginary radix point.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		// --snip--

		/// Manual accumulator, value starts at 1 (multiplicative identity)
		ManualAccumulator get(fn manual_value): u32 = 1 << 16;
	}
}
```

The extrinsic to multiply a new factor into the accumulator follows the same general flow as in the other two implementations. In this case, there are more intermediate values calculated, and more comments explaining the bit-shifting operations. In the function body most intermediate values are held in `u64` variables. This is because when you multiply two 32-bit numbers, you can end up with as much as 64 bits in the product.

```rust, ignore
fn update_manual(origin, new_factor: u32) -> DispatchResult {
	ensure_signed(origin)?;

	// To ensure we don't overflow unnecessarily, the values are cast up to u64 before multiplying.
	// This intermediate format has 48 integer positions and 16 fractional.
	let old_accumulated : u64 = Self::manual_value() as u64;
	let new_factor_u64 : u64 = new_factor as u64;

	// Perform the multiplication on the u64 values
	// This intermediate format has 32 integer positions and 32 fractional.
	let raw_product : u64 = old_accumulated * new_factor_u64;

	// Right shift to restore the convention that 16 bits are fractional.
	// This is a lossy conversion.
	// This intermediate format has 48 integer positions and 16 fractional.
	let shifted_product : u64 = raw_product >> 16;

	// Ensure that the product fits in the u32, and error if it doesn't
	if shifted_product > (u32::max_value() as u64) {
		return Err(Error::<T>::Overflow.into())
	}

	// Write the new value to storage
	ManualAccumulator::put(shifted_product as u32);

	// Emit event
	Self::deposit_event(Event::ManualUpdated(new_factor, shifted_product as u32));
	Ok(())
}
```

As mentioned above, when you multiply two 32-bit numbers, you can end up with as much as 64 bits in the product. In this 64-bit intermediate product, we have 32 integer bits and 32 fractional. We can simply throw away the 16 right-most fractional bits that merely provide extra precision. But we need to be careful with the 16 left-most integer bits. If any of those bits are non-zero after the multiplication it means overflow has occurred. If they are all zero, then we can safely throw them away as well.

> If this business about having more bits after the multiplication is confusing, try this exercise in the more familiar decimal system. Consider these numbers that have 4 total digits (2 integer, and two fractional): 12.34 and 56.78. Multiply them together. How many integer and fractional digits are in the product? Try that again with larger numbers like 01.23 * 02.48 and smaller like 11.11 and 22.22. Which of these products can be fit back into a 4-digit number like the ones we started with?

## Learn More

For a more realistic example of using fixed point arithmetic in practice, see the recipe on [compounding interest](../compounding-interest/README.md)
