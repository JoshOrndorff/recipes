# Misc Optimizations
> cache for unfinished ideas

## Random

* a loop over a vector is significantly slower than over the vector as slice; it's noticeably faster to iterate over a slice rather than a `vec!`
* `.iter.map(|x| x.0.into()).collect`
* **using `BtreeMap<T, ()>` to test for deduplication** for cache optimality (trading search efficiency in some cases...)
    * using a different scope for this pattern
* `format!` macro to `write!` macro to make it so that you don't have to create new objects in the heap

* maps use `Blake2` hash
* storage values use `twox` hash
* `Blake2` is ~6x slower than `twox`
* if you have keys in your map, that can be manipulated from the outside; an attacker could try to create hash collisions.
* for the map, you can set the hasher you want to use => look up the correct hash for your type in the metadata.

### Small Vector Optimization

By default, the value-set for each key in the map uses the `smallvec` crate to keep a maximum of one element stored inline with the map, as opposed to separately heap-allocated with a plain `Vec`. Operations such as `Fit` and `Replace` will automatically switch back to the inline storage if possible. This is ideal for maps that mostly use one element per key, as it can improvate memory locality with less indirection.