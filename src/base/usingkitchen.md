# Using the Kitchen

Let us take a deeper look at the Kitchen Node. Inside

**`kitchen/node/Cargo.toml`**

```TOML
# -- snip --
runtime = { package = "super-runtime", path = "../runtimes/super-runtime" }
# -- snip --
```

You see `node` is bringing in the `runtime` modules in, and use it to build up the node service in

**`kitchen/node/src/service.rs`**

```Rust
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

The `runtime` folder contains two folders, `super-genesis` for specifying how the first block on the blockchain (genesis block) is being produced, and `super-runtime` for specifying how the node runtime behaves. Let us focus on one module `simple-event` in the runtime. In

**`kitchen/runtimes/super-runtime/Cargo.toml`**

```TOML
# -- snip --

# `simple-event` module is specified as a relative path to the `modules` folder
[dependencies]
simple-event = { package = "simple-event", path = "../../modules/simple-event", default_features = false }

# -- snip --
```

This is where the node runtime includes additional module `simple-event` written in this recipe. The module is then included into the runtime by:

**`kitchen/runtimes/super-runtime/src/lib.rs`**

```Rust
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

Finally, you can see how the `simple-event` module is specified in `kitchen/modules/simple-event/src/lib.rs`.

This is the general pattern used throughout these recipes. We first talk about a new piece of module code stored in `kitchen/modules/<module-name>/src/lib.rs`. The module is then included into the runtime by adding the module name and relative path in `kitchen/runtimes/super-runtime/Cargo.toml` (if not yet added) and updating `kitchen/runtimes/super-runtime/src/lib.rs`.

## Learn More

In fact, the Kitchen Node and runtime structure has been refactored to cater for the recipe purpose. If you are interested to learn more about how to include your own module in a node runtime, we recommend you to go through the following two tutorials.

* [Writing a Runtime Module in its Own Crate Tutorial](https://substrate.dev/docs/en/tutorials/creating-a-runtime-module)
* [Adding `Contract` Module to Your Runtime Tutorial](https://substrate.dev/docs/en/tutorials/adding-a-module-to-your-runtime)