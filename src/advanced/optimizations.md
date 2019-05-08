## Optimization

Because Substrate is written in Rust, writing optimized Rust code reduces runtime overhead (costs) for Substrate deployments. Likewise, it is important to write clean, high-performance Rust code. There are a few tips here.

We call an algorithm "efficient" if its running time is polynomial in the size of the input, and "highly efficient" if its running time is linear in the size of the input. It is important for all on-chain algorithms to be highly efficient, because they must scale well as the size of the Polkadot network grows. In contrast, off-chain algorithms are only required to be efficient. [src](http://research.web3.foundation/en/latest/polkadot/NPoS/1.intro/)

## Iterate Through a Slice Rather than a Vec!

It's noticeably faster to iterate over a slice rather than a `vec!`.
* `.iter.map(|x| x.0.into()).collect`

## Small Vector Optimization

By default, the value-set for each key in the map uses the `smallvec` crate to keep a maximum of one element stored inline with the map, as opposed to separately heap-allocated with a plain `Vec`. Operations such as `Fit` and `Replace` will automatically switch back to the inline storage if possible. This is ideal for maps that mostly use one element per key, as it can improvate memory locality with less indirection.

### MISC NOTES

* maps use `Blake2` hash
* storage values use `twox` hash
* `Blake2` is ~6x slower than `twox`
* if you have keys in your map, that can be manipulated from the outside; an attacker could try to create hash collisions.
* for the map, you can set the hasher you want to use => look up the correct hash for your type in the metadata.