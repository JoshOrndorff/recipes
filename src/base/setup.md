# Getting Started

If you do not have a Substrate development environment setup on your machine, please install it by following these directions.

### For Linux / macOS

```bash
# Setup Rust and Substrate
curl https://getsubstrate.io -sSf | bash
```

### For Windows

Refer to our [Substrate Installation on Windows](https://substrate.dev/docs/en/next/getting-started#getting-started-on-windows).

## Kitchen Overview

The [`recipes/kitchen`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen) folder contains all the code necessary to run a Substrate node. Let us call it the Kitchen Node. There are three folders inside:

  * [`node`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/node) - The Kitchen Node's client; the non-runtime parts of the node.
  * [`runtimes`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/runtimes) - Complete runtimes that can be used with the Kitchen Node.
  * [`pallets`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets) - Pallets that make up the runtimes. A pallet gives the runtime a particular piece of functionality. Currently, most of the recipe code is stored under this folder.

This section teaches users to interact with [`recipes/kitchen`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen) by
* [Running a Node](./runnode.md)
* [Interacting with the Node](./interactnode.md)
* [Understanding the Kitchen Architecture](./kitchenoverview.md)
