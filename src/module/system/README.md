# System

* **[`system`](https://github.com/paritytech/substrate/tree/master/srml/system)** - handles depositing logs, basic set up and take down of temporary storage entries, access to old block hashes
* **[`support`](https://github.com/paritytech/substrate/tree/master/srml/support)** - support code for the runtime
* **[`executive`](https://github.com/paritytech/substrate/tree/master/srml/executive)** - handles all of the top-level stuff; essentially just executing blocks/extrinsics
* **[`indices`](https://github.com/paritytech/substrate/tree/master/srml/indices)** - an index is a short form of an address;  this module handles allocation of indices for a newly created accounts
* **[`timestamp`](https://github.com/paritytech/substrate/tree/master/srml/timestamp)** - provides means to find out the current time (set by validators at the beginning of each block)

