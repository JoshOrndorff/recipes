# Runtime APIs
*[`kitchen/pallets/sum-storage`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/sum-storage)*
*[`kitchen/runtimes/api-runtime`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/runtime/api-runtime)*

Each Substrate node contains a runtime. The runtime contains the business logic of the chain. It defines what transactions are valid and invalid and determines how the chain's state changes in response to transactions. The runtime is compiled to Wasm to facilitate runtime upgrades. The "outer node", everything other than the runtime, does not compile to Wasm, only to native. The outer node is responsible for handling peer discovery, transaction pooling, block and transaction gossiping, consensus, and answering RPC calls from the outside world. While performing these tasks, the outer node sometimes needs to query the runtime for information, or provide information to the runtime. A Runtime API facilitates this communication between the outer node and the runtime. In this recipe, we will write our own minimal runtime API.

## Our Example
For this example, we will write a pallet called `sum-storage` with two storage items, both `u32`s.

```rust
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Thing1 get(fn thing1): Option<u32>;
		Thing2 get(fn thing2): Option<u32>;
	}
}
```

Substrate already comes with a runtime API for querying storage values, which is why we can easily query our two storage values from a front-end. In this example we imagine that the outer node is interested in knowing the _sum_ of the two values, rather than either individual value. Our runtime API will provide a way for the outer node to query the inner node for this sum. Before we define the actual runtime API, let's write a public helper function to do the summing.

```rust
impl<T: Trait> Module<T> {
	pub fn get_sum() -> u32 {
		Thing1::get() + Thing2::get()
	}
}
```

So far, nothing we've done is specific to runtime APIs. In the coming sections, we will use this helper function in our runtime API's implementation.

## Defining the API
The first step in adding a runtime API to your runtime is defining its interface using a rust trait. This is done in the `sum-storage/rpc/runtime-api/src/lib.rs` file. This file can live anywhere you like, but because it defines an API that is closely related to a particular pallet, it makes sense to include the API definition in the pallet's directory.

> The `rpc` in the path is not relevant in _this_ recipe, but it is explained in the recipe on [custom RPCs](./custom-rpc.md).

The code to define the API is quite simple, and looks almost like any old rust trait. The one addition is that it must be placed in the [`decl_runtime_apis!` macro](https://substrate.dev/rustdocs/master/sp_api/macro.decl_runtime_apis.html). This macro allows the outer node to query the runtime API at specific blocks. Although this runtime API only provides a single function, you may write as many as you like.

```rust
sp_api::decl_runtime_apis! {
	pub trait SumStorageApi {
		fn get_sum() -> u32;
	}
}
```

## Implementing the API
With our pallet written and our runtime API defined, we may now implement the API. This happens in the main runtime aggregation file. In our case we've provided the `api-runtime` in `kitchen/runtimes/api-runtime/src/lib.rs`.

As with defining the API, implementing a runtime API looks similar to implementing any old rust trait with the exception that the implementation must go inside of the [`impl_runtime_apis!` macro](https://substrate.dev/rustdocs/master/sp_api/macro.impl_runtime_apis.html). If you've started by copying an existing runtime like the [node template](https://github.com/substrate-developer-hub/substrate-node-template/) you likely already have this block. We will add an implementation for our API to the existing block. Our implementation is straight-forward as it merely calls the pallet's helper function that we wrote previously.

```rust
impl_runtime_apis! {
  // --snip--

  impl sum_storage_rpc_runtime_api::SumStorageApi<Block> for Runtime {
		fn get_sum() -> u32 {
			SumStorage::get_sum()
		}
	}
}
```

You may be wondering about the `Block` type parameter which is present here, but not in our definition. This type parameter is added by the macros along with a few other features. All runtime APIs have the type parameter to facilitate querying the runtime at arbitrary blocks. Read more about this in the docs for [`impl_runtime_apis!`](https://substrate.dev/rustdocs/master/sp_api/macro.impl_runtime_apis.html).

## Calling the Runtime API
We've now successfully added a runtime API to our runtime. The outer node can now call this API to query the runtime for the sum of two storage values. Given a reference to a ['client'](https://substrate.dev/rustdocs/master/sc_client/struct.Client.html) we can make the call like this.

```rust
let sum_at_block_fifty = client.runtime_api().get_sum(&50);
```
Calling this runtime API is described with much more detail and context in the recipe on [custom RPCs](./custom-rpc.md).
