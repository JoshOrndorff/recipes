# Building a Node

## Prerequisites

Before we can even begin compiling our first blockchain node, we need to have a properly configured Rust toolchain. There is a convenient script that will set up this toolchain for us, and we can run it with the following command.

```bash
# Setup Rust and Substrate
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

> This command downloads and executes code from the internet. Give yourself peace-of-mind by inspecting the [script's source](https://getsubstrate.io) to confirm it isn't doing anything nasty.

### For Windows

These instructions and the rest of the instructions in this chapter assume a unix-like environment such as Linux, MacOS, or Windows Subsystem for Linux (WSL). If you are a Windows user, WSL is the best way to proceed. If you want or need to work in a native Windows environment, this is possible, but is not covered in detail here. Please follow along with the [Getting Started on Windows](https://substrate.dev/docs/en/overview/getting-started#getting-started-on-windows) guide, then return here when you're ready to proceed.

## Compile the Kitchen Node

If you haven't already, `git clone` the recipes repository. We also want to kick-start the node compilation as it may take about 30 minutes to complete depending on your hardware.

```bash
# Clone the Recipes Repository
git clone https://github.com/substrate-developer-hub/recipes.git
cd recipes

#  Update Rust-Wasm toolchain
./nodes/scripts/init.sh

# Compile the Kitchen Node
# This step takes a while to complete
cargo build --release -p kitchen-node
```

As you work through the recipes, refer back to these instructions each time you wish to re-compile the node. Over time the commands will become familiar, and you will even modify them to compile other nodes.

## Checking Your Work

Once the compilation is completed, you can ensure that the node has built properly by displaying its help page. Notice that the node has built to the `target/release` directory. This is the default location for Rust projects.

```bash
# Inside `recipes` directory

# Display the Kitchen Node's help page
./target/release/kitchen-node --help
```
