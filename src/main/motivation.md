Before smart contract blockchain platforms, application developers struggled to build software that effectively leveraged blockchain technology. 

While Bitcoin's proof of work algorithm incentivizes robust security, interaction with the UTXO model requires complicated architecture. This additional complexity constrains smart contract implementations, thereby limiting the near-term innovation that could occur on the application layer.

Although developers *could* alternatively launch their own permissionless blockchain, this option represents a false promise. Successful deployment of public blockchains is ridiculously difficult. Aside from the technical challenges of implementing networking, cryptography, consensus, and supporting infrastructure, permissionless blockchains require substantial community building as well as the incentivization of ongoing maintenance.

## DApps

The advent of smart contract platforms like Ethereum enabled rapid prototyping. By providing a sandboxed and secure execution environment, these platforms paved the way for innovation by appealing to developers. Keeping a global store of account balances fosters *statefulness*, thereby simplifying implementation logic in the context of basic value transfer as well as smart contract interaction. The technical burden placed on developers is also eased by the fact that users pay for the resources used. In the end, these factors decrease barriers to entry for curious developers and encourage innovative prototyping.

> *A bare-bones version of Namecoin can be written in two lines of code, and other protocols like currencies and reputation systems can be built in under twenty.* [Ethereum Whitepaper](https://github.com/ethereum/wiki/wiki/White-Paper#ethereum)

Although public smart contract blockchains simplify development, applications built on these platforms are still forced to accept the tradeoffs decided by protocol developers. This often translates to sacrificing privacy as well as scalability. By default, transactions stored on-chain are not encrypted and can be tracked by anyone watching the chain. Moreover, the spillover effects of heightened demand for a single DApp (ie Cryptokitties) can make interaction with all other deployed contracts prohibitively expensive.

## DAppchains

To increase privacy, developers can launch private DAppchains with Quorum, Hyperledger Fabric, or any other number of permissioned platforms. By construction, these platforms distribute trust among the defined participants. In cases where enough of the participants collude, protocol execution will not be guaranteed (versus in the case of public blockchains which distribute trust among an open validator/miner set). At the same time, this security threshold satisfies many enterprise applications (ie supply chain management).

By siloing economic interactions, permissioned blockchain frameworks increase efficiency and decrease costs for comparable resource usage. Depending on the implementation, costs *may* also be abstracted away from user interaction and delegated to the specific actors in charge of execution. However, modern permissioned frameworks tend to make performance-critical choices on behalf of application developers. This lack of flexibility limits future iterations.

Indeed, for all existing options, upgradability remains an ongoing concern. As development in the space continues at an increasingly rapid pace, it is wishful thinking to pretend that DApp deployments will be static. 

## Iterate with Substrate

**[Substrate](https://www.parity.io/what-is-substrate/)** provides DApp developers with a toolbox to implement a custom blockchain catered to the specific requirements of their application. By leveraging the Substrate Runtime Module Library ([SRML](https://github.com/paritytech/substrate)), developers can selectively choose the features that are useful for their application. 

In this way, Substrate empowers developers with increased flexibility without forcing them to build everything from the ground up. Substrate comes fully stocked with cryptographic primitives, [light client functionality](https://www.parity.io/what-is-a-light-client/), [networking support](https://www.parity.io/why-libp2p/), and modern consensus algorithms (as well as [tons of other cool stuff](https://www.parity.io/what-is-substrate/)). 

### Our Chain, Our Rules
Developers who build DAppchains with Substrate determine their own tradeoffs.

By coding the Substrate stack in [Rust](https://www.parity.io/why-rust/), the runtime logic can be stored on-chain in a [WASM binary blob](https://medium.com/polkadot-network/wasm-on-the-blockchain-the-lesser-evil-da8d7c6ef6bd). This architecture facilitates on-chain upgrades according to the consensus protocol chosen by developers. Because the consensus logic is included in the runtime, it can also be upgraded. This flexibility allows Substrate DAppchains to evolve and easily incorporate modern research into the runtime logic.

## With Great Power Comes Great Responsibility

Although Substrate provides increased freedom and flexibility, not every blockchain use case requires its own blockchain. 

> *It doesn't make sense to take a jet for a 10 km commute.*

For common token transfers, basic timestamping, and other simple blockchain use cases, existing smart contract platforms are still preferrable. By implementing computational metering and conditional transaction reversion, public smart contract platforms are conducive to non-upgradeable applications that require only occasional interaction with the blockchain.

Additionally, public smart contract blockchains foster [rich composability](https://www.youtube.com/watch?v=0IoUZdDi5Is&feature=youtu.be&t=47m27s) between deployed contracts. With a few existing public blockchains experiencing significant network effects, it may be advantageous to initially prototype on these platforms. When a DApp needs to scale to production, developers can migrate the smart contract logic to a Substrate-based DAppchain to enjoy increased efficiency as well as flexibility with respect to future upgrades.

Check out the [Substrate repo on Github](https://github.com/paritytech/substrate/), the [conceptual documentation](https://docs.substrate.dev/), and the [examples prepared by the Parity-Samples team](https://github.com/parity-samples).

## Great Videos to Get Started
* [Web3 Summit: Spin Up Your Own Blockchain with Substrate in Less Than 30 minutes](https://www.youtube.com/watch?v=0IoUZdDi5Is&feature=youtu.be&t=)
* [Parachains vs Smart Contracts](https://www.youtube.com/watch?v=LRAqF-8samI) by Adrian
* [Parachains vs Smart Contracts Panel in London](https://www.youtube.com/watch?v=xpjJPuQvSu4)
* [Implications of Interoperability](https://www.youtube.com/watch?v=TBeGIGvC6r8) by Rob
* [Composability Question at Web3 Summit](https://youtu.be/0IoUZdDi5Is?t=47m27s) (answer by Gavin)