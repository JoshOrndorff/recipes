# Cheap Inclusion Proofs: Child Tries
*[`kitchen/modules/child-trie`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/child-trie)*

* useful for proving key inclusion in the context of a set of `(key, value)` pairs
* I'm unsure whether there are advantages to retroactively searching or iterating through the associated values thereafter (relative to a vec)

## open questions

- does the child trie require `(key, value)` pairs?
- how efficient is iteration through a child-trie vs iteration through a vector?
- ...a question I still have is how/whether this actually reduces the complexity of inclusion proofs. If binary search of a vector is log(n) and trie proofs are log(n)?

## references
TODO: *[relevant issue (priority)](https://github.com/substrate-developer-hub/recipes/issues/35)*