# Consensus 

* **[`contract`](https://github.com/paritytech/substrate/tree/master/srml/contract)** - enables deployment and execution of smart-contracts expressed in WebAssembly
* **[`consensus`](https://github.com/paritytech/substrate/tree/master/srml/consensus)** - consensus module for runtime; manages the authority set ready for the native code
* **[`aura`](https://github.com/paritytech/substrate/tree/master/srml/aura)** - live block production algorithm
* **[`rhododendron`](https://github.com/paritytech/rhododendron)** - asynchronously safe, futures-based BFT (work-in-progress)
* **[`grandpa`](https://github.com/paritytech/substrate/tree/master/srml/grandpa)** - [GRANDPA](https://medium.com/polkadot-network/grandpa-block-finality-in-polkadot-an-introduction-part-1-d08a24a021b5) consensus module (provides finality on top of `aura` or similar block production algorithms)
* **[`finality-tracker`](https://github.com/paritytech/substrate/tree/master/srml/finality-tracker)** - tracks the last finalized block, as perceived by block authors