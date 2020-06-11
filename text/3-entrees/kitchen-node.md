# Kitchen Node (Instant Seal)

_[`nodes/kitchen-node`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/kitchen-node)_

This recipe demonstrates a general purpose Substrate node that supports most of the recipes'
runtimes, and uses
[Instant Seal consensus](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_consensus_manual_seal/index.html).

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
[`PreRuntime` `DigestItem`](https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/enum.DigestItem.html#variant.PreRuntime).

### Building a Service with the Runtime

With a runtime of our choosing listed among our dependencies, we can provide the runtime to the
[`ServiceBuilder`](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_service/struct.ServiceBuilder.html). The
`ServiceBuilder` is responsible for assembling all of the necessary pieces that a node will need,
and creating a [`Substrate Service`](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_service/struct.Service.html) which
will manage the communication between them.

We begin by invoking the
[`native_executor_instance!` macro](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_executor/macro.native_executor_instance.html).
This creates an executor which is responsible for executing transactions in the runtime and
determining whether to run the native or Wasm version of the runtime.

```rust_ignore
native_executor_instance!(
	pub Executor,
	runtime::api::dispatch,
	runtime::native_version,
);
```

Finally, we create a new `ServiceBuilder` for a full node. (The `$` in the syntax is because we are
in a [macro definition](https://doc.rust-lang.org/book/ch19-06-macros.html).

```rust, ignore
let builder = sc_service::ServiceBuilder::new_full::<
	runtime::opaque::Block, runtime::RuntimeApi, crate::service::Executor
>($config)?
// --snip--
```

## Instant Seal Consensus

The instant seal consensus engine, and its cousin the manual seal consensus engine, are both
included in the same
[`sc-consensus-manual-seal` crate](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_consensus_manual_seal/index.html).
The recipes has a recipe dedicated to using [manual seal](./manual-seal.md). Instant seal is a very
convenient tool for when you are developing or experimenting with a runtime. The consensus engine
simply authors a new block whenever a new transaction is available in the queue. This is similar to
[Truffle Suite's Ganache](https://www.trufflesuite.com/ganache) in the Ethereum ecosystem, but
without the UI.

### The Cargo Dependencies

Installing the instant seal engine has three dependencies whereas the runtime had only one.

```toml
sc-consensus = '0.8.0-rc3'
sc-consensus-manual-seal = '0.8.0-rc3'
sp-consensus = '0.8.0-rc3'
```

### The Proposer

We begin by creating a
[`Proposer`](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_basic_authorship/struct.Proposer.html) which will be
responsible for creating proposing blocks in the chain.

```rust, ignore
let proposer = sc_basic_authorship::ProposerFactory::new(
	service.client().clone(),
	service.transaction_pool(),
);
```

### The Import Queue

Next we make a manual-seal import queue. This process is identical to creating the import queue used
in the [Manual Seal Node](./manual-seal.md). It is also similar to, but simpler than, the
[basic-pow](./basic-pow.md) import queue.

```rust, ignore
.with_import_queue(|_config, client, _select_chain, _transaction_pool| {
	Ok(sc_consensus_manual_seal::import_queue::<_, sc_client_db::Backend<_>>(Box::new(client)))
})?;
```

### The Authorship Task

As with every authoring engine, instant seal needs to be run as an `async` authoring task.

```rust, ignore
let authorship_future = sc_consensus_manual_seal::run_instant_seal(
	Box::new(service.client()),
	proposer,
	service.client().clone(),
	service.transaction_pool().pool().clone(),
	service.select_chain().ok_or(ServiceError::SelectChainRequired)?,
	inherent_data_providers
);
```

With the future created, we can now kick it off using the service's
[`spawn_essential_task` method](https://substrate.dev/rustdocs/v2.0.0-rc3/sc_service/struct.Service.html#method.spawn_essential_task).

```rust, ignore
service.spawn_essential_task("instant-seal", authorship_future);
```

### What about the Light Client?

The light client is not yet supported in this node, but it likely will be in the future (See
[issue #238](https://github.com/substrate-developer-hub/recipes/pull/238).) Because it will
typically be used for learning, experimenting, and testing in a single-node environment this
restriction should not cause many problems.. Instead we mark it as `unimplemented!`.

```rust, ignore
/// Builds a new service for a light client.
pub fn new_light(_config: Configuration) -> Result<impl AbstractService, ServiceError>
{
	unimplemented!("No light client for manual seal");

	// This needs to be here or it won't compile.
	#[allow(unreachable_code)]
	new_full(_config, false)
}
```
