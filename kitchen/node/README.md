# Kitchen Node
This Substrate-based node does not contain its own runtime. Rather it imports another runtime (like the ones here in the kitchen) through it's `Cargo.toml` file, and wraps them with a standard blockchain chasis including:

* Babe block production
* Grandpa Finality
* A CLI interface
* An RPC compatible with Polkadot-js API

## Building

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Starting a Node
To start a dev node (after building) run

```bash
./target/release/kitchen-node purge-chain --dev -y
./target/release/kitchen-node --dev
```

There are many other ways to use this node which can be explored by running `kitchen-node --help` or reading general [Substrate Documentation](https://substrate.dev/).

## Swapping Runtimes
All runtimes in the kitchen are compatible with this node. To sawp just edit the `Cargo.toml` file. You may also use this node template to wrap your own custom runtimes. Just make sure you have Babe, Grandpa, and possibly other necessary modules installed properly.

```toml
# Edit these lines to point to a different runtime.
# Your runtime must have the necessary runtime modules to support consensus (Babe, Grandpa, etc)
runtime = { package = "super-node-runtime", path = "../runtimes/super-node-runtime" }
runtime-genesis = { package = "super-node-genesis", path = "../runtimes/super-node-genesis" }
```
