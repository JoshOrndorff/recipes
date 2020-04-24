# Manual Seal
*[`nodes/manual-seal`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/manual-seal)*

This recipe demonstrates a Substrate node using the [Manual Seal consensus](https://substrate.dev/rustdocs/master/sc_consensus_manual_seal/index.html). Unlike the other consensus engines included with Substrate, manual seal does not create blocks on a regular basis. Rather, it waits for an RPC call telling to create a block. This recipe also demonstrates the Instant Seal engine which creates a block as soon as a transaction is ready in the pool.

## Using Manual Seal

Before we explore the code, let's begin by seeing how to use the manual-seal node. Build and start the node in the usual way.

## Manually Sealing Blocks
Once your node is running, you will see that it just sits there idly. It will accept transactions to the pool, but it will not author blocks on its own. In manual seal, the node does not author a block until we explicitly tell it to. We can tell it to author a block by calling the `engine_createBlock` RPC.

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
`create_empty` is a Boolean value indicating whether empty blocks may be created. Setting `create-empty` to true does not mean that an empty block will necessarily be created. Rather it means that the engine should go ahead creating a block even if no transaction are present. If transactions are present in the queue, they will be included regardless of `create_empty`'s value.'

### Finalize
`finalize` is a Boolean indicating whether the block (and its ancestors, recursively) should be finalized after creation. Manually controlling finality is interesting, but also dangerous. If you attempt to author and finalize a block that does not build on the best finalized chain, the block will not be imported. If you finalize one block in one node, and a conflicting block in another node, you will cause a safety violation when the nodes synchronize.

### Parent Hash
`parent_hash` is an optional hash of a block to use as a parent. To set the parent, use the format `"0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764"`. To omit the parent, use `null`. When the parent is omitted the block is built on the current best block. Manually specifying the parent is useful for constructing fork scenarios and demonstrating chain reorganizations.

## Manually Finalizing Blocks
In addition to finalizing blocks while creating them, they can be finalized later by using the second provided RPC call, `engine_finalizeBlock`.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_finalizeBlock",
      "params": ["0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764", null]
    }'
```

The two parameters are:
* The hash of the block to finalize.
* A Justification. TODO what is the justification and why might I want to use it?

## Using Instant Seal

In addition to the manual seal mechanism we've explored so far, this node also provides an option to seal blocks instantly when transactions are received into the pool. To run the node in this mode use the `--instant-seal` flag like so.

```bash
./target/release/manual-seal --instant-seal
```

When running in this mode, there is no need (or ability) to issue the RPC commands provided by manual seal. Rather, blocks are instantly created and sealed when a transaction arrives in the pool.

This mode is extremely useful for testing runtime logic. It is equivalent to [ganache](https://www.trufflesuite.com/ganache) in the ethereum ecosystem.

## Building the Service

So far we've learned how to use the manual seal node and why it might be useful. Let's now turn our attention to how the service is built in the nodes `src/service.rs` file.

### The Import Queue

We begin by creating a manual-seal import queue. This process is similar, but simpler, to that described in the [basic-pow recipe](./basic-pow.md).

Although this `import_queue` function comes from the [`sc_consensus_manual_seal` crate](https://substrate.dev/rustdocs/master/sc_consensus_manual_seal/index.html), it contains the logic for both manual and instant seal, and this import queue can be used for both.

```rust, ignore

.with_import_queue(|_config, client, _select_chain, _transaction_pool| {
	Ok(sc_consensus_manual_seal::import_queue::<_, sc_client_db::Backend<_>>(Box::new(client)))
})?;
```

### The Manual Seal RPC

If the node is being run in manual seal mode, we need to write up the RPC commands that we explored earlier. This process is nearly identical to those described in the [custom rpc recipe](./custom-rpc.md).

As prep work, we make a type alias,
```rust, ignore
type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;
```

Then we implement the wiring. The very first task is to check whether we're running in instant seal mode or manual seal mode. If it is instant seal mode, then no RPC extensions are necessary.

```rust, ignore
// channel for the rpc handler to communicate with the authorship task.
let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);

let service = if instant_seal {
	builder.build()?
} else {
	builder
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
		.build()?
};
```

The command stream created at the beginning is the means by which the RPC handler communicates with the authorship task that we will create next.

### The Authorship Task

As with every authoring engine, manual and instant seal need to be run as async authoring tasks. Again we check whether this is in manual or instant seal mode and create the correct kind of future. Creating the two authorship tasks is nearly identical. The only difference is that the manual seal task requires the command stream we created earlier.

```rust, ignore
// Background authorship future.
let future = if instant_seal {
	log::info!("Running Instant Sealing Engine");
	Either::Right(manual_seal::run_instant_seal(
		Box::new(service.client()),
		proposer,
		service.client().clone(),
		service.transaction_pool().pool().clone(),
		service.select_chain().unwrap(),
		inherent_data_providers
	))
} else {
	log::info!("Running Manual Sealing Engine");
	Either::Left(manual_seal::run_manual_seal(
		Box::new(service.client()),
		proposer,
		service.client().clone(),
		service.transaction_pool().pool().clone(),
		commands_stream,
		service.select_chain().unwrap(),
		inherent_data_providers
	))
};
```

With the future created, we can now kick it off using the service's [`spawn_essential_task` method](https://substrate.dev/rustdocs/master/sc_service/struct.Service.html#method.spawn_essential_task).

```rust, ignore
service.spawn_essential_task(
	if instant_seal { "instant-seal" } else { "manual-seal" },
	future
);
```

### What about the Light Client?

We've chosen not to support the light client in this node because it will typically be used for learning, experimenting, and testing in a single-node environment. Instead we mark it as `unimplemented!`.

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

Because the return type of this function contains `impl AbstractService`, Rust's typechecker is unable to infer the concrete type. We give it a hand by calling `new_full` at the end, but don't worry, this code will never actually be executed. `unimplemented!` will panic first.
