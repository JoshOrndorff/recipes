# Kitchen Organization

Now that your kitchen is well-equipped with all the right tools (bowls, knives, Rust compiler, etc), let's take a look at how it is organized.

## Structure of a Substrate Node

It is useful to recognize that [coding is all about abstraction](https://youtu.be/05H4YsyPA-U?t=1789).

To understand how the code in this repository is organized, let's first take a look at how a Substrate node is constructed. Each node has many components that manage things like queueing transaction, communicating over a P2P network, reaching consensus on the state of the blockchain, and the chain's actual runtime logic. Each aspect of the node is interesting in its own right, and the runtime is particularly interesting because it contains the business logic (aka "state transition function") that codifies the chain's functionality.

Much, but not all, of the Recipes focuses on writing runtimes with FRAME, Parity's Framework for composing runtimes from individual building blocks called Pallets. Runtimes built with FRAME typically contain several such pallets. The kitchen node you built previously follows this paradigm.

![Substrate Architecture Diagram](../img/substrate-architecture.png)

## The Directories in our Kitchen

There are five primary directories in this repository:

* **Text**: Source of [the book](https://substrate.dev/recipes) written in markdown. This is what you're reading right now.
* **Pallets**: Pallets for use in FRAME-based runtimes.
* **Runtimes**: Runtimes for use in Substrate nodes.
* **Consensus**: Consensus engines for use in Substrate nodes.
* **Nodes**: Complete Substrate nodes ready to run.

Exploring those directories reveals a tree that looks like this
```
recipes
|
+-- text
|
+-- consensus
  |
  +-- shaw3pow
|
+-- nodes
	|
	+-- kitchen-node    <-- You built this previously
	|
	+-- rpc-node
|
+-- runtimes
	|
	+-- api-runtime
	|
	+-- super-runtime    <-- You built this too (it is part of the kitchen-node)
	|
	+-- weight-fee-runtime
	|
	+ ...
|
+-- pallets
	|
	+-- adding-machine    <-- You built this too (it is part of super-runtime)
	|
	+-- basic-token        <-- You built this too (it is part of super-runtime)
	|
	+ ...
	|
	+-- weights
```

## Inside the Kitchen Node

Let us take a deeper look at the [Kitchen Node](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/kitchen-node).

Looking inside the Kitchen Node's `Cargo.toml` file we see that it has many dependencies. Most of them come from Substrate itself. Indeed most parts of this Kitchen Node are not unique or specialized, and Substrate offers robust implementations that we can use. The runtime does not come from Substrate. Rather, we use our super-runtime which is in the `runtimes` folder.

**`nodes/kitchen-node/Cargo.toml`**
```TOML
# This node is compatible with any of the runtimes below
# ---
# Common runtime configured with most Recipes pallets.
runtime = { package = "super-runtime", path = "../../runtimes/super-runtime" }

# Runtime with custom weight and fee calculation.
# runtime = { package = "weight-fee-runtime", path = "../../runtimes/weight-fee-runtime"}

# Runtime with off-chain worker enabled.
# To use this runtime, compile the node with `ocw` feature enabled,
#   `cargo build --release --features ocw`.
# runtime = { package = "ocw-runtime", path = "../../runtimes/ocw-runtime" }

# Runtime with custom runtime-api (custom API only used in rpc-node)
# runtime = { package = "api-runtime", path = "../../runtimes/api-runtime" }
# ---
```

The commented lines, quoted above, show that the Super Runtime is not the only runtime we could have chosen. We could also use the Weight-Fee runtime, and I encourage you to try that experiment (remember, instructions to re-compile the node are in the previous section).

Every node must have a runtime. You may confirm that by looking at the `Cago.toml` files of the other nodes included in our kitchen.


## Inside the Super Runtime

Having seen that the Kitchen Node depends on a runtime, let us now look deeper at the [Super Runtime](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/super-runtime).

**`runtimes/super-runtime/Cargo.toml`**
```TOML
# -- snip --

# Substrate Pallets
balances = { package = 'pallet-balances', , ... }
transaction-payment = { package = 'pallet-transaction-payment', ,... }
# Recipe Pallets
adding-machine = { path = "../../pallets/adding-machine", default-features = false }
basic-token = { path = "../../pallets/basic-token", default-features = false }
```

Here we see that the runtime depends on many pallets. Some of these pallets come from Substrate itself. Indeed, Substrate offers a rich collection of commonly used pallets which you may use in your own runtimes. This runtime also contains several custom pallets that are written right here in our Kitchen.

## Common Patterns

We will not yet look closely at individual Pallets. We will begin that endeavor in the next chapter -- Appetizers.

We've just observed the general pattern used throughout the recipes. From the inside out, we see a piece of pallet code stored in `pallets/<pallet-name>/src/lib.rs`. The pallet is then included into a runtime by adding its name and relative path in `runtimes/<runtime-name>/Cargo.toml`. That runtime is then installed in a node by adding its name and relative path in `nodes/<node-name>/Cargo.toml`. Of course adding pallets and runtimes also requires changing actual _code_ as well. We will cover those details in due course. For now we're just focusing on macroscopic relationships between the parts of a Substrate node.

Some recipes explore aspects of Blockchain development that are outside of the runtime. Looking back to our node architecture at the beginning of this section, you can imagine that changing a node's RPC or Consensus would be conceptually similar to changing its runtime.

## Additional Resources

Substrate Developer Hub offers tutorials that go into more depth about writing pallets and including them in runtimes. If you desire, you may read them as well.

* [Creating an External Pallet](https://substrate.dev/docs/en/next/tutorials/creating-a-runtime-module)
* [Adding a Pallet to Your Runtime Tutorial](https://substrate.dev/docs/en/next/tutorials/adding-a-module-to-your-runtime)

# Let's Get Cooking!

When you're ready, we can begin by cooking some appetizer pallets.
