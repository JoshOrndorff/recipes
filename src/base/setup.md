# Installation and Running Recipe Kitchen Node

## Setup

If you do not have Subtrate development environment setup on your machine, please install it as followed.

### For Linux / macOS

```bash
# Setup Rust and Substrate
curl https://getsubstrate.io -sSf | bash
```

### For Windows

Refer to our [Substrate Installation on Windows](https://substrate.dev/docs/en/next/getting-started#getting-started-on-windows).

## Running the Recipe Kitchen Node

To interact with the code in this recipe, `git clone` the source repository. We also want to kick-start the node compilation as it may take about 30 minutes to complete depending on your hardware.

```bash
git clone https://github.com/substrate-developer-hub/recipes.git
cd recipes/kitchen/node

# This step takes a while to complete
cargo build --release
```

Here, `recipes/kitchen` folder contains all the code necessary to run a Substrate node. Let us call it the Recipe Kitchen Node. There are three folders inside:

  * `node` - contains the code to start the Recipe Kitchen Node.
  * `runtimes` - contains the runtime of the Recipe Kitchen Node.
  * `modules` - the runtime includes multiple modules. Each module gives the runtime a new set of functionality. Most of the recipe module code we discuss afterwards is stored in this folder

> **Notes**
>
> Refer to the following sections to:
>
>  * Learn more about [Substrate runtime](https://substrate.dev/docs/en/runtime/architecture-of-a-runtime)
>  * Learn more about [Substrate modules](https://substrate.dev/docs/en/runtime/substrate-runtime-module-library)

Once the compilation is completed, you can first purge any existing blockchain data (useful to start your node from a clean state in future) and then start the node.

```bash
# Inside `recipes/kitchen/node` folder

# Purge any existing blockchain data. Enter `y` upon prompt.
./target/release/kitchen-node purge-chain --dev

# Start the Recipe Kitchen Node
./target/release/kitchen-node --dev
```

## Interact with your Node

You should see blocks are being created on the console. You can now use our [Polkadot-JS Apps to interact with your locally running node](https://polkadot.js.org/apps/#/explorer?rpc=ws://127.0.0.1:9944). You will be mainly using the **Chain state** tab to query the blockchain status and **Extrinsics** to send transactions to the blockchain.

Congratulation on running your Recipe Kitchen Node and able to interact with it!

## Understanding the Recipe Kitchen Node

Let us take a deeper look at the Recipe Kitchen Node. Inside

**`kitchen/node/Cargo.toml`**

```TOML
# -- snip --
runtime = { package = "super-node-runtime", path = "../runtimes/super-node-runtime" }
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

The `runtime` folder contains two folders, `super-node-genesis` for specifying how the first block on the blockchain (genesis block) is being produced, and `super-node-runtime` for specifying how the node runtime behaves. Let us focus on one module `simple-event` in the runtime. In

**`kitchen/runtimes/super-node-runtime/Cargo.toml`**

```TOML
# -- snip --

# `simple-event` module is specified as a relative path to the `modules` folder
[dependencies]
simple-event = { package = "simple-event", path = "../../modules/simple-event", default_features = false }

# -- snip --
```

This is where the node runtime includes additional module `simple-event` written in this recipe. The module is then included into the runtime by:

**`kitchen/runtimes/super-node-runtime/src/lib.rs`**

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

This is the general pattern used throughout this recipe. We first talk about a new piece of module code stored in `kitchen/modules/<module-name>/src/lib.rs`. The module is then included into the runtime by adding the module name and relative path in `kitchen/runtimes/super-node-runtime/Cargo.toml` (if not yet added) and updating `kitchen/runtimes/super-node-runtime/src/lib.rs`.

## Learn More

In fact, the Recipe Kitchen Node and runtime structure has been refactored to cater for the recipe purpose. If you are interested to learn more about how to include your own module in a node runtime, we recommend you to go through the following two tutorials.

* [Writing a Runtime Module in its Own Crate Tutorial](https://substrate.dev/docs/en/tutorials/ creating-a-runtime-module)
* [Adding `Contract` Module to Your Runtime Tutorial](https://substrate.dev/docs/en/tutorials/adding-a-module-to-your-runtime)
