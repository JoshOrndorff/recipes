# Prerequisites
If you do not have `substrate` installed on your machine, run:

```bash
curl https://getsubstrate.io -sSf | bash
```

While the code compiles, read about how the [Substrate runtime architecture](https://substrate.dev/docs/en/runtime/architecture-of-a-runtime) composes [modules](https://substrate.dev/docs/en/runtime/substrate-runtime-module-library) to configure a runtime. 

## Module

*At the moment*, this resource focuses primarily on module development patterns, though there are plans to add examples of interesting runtime configurations using the existing modules. To develop in the context of the module, it is sufficient to clone the [module-template](https://github.com/shawntabrizi/substrate-module-template)

```bash
$ git clone https://github.com/shawntabrizi/substrate-module-template
```

build with 

```bash
$ cargo build
```

test with 

```bash
$ cargo test
```

See **[Creating a Runtime Module](https://substrate.dev/docs/en/tutorials/creating-a-runtime-module)** in [the official docs](https://substrate.dev/en/tutorials).

## Runtime

To develop in the context of the runtime, clone the [substrate-node-template](https://github.com/shawntabrizi/substrate-package/tree/master/substrate-node-template) and add module logic to [`runtime/src/template.rs`](https://github.com/shawntabrizi/substrate-package/blob/master/substrate-node-template/runtime/src/template.rs).

**Updating the Runtime**

Compile runtime binaries

```bash
cd runtime
cargo build --release
```

Delete the old chain before you start the new one (*this is a very useful command sequence when building and testing runtimes*)

```bash
./target/release/substrate-example purge-chain --dev
./target/release/substrate-example --dev
```