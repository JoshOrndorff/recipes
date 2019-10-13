# Cheap Inclusion Proofs: Child Tries
*[`kitchen/modules/child-trie`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/child-trie)*

* useful for proving key inclusion in the context of a set of `(key, value)` pairs
* I'm unsure whether there are advantages to retroactively searching or iterating through the associated values thereafter (relative to a vec)