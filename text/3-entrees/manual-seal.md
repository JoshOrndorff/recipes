# Manual Seal

`nodes/manual-seal`
[
	![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)
](https://playground-staging.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fnodes%2Fmanual-seal%2Fsrc%2Flib.rs)
[
	![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)
](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/manual-seal/src/lib.rs)

This recipe demonstrates a Substrate node using the
[Manual Seal consensus](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_consensus_manual_seal/index.html). Unlike the
other consensus engines included with Substrate, manual seal does not create blocks on a regular
basis. Rather, it waits for an RPC call telling to create a block.

## Using Manual Seal

Before we explore the code, let's begin by seeing how to use the manual-seal node. Build and start
the node in the usual way.

```bash
cargo build --release -p manual-seal
./target/release/manual-seal
```

## Manually Sealing Blocks

Once your node is running, you will see that it just sits there idly. It will accept transactions to
the pool, but it will not author blocks on its own. In manual seal, the node does not author a block
until we explicitly tell it to. We can tell it to author a block by calling the `engine_createBlock`
RPC.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_createBlock",
      "params": [true, false, null]
    }'
```

This call takes three parameters, each of which are worth exploring.

### Create Empty

`create_empty` is a Boolean value indicating whether empty blocks may be created. Setting
`create-empty` to true does not mean that an empty block will necessarily be created. Rather it
means that the engine should go ahead creating a block even if no transaction are present. If
transactions are present in the queue, they will be included regardless of `create_empty`'s value.'

### Finalize

`finalize` is a Boolean indicating whether the block (and its ancestors, recursively) should be
finalized after creation. Manually controlling finality is interesting, but also dangerous. If you
attempt to author and finalize a block that does not build on the best finalized chain, the block
will not be imported. If you finalize one block in one node, and a conflicting block in another
node, you will cause a safety violation when the nodes synchronize.

### Parent Hash

`parent_hash` is an optional hash of a block to use as a parent. To set the parent, use the format
`"0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764"`. To omit the parent, use
`null`. When the parent is omitted the block is built on the current best block. Manually specifying
the parent is useful for constructing fork scenarios and demonstrating chain reorganizations.

## Manually Finalizing Blocks

In addition to finalizing blocks while creating them, they can be finalized later by using the
second provided RPC call, `engine_finalizeBlock`.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_finalizeBlock",
      "params": ["0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764", null]
    }'
```

The two parameters are:

-   The hash of the block to finalize.
-   A Justification. TODO what is the justification and why might I want to use it?

## Building the Service

So far we've learned how to use the manual seal node and why it might be useful. Let's now turn our
attention to how the service is built in the nodes `src/service.rs` file.

### The Import Queue

We begin by creating a manual-seal import queue. This process is identical to creating the import
queue used in the [Kitchen Node](./kitchen-node.md). It is also similar to, but simpler than, the
[basic-pow](./basic-pow.md) import queue.

```rust, ignore
.with_import_queue(|_config, client, _select_chain, _transaction_pool| {
	Ok(sc_consensus_manual_seal::import_queue::<_, sc_client_db::Backend<_>>(Box::new(client)))
})?;
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

Because the return type of this function contains `impl AbstractService`, Rust's typechecker is
unable to infer the concrete type. We give it a hand by calling `new_full` at the end, but don't
worry, this code will never actually be executed. `unimplemented!` will panic first.

### The Manual Seal RPC

Because the node runs in manual seal mode, we need to wire up the RPC commands that we explored
earlier. This process is nearly identical to those described in the
[custom rpc recipe](./custom-rpc.md).

As prep work, we make a type alias,

```rust, ignore
type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;
```

Next we create a channel over which the rpc handler and the authorship task can communicate with one
another. The RPC handler will send messages asking to create or finalize a block and the import
queue will receive the message and do so.

```rust, ignore
// channel for the rpc handler to communicate with the authorship task.
let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);
```

```rust, ignore
let service = builder
	// manual-seal relies on receiving sealing requests aka EngineCommands over rpc.
	.with_rpc_extensions(|_| -> Result<RpcExtension, _> {
		let mut io = jsonrpc_core::IoHandler::default();
		io.extend_with(
			// We provide the rpc handler with the sending end of the channel to allow the rpc
			// send EngineCommands to the background block authorship task.
			rpc::ManualSealApi::to_delegate(rpc::ManualSeal::new(command_sink)),
		);
		Ok(io)
	})?
	.build()?;
```

### The Authorship Task

As with every authoring engine, manual seal needs to be run as an `async` authoring tasks. Here we
provide the receiving end of the channel we created earlier.

```rust, ignore
// Background authorship future.
let authorship_future = manual_seal::run_manual_seal(
		Box::new(service.client()),
		proposer,
		service.client().clone(),
		service.transaction_pool().pool().clone(),
		commands_stream,
		service.select_chain().unwrap(),
		inherent_data_providers
	);
```

With the future created, we can now kick it off using the service's
[`spawn_essential_task` method](https://substrate.dev/rustdocs/v2.0.0-rc4/sc_service/struct.Service.html#method.spawn_essential_task).

```rust, ignore
// we spawn the future on a background thread managed by service.
service.spawn_essential_task_handle().spawn_blocking("manual-seal", authorship_future);
```

## Combining Instant Seal with Manual Seal

It is possible to combine the manual seal of the node we built above with the functionality of the
[Kitchen Node's](./kitchen-node.md) instant seal to get the best of both worlds. This configuration
may be desirable in development and testing environments. We can use the normal behavior of instant
seal to create blocks any time a transaction is imported into the pool. On the other hand we can
move forward in block number by instantly sealing empty blocks. The functionality may be familiar to
developers of Ethereum smart contracts that have used `ganache-cli`.

### Implementation

In the same directory for the manual seal node is a file called `combined_service.rs`. This file
contains modified code of the `service.rs` file we just looked at in the section above. Some modification have been made. These modifications are
numbered and begin at line 85 in the source.

```rust, ignore
let pool = service.transaction_pool().pool().clone();
```

The first step is to create an instance of a transaction pool that will be shared between the
`pool_stream` which receives events whenever a new transaction is imported and the service builder.

```rust, ignore
let pool_stream = pool
	.validated_pool()
	.import_notification_stream()
	.map(|_| {
		// Every new block create an `EngineCommand` that will seal a new block.
		rpc::EngineCommand::SealNewBlock {
			create_empty: false,
			finalize: false,
			parent_hash: None,
			sender: None,
		}
	});
```

Next we implement the instant seal just as it's implemented under the covers in the call to
`run_instant_seal`. Namely, we make sure that any new notifications we will submit an RPC
`EngineCommand` to seal a new block.

```rust, ignore
let combined_stream = futures::stream::select(commands_stream, pool_stream);
```

We combine the futures using the `select` utility which will receive events from either one of the
streams we pass to it. In this case, we're passing all notifications received from the manual seal
stream and the instant seal stream together.

```rust, ignore
let authorship_future = manual_seal::run_manual_seal(
	Box::new(service.client()),
	proposer,
	service.client(), // 4) vvvvv
	pool,             // <- Use the same pool that we used to get `pool_stream`.
	combined_stream,  // <- Here we place the combined streams.
	service.select_chain().unwrap(),
	inherent_data_providers,
);
```

Finally we initialize the `authorship_future` with the combined streams.

In order to run this variant of the node you will need to uncomment two lines and rebuild the node.
In `command.rs` comment the line that reads `use crate::service;` and uncomment
`use crate::combined_service as service;`. In `main.rs` comment `mod service;` and uncomment
`mod combined_service'`. Now you can rebuild the node and test out that it will seal blocks using
the manual method and the instant method together.
