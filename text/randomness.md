# Generating Randomness

`pallets/randomness`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Frandomness%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/randomness/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

Randomness is useful in computer programs for everything from gambling, to generating DNA for
digital kitties, to selecting block authors. Randomness is hard to come by in deterministic
computers as explained at [random.org](https://www.random.org/randomness/). This is particularly
true in the context of a blockchain when all the nodes in the network must agree on the state of the
chain. Some techniques have been developed to address this problem including
[RanDAO](https://github.com/randao/randao) and
[Verifiable Random Functions](https://en.wikipedia.org/wiki/Verifiable_random_function). Substrate
abstracts the implementation of a randomness source using the
[`Randomness` trait](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Randomness.html), and
provides a few implementations. This recipe will demonstrate using the `Randomness` trait and two
concrete implementations.

## Disclaimer

All of the randomness sources described here have limitations on their usefulness and security. This
recipe shows how to use these randomness sources and makes an effort to explain their trade-offs.
However, the author of this recipe is a blockchain chef, **not a trained cryptographer**. It is your
responsibility to understand the security implications of using any of the techniques described in
this recipe, before putting them to use. When in doubt, consult a trustworthy cryptographer.

The resources linked at the end of this recipe may be helpful in assessing the security and
limitations of these randomness sources.

## Randomness Trait

The randomness trait provides two methods, `random_seed`, and `random`, both of which provide a
pesudo-random value of the type specified in the traits type parameter.

### `random_seed`

The `random_seed` method takes no parameters and returns a random seed which changes once per block.
If you call this method twice in the same block you will get the same result. This method is
typically not as useful as its counterpart.

### `random`

The `random` method takes a byte array, `&[u8]`, known as the subject, and uses the subject's bytes
along with the random seed described in the previous section to calculate a final random value.
Using a subject in this way allows pallet (or multiple pallets) to seek randomness in the same block
and get different results. The subject does not add entropy or security to the generation process,
it merely prevents each call from returning identical values.

Common values to use for a subject include:

-   The block number
-   The caller's accountId
-   A Nonce
-   A pallet-specific identifier
-   A tuple containing several of the above

To bring a randomness source into scope, we include it in our configuration trait with the
appropriate trait bound. This pallet, being a demo, will use two different sources. Using multiple
sources is not necessary in practice.

```rust, ignore
pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;

	type RandomnessSource: Randomness<H256>;
}
```

We've provided the `Output` type as [`H256`](https://substrate.dev/rustdocs/v3.0.0/sp_core/struct.H256.html).

## Consuming Randomness

Calling the randomness source from Rust code is straightforward. Our `consume_randomness` extrinsic
demonstrates consuming the raw random seed as well as a context-augmented random value. Try submitting the same extrinsic twice in the same block. The raw seed should be the same each time.

```rust, ignore
fn consume_randomness(origin) -> DispatchResult {
	let _ = ensure_signed(origin)?;

	// Using a subject is recommended to prevent accidental re-use of the seed
	// (This does not add security or entropy)
	let subject = Self::encode_and_update_nonce();

	let random_seed = T::RandomnessSource::random_seed();
	let random_result = T::RandomnessSource::random(&subject);

	Self::deposit_event(Event::RandomnessConsumed(random_seed, random_result));
	Ok(())
}
}
```

## Collective Coin Flipping

Substrate's
[Randomness Collective Flip pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_randomness_collective_flip/index.html)
uses a safe mixing algorithm to generate randomness using the entropy of previous block hashes.
Because it is dependent on previous blocks, it can take many blocks for the seed to change.

A naive randomness source based on block hashes would take the hash of the previous block and use it
as a random seed. Such a technique has the significant disadvantage that the block author can
preview the random seed, and choose to discard the block choosing a slightly modified block with a
more desirable hash. This pallet is subject to similar manipulation by the previous 81 block authors
rather than just the previous 1.

Although it may _seem_ harmless, **you should not hash the result** of the randomness provided by
the collective flip pallet. Secure hash functions satisfy the
[Avalance effect](https://en.wikipedia.org/wiki/Avalanche_effect) which means that each bit of input
is equally likely to affect a given bit of the output. Hashing will negate the low-influence
property provided by the pallet.

## Babe VRF Output

Substrate's [Babe pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_babe/index.html) which is primarily
responsible for managing validator rotation in Babe consensus, also collects the VRF outputs that
Babe validators publish to demonstrate that they are permitted to author a block. These VRF outputs
can be used to provide a random seed.

Because we are accessing the randomness via the `Randomness` trait, the calls look the same as
before.

```rust, ignore
let random_seed = T::BabeRandomnessSource::random_seed();
let random_result = T::BabeRandomnessSource::random(&subject);
```

In production networks, Babe VRF output is preferable to Collective Flip. Collective Flip provides
essentially no real security.

## Down the Rabbit Hole

As mentioned previously, there are many tradeoffs and security concerns to be aware of when using
these randomness sources. If you'd like to get into the research, here are some jumping off points.

-   [https://github.com/paritytech/ink/issues/57](https://github.com/paritytech/ink/issues/57)
-   [https://wiki.polkadot.network/docs/en/learn-randomness](https://wiki.polkadot.network/docs/en/learn-randomness)
<!-- markdown-link-check-disable-next-line -->
-   [http://www.cse.huji.ac.il/~nati/PAPERS/coll_coin_fl.pdf](http://www.cse.huji.ac.il/~nati/PAPERS/coll_coin_fl.pdf)
-   [https://eccc.weizmann.ac.il/report/2018/140/](https://eccc.weizmann.ac.il/report/2018/140/)
