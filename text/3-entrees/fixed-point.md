# Fixed Point Arithmetic
*[`pallets/fixed-point`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/fixed-point)* *[`pallets/compounding-interest`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/compounding-interest)*

When programmers learn to use non-integer numbers in their programs, they are usually taught to use [floating point](https://en.wikipedia.org/wiki/Floating-point_arithmetic)s. In blockchain, we use an alternative representation of fractional numbers called [fixed point](https://en.wikipedia.org/wiki/Fixed-point_arithmetic). There are several ways to use fixed point numbers, and this recipe will introduce all of them. In particular we'll see:

* A manual implementation of fixed point math (and why it's nicer to use a library)
* Substrate's own implementation of base-ten fixed point
* An external library that provides base-two fixed point
* A comparison of the two libraries in a compounding interest example

## What's Wring with Floats?

Floats are cool for all kinds of reasons, but they also have one important drawback. Floating point arithmetic is **nondeterministic**. In order for the nodes in a blockchain network to reach agreement on the state of the Chain, all operations must be completely deterministic. Luckily fixed point arithmetic is deterministic.

## Multiplicative Accumulators

The first pallet covered in this recipe contains three implementations of a multiplicative accumulator. That's a fancy way to say the pallet lets users submit numbers and keeps track of the product from multiplying them all together. The value starts out at one (the [multiplicative identity](https://en.wikipedia.org/wiki/Identity_element)), and it gets multiplied by whatever value the user submits. The three implementations compare and contrast the features of each.

### Manual Accumulator

Fixed Point is really not very complex conceptually. We represent fractional numbers as regular old integers, and we decide in advance to consider some of the place values fractional. It's just like saying we'll write 1995 when we really mean 19.95. Although the concepts are straight-forward, you'll see that manually implementing operations like multiplication is quite error prone. Therefore, when writing your own blockchain applications, it is often best to use on of the provided libraries covered in the other two implementations of the accumulator.

In this example we'll

### Permill Accumulator

### Substrate-fixed Accumulator

## Compounding Interest

Many financial agreements involve interest for loaned money. [Compounding interest](https://en.wikipedia.org/wiki/Compound_interest) is when interest is paid on top of not only the original loan amount, the so-called principle, but also any interest that has been previously paid.

### Discrete Compounding

Our first example will look at discrete compounding interest. This is when interest is paid at a fixed interval. In our case, interest will be paid every ten blocks.

For this implementation we've chosen to use Substrate's [`Percent` type]. It works nearly the same as `Permill`. We could also have used Substrate-fixed for this implementation, but chose to save it for the next example.

### Continuously Compounding
You can imagine increasing the frequency a which the interest is paid out. Increasing the frequency enough approaches [continuously compounding interest](https://en.wikipedia.org/wiki/Compound_interest#Continuous_compounding). Calculating continuously compounding interest requires the [exponential function](https://en.wikipedia.org/wiki/Exponential_function) which is only available in Substrate-fixed.
