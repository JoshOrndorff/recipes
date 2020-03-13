# Basic Proof of Work
*[`nodes/basic-pow`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/basic-pow)*

The `basic-pow` node uses a minimal [Proof of Work](https://en.wikipedia.org/wiki/Proof_of_work) consensus engine to reach agreement over the blockchain. Being our first recipe on consensus, this node is kept intentionally simple. It omits some features that would make it practical for real-world use such as difficulty adjustment and block rewards. Nonetheless, it is a real usable consensus engine that will teach us many useful aspects of dealing with consensus and prepare us to understand more advanced consensus engines in the future. In particular we will learn about:
* Substrate's [`BlockImport` trait](https://substrate.dev/rustdocs/master/sp_consensus/block_import/trait.BlockImport.html)
* Substrate's [import pipeline](https://substrate.dev/rustdocs/master/sp_consensus/import_queue/index.html)
* Structure of a typical [Substrate Service](https://substrate.dev/rustdocs/master/sc_service/index.html)
* Configuring [`InherentDataProvider`](https://substrate.dev/rustdocs/master/sp_authorship/struct.InherentDataProvider.html)s

## The Structure of a Node

You may remember from the [hello-substrate recipe](../2-appetizers/1-hello-substrate.md) that a Substrate node has two parts. An outer part that is responsible for gossiping transactions and blocks, handling [rpc requests](./custom-rpc.md), and reaching consensus. And a runtime that is responsible for the business logic of the chain. This architecture diagram illustrates the distinction.

![Substrate Architecture Diagram](../img/substrate-architecture.png)

In principle the consensus engine, part of the outer node, is agnostic over the runtime that is used with it. But in practice, most consensus engines will require the runtime to provide certain [runtime APIs](./runtime-api.md) that effect the engine. For example Aura, and Babe, query the runtime for the set of validators. A more real-world PoW consensus would query the runtime for the bock difficulty. Luckily this simple PoW engine really is agnostic over the runtime, and thus we will use the familiar `super-runtime`.


## Proof of Work Algorithms

Proof of work is not a single consensus algorithm. Rather it is a class of Algorithms represented by the https://substrate.dev/rustdocs/master/sc_consensus_pow/trait.PowAlgorithm.html trait. Before we can build a PoW node we must specify a concrete PoW algorithm by implementing this trait.

```rust, ignore
//TODO copy struct definition code here
```

We will use the [sha3 hashing algorithm](https://en.wikipedia.org/wiki/SHA-3) which we have indicated in the name of our struct. Because this is a _minimal_ PoW algorithm, our struct can also be quite simple. In fact, it is a [unit struct](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html). A more complex PoW algorithm that interfaces with the runtime, would need to hold a reference to the client. An example of this (on an older Substrate codebase) can be seen in [Kulupu](https://github.com/kulupu/kulupu/)'s [RandomXAlgorithm](https://github.com/kulupu/kulupu/blob/3500b7f62fdf90be7608b2d813735a063ad1c458/pow/src/lib.rs#L137-L145).

## The Service Builder

talk about builder pattern
link to https://substrate.dev/rustdocs/master/sc_service/struct.ServiceBuilder.html

structure of service.rs
	macro
	new full
	new light

## Chain Spec

All of the node's in the recipes have a `chain_spec.rs` file, and they mostly look the same. `basic-pow`'s chain spec will also be familiar, but it is shorter and simpler. There are a few specific differences worth observing.

We don't need the help function
```rust, ignore
/// Taken from the super-runtime chain_spec.rs
/// Helper function to generate session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (BabeId, GrandpaId)
```

We don't provide any initial authorities
