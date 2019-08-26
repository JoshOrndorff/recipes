# Prerequisites
If you do not have `substrate` installed on your machine, run:

```bash
curl https://getsubstrate.io -sSf | bash
```

<!-- fast install with --fast -->

## Substrate Templates

[substrate-package](https://github.com/shawntabrizi/substrate-package) contains the UI, module, and runtime templates for building with Substrate. The [substrate-module-template](https://github.com/shawntabrizi/substrate-module-template) is the simplest path to experimenting with Substrate. Modules are modular pieces of code that can be composed within a single runtime. 

Likewise, the [substrate-node-template](https://github.com/shawntabrizi/substrate-package/tree/master/substrate-node-template) provides all necessary scaffolding for running a functional Substrate node. Each Substrate runtime contains multiple modules that comprise the logic of the defined Substrate blockchain.

The [substrate-ui](https://github.com/shawntabrizi/substrate-package/tree/master/substrate-ui) provides a template for building a compatible UI that works with the node template.

### Runtime Module

Clone the [substrate-module-template](https://github.com/shawntabrizi/substrate-module-template)

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

### Runtime Node

Clone the [substrate-node-template](https://github.com/shawntabrizi/substrate-package/tree/master/substrate-node-template) and add module logic to [`runtime/src/template.rs`](https://github.com/shawntabrizi/substrate-package/blob/master/substrate-node-template/runtime/src/template.rs).

Update the runtime root `lib.rs` file to include the new `Event<T>` type under the module's `Trait` implementation

```rust
/// in root `lib.rs`
mod mymodule;

impl mymodule::Trait for Runtime {
    type Event = Event<T>;
}
```

Include the `Event<T>` type in the module's definition in the [`construct_runtime`](https://crates.parity.io/srml_support/macro.construct_runtime.html) macro block.

```rust
/// in root `lib.rs`
construct_runtime!(
    pub enum Runtime for Log(InteralLog: DigestItem<Hash, Ed25519AuthorityId) where
        Block = Block,
        NodeBlock = opaque::Block,
        InherentData = BasicInherentData
    {
        ...
        MyModule: mymodule::{Module, Call, Storage, Event<T>},
    }
);
```

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