# Cheap Inclusion Proofs: Child Tries
*[`kitchen/modules/child-trie`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/child-trie)*




## notes on usage norms

The child trie is much more costly than a standard kv store. There is a tradeoff between the size of an inclusion proof and the distance between different fields (in a trie). To me, this implies that for small kv-stores, it is not useful to use child-tries, but, for larger stores that require inclusion proofs, the child trie can be useful because the relevant membership proofs are small in size.

* useful for proving key inclusion in the context of a set of `(key, value)` pairs

## examples and ideas

* crowdfund `=>` substrate-ico
* kickback
* voting modules (proving participation)
* using `offchain-workers` to minimize iteration `=>` use alongside `double_map` as a cache as `child_trie` as the large store for big structs `=>` we don't want to store a lot of data in the runtime...

## direction and storage in general...

structs aren't stored in the runtime, etc etc etc

## open questions (discussed in original issue as well)
* I'm unsure whether there are advantages to retroactively searching or iterating through the associated values thereafter (relative to a vec)