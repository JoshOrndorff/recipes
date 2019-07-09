# Module Menu

The [official Substrate documentation](https://docs.substrate.dev/docs/srml-overview) provides a comprehensive overview of the Substrate runtime module libraries. Although modules are designed to be stand-alone, the modules in the [Substrate Runtime Module Library](https://github.com/paritytech/substrate/tree/master/srml) provide useful code patterns that are applicable to many applications leveraging the framework.

Unlike in smart contract development, the way to emulate these patterns is not to directly utilize these modules. Instead, the best approach either implements the same logic in the new context or utilizes a trait from [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/traits.rs) to guide the new implementation. By abstracting shared behavior from the runtime modules into [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/traits.rs), Substrate makes it easy to extract and enforce best practices in the unique runtime. You can find the trait documentation [here](https://crates.parity.io/srml_support/traits/index.html).

## Module Tour

* [Aura](https://crates.parity.io/srml_aura/index.html) - manages offline reporting for Aura consensus
* **[Balances](https://crates.parity.io/srml_balances/index.html)** - handles accounts and balances
* [Consensus](https://crates.parity.io/srml_consensus/index.html) - manages the authority set for the native code
* [Contract](https://crates.parity.io/srml_contract/index.htmlt) - functionality for the runtime to deploy and execute WebAssembly smart contracts
* **[Council](https://crates.parity.io/srml_council/index.html)** - handles voting and maintenance of council members
* **[Democracy](https://crates.parity.io/srml_democracy/index.html)** - handles administration of general stakeholder voting
* [Executive](https://crates.parity.io/srml_executive/index.html) - dispatches incoming extrinsic calls to the respective modules in the runtime
* [Grandpa](https://crates.parity.io/srml_grandpa/index.html) - manages the GRANDPA authority set ready for the native code
* [Indices](https://crates.parity.io/srml_indices/index.html) - an index is a short form of an address; this module handles allocation of indices for a newly created accounts
* [Session](https://crates.parity.io/srml_session/index.html) - allows validators to manage their session keys, provides a function for changing the session length, and handles session rotation
* **[Staking](https://crates.parity.io/srml_staking/index.html)** - manage funds at stake by network maintainers
* [Sudo](https://crates.parity.io/srml_sudo/index.html) - allows a single account to execute dispatchable functions
* [System](https://crates.parity.io/srml_system/index.html) - low-level access to core types and cross-cutting utilities
* [Timestamp](https://crates.parity.io/srml_timestamp/index.html) - get and set the on-chain time
* [Treasury](https://crates.parity.io/srml_treasury/index.html) - keeps account of currency in a `pot` and manages the subsequent deployment of these funds