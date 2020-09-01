# Kitchen Node (Instant Seal)

`nodes/kitchen-node`
[
	![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)
](https://playground-staging.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fnodes%2Fkitchen-node%2Fsrc%2Flib.rs)
[
	![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)
](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/kitchen-node/src/lib.rs)

This recipe demonstrates a general purpose Substrate node that supports most of the recipes'
runtimes, and uses
[Instant Seal consensus](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_consensus_manual_seal/index.html).

The kitchen node serves as the first point of entry for most aspiring chefs when they first
encounter the recipes. By default it builds with the super-runtime, but it can be used with most of
the runtimes in the recipes. Changing the runtime is described below. It features the instant seal
consensus which is perfect for testing and iterating on a runtime.

## Installing a Runtime

### Cargo Dependency

The `Cargo.toml` file specifies the runtime as a dependency. The file imports the super-runtime, and
has dependencies on other runtimes commented out.

```toml
# Common runtime configured with most Recipes pallets.
runtime = { package = "super-runtime", path = "../../runtimes/super-runtime" }

# Runtime with custom weight and fee calculation.
# runtime = { package = "weight-fee-runtime", path = "../../runtimes/weight-fee-runtime"}

# Runtime with off-chain worker enabled.
# To use this runtime, compile the node with `ocw` feature enabled,
#   `cargo build --release --features ocw`.
# runtime = { package = "ocw-runtime", path = "../../runtimes/ocw-runtime" }

# Runtime with custom runtime-api (custom API only used in rpc-node)
#runtime = { package = "api-runtime", path = "../../runtimes/api-runtime" }
```

Installing a different runtime in the node is just a matter of commenting out the super-runtime
line, and enabling another one. Try the weight-fee runtime for example. Of course cargo will
complain if you try to import two crates under the name `runtime`.

It is worth noting that this node does not work with _all_ of the recipes' runtimes. In particular,
it is not compatible with the babe-grandpa runtime. That runtime uses the babe pallet which requires
a node that will include a special
[`PreRuntime` `DigestItem`](https://substrate.dev/rustdocs/v2.0.0-rc6/sp_runtime/enum.DigestItem.html#variant.PreRuntime).

### Building a Service with the Runtime

With a runtime of our choosing listed among our dependencies, we can wiring the nodes [`Service`](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_service/index.html), the part of the node that coordinates communication between all other parts.

We begin by invoking the
[`native_executor_instance!` macro](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_executor/macro.native_executor_instance.html).
This creates an executor which is responsible for executing transactions in the runtime and
determining whether to run the native or Wasm version of the runtime.

```rust_ignore
native_executor_instance!(
	pub Executor,
	runtime::api::dispatch,
	runtime::native_version,
);
```

The remainder of the file will create the individual components of the node and connect them together. Most of this code is boilerplate taken from the Substrate Node Template. We will focus specifically on the unique consensus engine used here.

## Instant Seal Consensus

The instant seal consensus engine, and its cousin the manual seal consensus engine, are both
included in the same
[`sc-consensus-manual-seal` crate](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_consensus_manual_seal/index.html). Instant seal
simply authors a new block whenever a new transaction is available in the queue. This is similar to
[Truffle Suite's Ganache](https://www.trufflesuite.com/ganache) in the Ethereum ecosystem, but
without the UI.

### The Cargo Dependencies

Installing the instant seal engine has three dependencies whereas the runtime had only one.

```toml
sc-consensus = '0.8.0-rc4'
sc-consensus-manual-seal = '0.8.0-rc4'
sp-consensus = '0.8.0-rc4'
```

### The Import Queue

We begin in `new_partial` by creating a manual-seal import queue. Both instant seal and manual seal use the same import queue. This process is similar to, but simpler than, the
[basic-pow](./basic-pow.md) import queue.

```rust, ignore
let import_queue = sc_consensus_manual_seal::import_queue(
	Box::new(client.clone()),
	&task_manager.spawn_handle(),
	config.prometheus_registry(),
);
```

### The Proposer

Now we pick up in the `new_full` function. All of the code in this portion is executed only if the node is an authority. Create a
[`Proposer`](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_basic_authorship/struct.Proposer.html) which will be
responsible for creating proposing blocks in the chain.

```rust, ignore
let proposer = sc_basic_authorship::ProposerFactory::new(
	service.client().clone(),
	service.transaction_pool(),
);
```

### The Authorship Task

As with every authoring engine, instant seal needs to be run as an `async` authoring task.

```rust, ignore
let authorship_future = sc_consensus_manual_seal::run_instant_seal(
	Box::new(client.clone()),
	proposer,
	client,
	transaction_pool.pool().clone(),
	select_chain,
	inherent_data_providers,
);
```

With the future created, we can now kick it off using the [`TaskManager`](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_service/struct.TaskManager.html)'s
[`spawn_essential_handle` method](https://substrate.dev/rustdocs/v2.0.0-rc6/sc_service/struct.TaskManager.html#method.spawn_essential_handle).

```rust, ignore
task_manager.spawn_essential_handle().spawn_blocking("instant-seal", authorship_future);
```

## Manual Seal Consensus

The instant seal consensus engine used in this node is built on top of a similar manual seal engine. Manual seal listens for commands to come over the RPC instructing it to author blocks. To see this engine in use, check out the [RPC node recipe](./custom-rpc.md).
