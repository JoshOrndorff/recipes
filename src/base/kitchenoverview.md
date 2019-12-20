# Using the Kitchen

Let us take a deeper look at the [Kitchen Node](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/node). Inside

**`kitchen/node/Cargo.toml`**

```TOML
# -- snip --
runtime = { package = "super-runtime", path = "../runtimes/super-runtime" }
# -- snip --
```

You see `node` is bringing in the pallets in and using them to build up the node service in

**`kitchen/node/src/service.rs`**

```rust, ignore
// -- snip --
use runtime::{self, GenesisConfig, opaque::Block, RuntimeApi};
// -- snip --

macro_rules! new_full_start {
  // -- snip --
  let builder = substrate_service::ServiceBuilder::new_full::<
    runtime::opaque::Block, runtime::RuntimeApi, crate::service::Executor
  >($config)?
  // -- snip --
}
```

The `runtime` folder contains two folders for each runtime. Let's consider the super runtime as an example. `super-genesis` is for specifying how the first block on the blockchain (genesis block) is being produced, and `super-runtime` for specifying how the node runtime behaves. Now let us focus on one pallet `simple-event` in the runtime. In

**`kitchen/runtimes/super-runtime/Cargo.toml`**

```TOML
# -- snip --

# `simple-event` pallet is specified as a relative path to the `pallets` folder
[dependencies]
simple-event = { path = "../../pallets/simple-event", default_features = false }

# -- snip --
```

This is where the node runtime includes the pallet `simple-event` written here in the recipes.

**`kitchen/runtimes/super-runtime/src/lib.rs`**

```rust, ignore
// -- snip --
use simple_event;

// -- snip --
impl simple_event::Trait for Runtime {
  type Event = Event;
}

// -- snip --
construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    // -- snip --
    SingleValue: single_value::{Module, Call, Storage, Event<T>},
  }
);
```

Finally, you can see how the `simple-event` pallet is specified in `kitchen/pallets/simple-event/src/lib.rs`.

This is the general pattern used throughout these recipes. We first talk about a new piece of pallet code stored in `kitchen/pallets/<pallet-name>/src/lib.rs`. The pallet is then included into a runtime by adding its name and relative path in `kitchen/runtimes/<runtime-name>/Cargo.toml` (if not yet added) and updating `kitchen/runtimes/<runtime-name>/src/lib.rs`.

## Learn More

If you are interested to learn more about how to include your own pallets in a node runtime, we recommend you to go through the following two tutorials.

* [Creating an External Pallet](https://substrate.dev/docs/en/next/tutorials/creating-a-runtime-module)
* [Adding a Pallet to Your Runtime Tutorial](https://substrate.dev/docs/en/next/tutorials/adding-a-module-to-your-runtime)
