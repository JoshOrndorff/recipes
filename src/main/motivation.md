Today's DApp developers are forced to accept the tradeoffs made by existing blockchains. With only a few options, devs struggle to align the specific privacy, security, scalability, and governance requirements of their DApp with existing protocols.

Deploying a DApp on a **public smart contract platform** provides security through network effects, but often sacrifices privacy as well as scalability. When a DApp like [Cryptokitties experiences heightened demand](https://media.consensys.net/the-inside-story-of-the-cryptokitties-congestion-crisis-499b35d119cc), state bloat can render interaction prohibitively expensive for all other contracts deployed on-chain.

**Layer 2 solutions** scale by storing data off-chain and only committing periodic updates to the blockchain. Although this approach may be viable for some applications, modern frameworks [impose](https://twitter.com/JTremback/status/1097242527424364545) cumbersome costs to open and refill channels frequently.

To uphold privacy and delegate trust to a closed participant set, developers can launch their own **private DAppchain** with Quorum, Hyperledger Fabric, or any other number of permissioned platforms. While private DAppchains remain suitable for many enterprise applications (i.e. Supply Chain Management), existing frameworks make performance-critical choices on behalf of application developers, thereby limiting the flexibility of the implementation. 

For all applications built on blockchains, upgradability remains an ongoing concern. As development in the space continues at an increasingly rapid pace, it is wishful thinking to pretend that DApp deployments will be static.

While [proxy contract patterns](https://blog.zeppelinos.org/proxy-patterns/) provide a workaround for smart contract upgrades, they circumvent the problem at hand by launching new contracts and updating the proxy accordingly. This approach is unwieldy and only works in the absence of a sustainable model for [managing state bloat](https://www.ethnews.com/to-alleviate-ethereum-state-bloat-developers-consider-charging-rent).  

## Iterate with Substrate

**[Substrate](https://www.parity.io/what-is-substrate/)** provides DApp developers with a toolbox to implement a custom blockchain catered to the specific requirements of their application. By leveraging the Substrate Runtime Module Library ([SRML](https://github.com/paritytech/substrate/tree/master/srml)), developers can selectively choose the features that are useful for their application. 

In this way, Substrate empowers developers with increased flexibility without forcing them to build everything from the ground up. Substrate comes fully stocked with cryptographic primitives, [light client functionality](https://www.parity.io/what-is-a-light-client/), [networking support](https://www.parity.io/why-libp2p/), and modern consensus algorithms (as well as [tons of other cool stuff](https://www.parity.io/substrate-in-a-nutshell/)). 

### Our Chain, Our Rules
Developers who build DAppchains with Substrate determine their own tradeoffs.

By coding the Substrate stack in [Rust](https://www.parity.io/why-rust/), the runtime logic can be stored on-chain in a [WASM binary blob](https://medium.com/polkadot-network/wasm-on-the-blockchain-the-lesser-evil-da8d7c6ef6bd). This architecture facilitates on-chain upgrades according to the consensus protocol chosen by developers. Because the consensus logic is included in the runtime, it can also be upgraded. This flexibility allows Substrate DAppchains to evolve and easily incorporate modern research into the runtime logic.

With the advent of [Polkadot](https://medium.com/polkadot-network/polkadot-the-foundation-of-a-new-internet-e8800ec81c7) in Q4 2019, DAppchains built with Substrate can eventually be deployed in a shared security context. More specifically, Substrate-based DAppchains that implement the parachain interface for message passing and block authoring can join the Polkadot network. Parachains deployed on Polkadot will enjoy the benefits of pooled validator security while also not incurring any additional costs if another parachain experiences a rapid increase in demand (no more Cryptokitties-esque state congestion).

1. **Specialization breeds optimization**
2. Out-of-the-box features facilitate developability
3. Upgradeability fosters flexibility
4. Polkadot interoperability confers pooled security

*Not to mention the benefits of development with [Rust](https://medium.com/paritytech/why-rust-846fd3320d3f), [WebAssembly](https://medium.com/polkadot-network/wasm-on-the-blockchain-the-lesser-evil-da8d7c6ef6bd), and [Libp2p](https://www.parity.io/why-libp2p/).*

## "With Great Power comes Great Responsibility"

Although Substrate provides increased freedom and flexibility, not every blockchain use case requires its own blockchain. 

> *It doesn't make sense to take a jet for a 10 km commute.*

For common token transfers, basic timestamping, and other simple blockchain use cases, existing smart contract platforms are still preferrable. By implementing computational metering and conditional transaction reversion, public smart contract platforms are conducive to non-upgradeable applications that require only occasional interaction with the blockchain.

Additionally, public smart contract blockchains foster [rich composability](https://www.youtube.com/watch?v=0IoUZdDi5Is&feature=youtu.be&t=47m27s) between deployed contracts. With a few existing public blockchains experiencing significant network effects, it may be advantageous to initially prototype on these platforms. When a DApp needs to scale to production, developers can migrate the smart contract logic to a Substrate-based DAppchain to enjoy increased efficiency as well as flexibility with respect to future upgrades.

Even so, developers that use Substrate to scale to production must remain aware of the delicate balance between *resources used* and *price paid*. [Any resource used by a transaction (extrinsic) must be explicitly paid for within the module](LINKTOGUIDE). Although this can add complexity to the in-module fee structure, it is a necessary cost for alleviating state bloat and safeguarding against economic attacks.

In the Substrate Cookbook, we'll review best practices for development with Substrate. For more resources, check out [Substrate on Github](https://github.com/paritytech/substrate/), the [official documentation](https://substrate.readme.io/docs/what-is-substrate), and [examples prepared by the Parity-Samples team](https://github.com/parity-samples).

### Great Videos to Get Started
* [Web3 Summit: Spin Up Your Own Blockchain with Substrate in Less Than 30 minutes](https://www.youtube.com/watch?v=0IoUZdDi5Is&feature=youtu.be&t=)
* [Parachains vs Smart Contracts](https://www.youtube.com/watch?v=LRAqF-8samI) by Adrian
* [Parachains vs Smart Contracts Panel in London](https://www.youtube.com/watch?v=xpjJPuQvSu4)
* [Implications of Interoperability](https://www.youtube.com/watch?v=TBeGIGvC6r8) by Rob
* [Composability Question at Web3 Summit](https://youtu.be/0IoUZdDi5Is?t=47m27s) (answer by Gavin)