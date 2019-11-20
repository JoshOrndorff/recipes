# Custom Runtime Environments

* use `new_test_ext` to introduce new syntax `=>` do this for `smpl-treasury`
* genesis config (balances)
* `add_genesis` based on `scored_pool` example

* auxiliary methods
* I expect that runtime testing might be separated from offchain workers testing such that runtime assumes interaction from ofc_worker, interaction is tested, and the ofc_worker is also tested separately `=>` it seems difficult to test them together without generating a whole lots of states...

## contracts-based constant configuration

* constant configuration