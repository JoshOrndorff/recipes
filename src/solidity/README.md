# Prototype in Solidity => Scale on Substrate

This tutorial follows the recommended pattern of prototyping with Solidity on Ethereum before gradually migrating to Substrate. We assume some familiarity with Solidity, but it may not be necessary if you have experience with TypeScript or any other strongly typed language in the past.

**Prototype on Ethereum**
1. permissionless deployment
2. costs are passed to users via *gas*
3. network effects

**Scale on Substrate**
1. Compilation from Rust to WASM facilitates on-chain upgrades, thereby increasing the application's relative flexibility. 
2. Rust's low-level handling encourages creative code patterns that optimize performance while protecting memory safety. 
3. Deployment in the context of Polkadot fosters shared security without expensive spillover costs from other parachain activity.

So, why not just start with Substrate? Well, Rust maintains a relatively high learning curve compounded by the macro magic used in Substrate, but, trust me, it's worth it! To be thoroughly convinced, [read more here](https://github.com/AmarRSingh/Parity/blob/master/Blog/ysubstrate/v3.md).

* [Organizing Code](./org.md)
* [Data Structures](./data.md)
* [Modifiers and Traits](./modifiers.md)
* [Functions](./fn.md)
* [Key Differences](./diff.md)
* [Bonus: Smart Contracts on Substrate](./contracts.md)

## Related Samples

* MoloChameleon DAO
* Liberal Radicalism DAO
* Uniswap and MakerDAO