**[Substrate](https://www.parity.io/what-is-substrate/)** fundamentally changes how teams interact with blockchain technology; this modular toolbox empowers developers to build custom blockchains catered to the specific requirements of their applications. 

By leveraging the Substrate Runtime Module Library ([SRML](https://github.com/paritytech/substrate)), developers can selectively choose the features that are useful for their application. In this way, Substrate gives developers increased flexibility without forcing them to build everything from the ground up. 

Substrate comes fully stocked with cryptographic primitives, [light client functionality](https://www.parity.io/what-is-a-light-client/), [networking support](https://www.parity.io/why-libp2p/), as well as [tons of other cool stuff](https://www.parity.io/what-is-substrate/). The modularity of the runtime module structure fosters dynamic implementations that can be updated according to the consensus algorithm chosen by the application developers.

Developers should opt to use Substrate when they are building applications that require high performance, integration of off-chain information in on-chain processes, as well as other complex logic that extends beyond basic blockchain use cases. Even so, not every use case requires its own blockchain. Indeed, *it doesn't make sense to take a jet for a 10 km commute.*

## Prototype with Smart Contracts

For common token transfers, timestamping, and basic token sales, existing smart contract platforms are still preferrable. By implementing computational metering and conditional transaction reversion, public smart contract platforms are conducive to non-upgradeable applications that require only occasional interaction with the blockchain. 

As a result of storing a global state of account balances, Ethereum's sandboxed and secure execution environment also fosters *statefulness*, thereby simplifying implementation logic in the context of basic value transfer as well as smart contract interaction. The technical burden placed on developers is also eased by the fact that users pay for the resources used. In the end, these factors decrease barriers to entry for curious developers and encourage innovative prototyping.

> *A bare-bones version of Namecoin can be written in two lines of code, and other protocols like currencies and reputation systems can be built in under twenty.* [Ethereum Whitepaper](https://github.com/ethereum/wiki/wiki/White-Paper#ethereum)

Although public smart contract blockchains simplify development, applications built on these platforms are still forced to accept the tradeoffs decided by protocol developers. This often translates to sacrificing privacy as well as scalability. By default, transactions stored on-chain are not encrypted and can be tracked by anyone watching the chain. Moreover, the spillover effects of heightened demand for a single DApp (ie Cryptokitties) can make interaction with all other deployed contracts prohibitively expensive.

## Graduate to DAppchains

To increase privacy, developers can launch private DAppchains with Quorum, Hyperledger Fabric, or any other number of permissioned platforms. By construction, these platforms distribute trust among the defined participants. In cases where enough of the participants collude, protocol execution will not be guaranteed (versus in the case of public blockchains which distribute trust among an open validator/miner set). At the same time, this security threshold satisfies many enterprise applications (ie supply chain management).

By siloing economic interactions, permissioned blockchain frameworks increase efficiency and decrease costs for comparable resource usage. Depending on the implementation, costs *may* also be abstracted away from user interaction and delegated to the specific actors in charge of execution. 

However, modern permissioned frameworks tend to make performance-critical choices on behalf of application developers. This lack of flexibility limits future iterations. Indeed, for all existing options, upgradability remains an ongoing concern. As development in the space continues at an increasingly rapid pace, it is wishful thinking to pretend that DApp deployments will be static. 

## Migrate to Substrate
Developers who build DAppchains with Substrate determine their own tradeoffs.

By coding the Substrate stack in [Rust](https://www.parity.io/why-rust/), the runtime logic can be stored on-chain in a [WASM binary blob](https://medium.com/polkadot-network/wasm-on-the-blockchain-the-lesser-evil-da8d7c6ef6bd). This architecture facilitates on-chain upgrades according to the consensus protocol chosen by developers. Because the consensus logic is included in the runtime, it can also be upgraded. This flexibility allows Substrate DAppchains to evolve and easily incorporate modern research into the runtime logic.

With the advent of [Polkadot](https://medium.com/polkadot-network/polkadot-the-foundation-of-a-new-internet-e8800ec81c7) in Q4 2019, DAppchains built with Substrate can eventually be deployed in a shared security context. More specifically, Substrate-based DAppchains that implement the parachain interface for message passing and block authoring can join the Polkadot network (see [Cumulus](https://github.com/paritytech/cumulus)). Parachains deployed on Polkadot will enjoy the benefits of pooled validator security while also not incurring any additional costs if another parachain experiences a rapid increase in demand (no more Cryptokitties-esque state congestion).

*There are also significant benefits that arise from development with [Rust](https://medium.com/paritytech/why-rust-846fd3320d3f), [WebAssembly](https://medium.com/polkadot-network/wasm-on-the-blockchain-the-lesser-evil-da8d7c6ef6bd), and [Libp2p](https://www.parity.io/why-libp2p/).*

Check out the [Substrate repo on Github](https://github.com/paritytech/substrate/), the [conceptual documentation](https://docs.substrate.dev/), and the [examples prepared by the Parity-Samples team](https://github.com/parity-samples).

## Great Videos to Get Started
* [Web3 Summit: Spin Up Your Own Blockchain with Substrate in Less Than 30 minutes](https://www.youtube.com/watch?v=0IoUZdDi5Is&feature=youtu.be&t=)
* [Parachains vs Smart Contracts](https://www.youtube.com/watch?v=LRAqF-8samI) by Adrian
* [Parachains vs Smart Contracts Panel in London](https://www.youtube.com/watch?v=xpjJPuQvSu4)
* [Implications of Interoperability](https://www.youtube.com/watch?v=TBeGIGvC6r8) by Rob
* [Composability Question at Web3 Summit](https://youtu.be/0IoUZdDi5Is?t=47m27s) (answer by Gavin)