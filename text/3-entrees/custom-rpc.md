# Custom RPCs

_[`nodes/rpc-node`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/rpc-node)_
_[`runtime/api-runtime`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/api-runtime)_

Remote Procedure Calls, or RPCs, are a way for an external program (eg. a frontend) to communicate
with a Substrate node. They are used for checking storage values, submitting transactions, and
querying the current consensus authorities. Substrate comes with several
[default RPCs](https://polkadot.js.org/api/substrate/rpc.html). In many cases it is useful to add
custom RPCs to your node. In this recipe, we will add two custom RPCs to our node, one of which
calls into a [custom runtime API](./runtime-api.md).

## Defining an RPC

Every RPC that the node will use must be defined in a trait. We'll begin by defining a simple RPC
called "silly rpc" which just returns constant integers. A Hello world of sorts. In the
`nodes/rpc-node/src/silly_rpc.rs` file, we define a basic rpc as

```rust
#[rpc]
pub trait SillyRpc {
	#[rpc(name = "silly_seven")]
	fn silly_7(&self) -> Result<u64>;

	#[rpc(name = "silly_double")]
	fn silly_double(&self, val: u64) -> Result<u64>;
}
```

This definition defines two RPC methods called `hello_five` and `hello_seven`. Each RPC method must
take a `&self` reference and must return a `Result`. Next, we define a struct that implements this
trait.

```rust
pub struct Silly;

impl SillyRpc for Silly {
	fn silly_7(&self) -> Result<u64> {
		Ok(7)
	}

	fn silly_double(&self, val: u64) -> Result<u64> {
		Ok(2 * val)
	}
}
```

Finally, to make the contents of this new file usable, we need to add a line in our `main.rs`.

```rust
mod silly_rpc;
```

## Including the RPC

With our RPC written, we're ready to install it on our node. We begin with a few dependencies in our
`rpc-node`'s `Cargo.toml`.

```toml
jsonrpc-core = "14.0.3"
jsonrpc-core-client = "14.0.3"
jsonrpc-derive = "14.0.3"
sc-rpc = '2.0.0-rc2'
```

Next, in our `rpc-node`'s `service.rs` file, we extend the service with our RPC. We've chosen to
install this RPC for full nodes, so we've included the code in the `new_full_start!` macro. You
could also install the RPC on a light client by making the corresponding changes to `new_light`.

The first change to this macro is a simple type definition

```rust
type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;
```

Then, once you've called the service builder, you can extend it with an RPC by using its
`with_rpc_extensions` method as follows.

```rust
.with_rpc_extensions(|builder| -> Result<RpcExtension, _> {
	// Make an io handler to be extended with individual RPCs
	let mut io = jsonrpc_core::IoHandler::default();

	// Use the fully qualified name starting from `crate` because we're in macro_rules!
	io.extend_with(crate::silly_rpc::SillyRpc::to_delegate(crate::silly_rpc::Silly{}));

	// --snip--

	Ok(io)
})
```

## Calling the RPC

Once your node is running, you can test the RPC by calling it with any client that speaks json RPC.
One widely available option is `curl`.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"silly_seven",
      "params": []
    }'
```

To which the RPC responds

```
{"jsonrpc":"2.0","result":7,"id":1}
```

You may have noticed that our second RPC takes a parameter, the value to double. You can supply this
parameter by including its in the `params` list. For example:

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"silly_double",
      "params": [7]
    }'
```

To which the RPC responds with the doubled parameter

```
{"jsonrpc":"2.0","result":14,"id":1}
```

## RPC to Call a Runtime API

The silly RPC demonstrates the fundamentals of working with RPCs in Substrate. Nonetheless, most
RPCs will go beyond what we've learned so far, and actually interact with other parts of the node.
In this second example, we will include an RPC that calls into the `sum-storage` runtime API from
the [runtime API recipe](./runtime-api.md). While it isn't strictly necessary to understand what the
runtime API does, reading that recipe may provide helpful context.

Because this RPC's behavior is closely related to a specific pallet, we've chosen to define the RPC
in the pallet's directory. In this case the RPC is defined in `pallets/sum-storage/rpc`. So rather
than using the `mod` keyword as we did before, we must include this RPC definition in the node's
`Cargo.toml` file.

```toml
sum-storage-rpc = { path = "../../pallets/sum-storage/rpc" }
```

Defining the RPC interface is similar to before, but there are a few differences worth noting.
First, the struct that implements the RPC needs a reference to the `client`. This is necessary so we
can actually call into the runtime. Second the struct is generic over the `BlockHash` type. This is
because it will call a runtime API, and runtime APIs must always be called at a specific block.

```rust
#[rpc]
pub trait SumStorageApi<BlockHash> {
	#[rpc(name = "sumStorage_getSum")]
	fn get_sum(
		&self,
		at: Option<BlockHash>
	) -> Result<u32>;
}

/// A struct that implements the `SumStorageApi`.
pub struct SumStorage<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> SumStorage<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}
```

The RPC's implementation is also similar to before. The additional syntax here is related to calling
the runtime at a specific block, as well as ensuring that the runtime we're calling actually has the
correct runtime API available.

```rust
impl<C, Block> SumStorageApi<<Block as BlockT>::Hash>
	for SumStorage<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi,
	C: HeaderBackend<Block>,
	C::Api: SumStorageRuntimeApi<Block>,
{
	fn get_sum(
		&self,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<u32> {

		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

		let runtime_api_result = api.get_sum(&at);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
```

Finally, to install this RPC on in our service, we expand the existing `with_rpc_extensions` call to

```rust
.with_rpc_extensions(|builder| -> Result<RpcExtension, _> {
	// Make an io handler to be extended with individual RPCs
	let mut io = jsonrpc_core::IoHandler::default();

	// Add the first rpc extension
	io.extend_with(crate::silly_rpc::SillyRpc::to_delegate(crate::silly_rpc::Silly{}));

	// Add the second RPC extension
	// Because this one calls a Runtime API it needs a reference to the client.
	io.extend_with(sum_storage_rpc::SumStorageApi::to_delegate(sum_storage_rpc::SumStorage::new(builder.client().clone())));

	Ok(io)
})?;
```

## Optional RPC Parameters

This RPC takes a parameter ,`at`, whose type is `Option<_>`. We may call this RPC by omitting the
optional parameter entirely. In this case the implementation provides a default value of the best
block.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"sumStorage_getSum",
      "params": []
    }'
```

We may also call the RPC by providing a block hash. One easy way to get a block hash to test this
call is by copying it from the logs of a running node.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"sumStorage_getSum",
      "params": ["0x87b2e4b93e74d2f06a0bde8de78c9e2a9823ce559eb5e3c4710de40a1c1071ac"]
    }'
```

As an exercise, change the storage values and confirm that the RPC provides the correct
updated sum. Then call the RPC at an old block and confirm you get the old sum.

## Polkadot JS API

Many frontends interact with Substrate nodes through Polkadot JS API. While the Recipes does not
strive to document that project, we have included a snippet of javascript for interacting with these
custom RPCs in the `nodes/rpc-node/js` directory.
