# Prerequisites
If you do not have `substrate` installed on your machine, run:

```bash
curl https://getsubstrate.io -sSf | bash
```

## Substrate Module Template

* the difference between the **Runtime** template and the **Module** template

* explain how the kitchen demonstrates how these patterns can be compiled
* explain how these recipes can be imported and used in runtimes as well (see `HOWTO.md`)

**NODE TEMPLATE**
First, modify `lib.rs`. Add `type Event = Event;` to the trait implementation and add `Event` to [`construct_runtime`](https://crates.parity.io/srml_support/macro.construct_runtime.html)

```rust
/// root `lib.rs`
impl runtime_example::Trait for Runtime {
	type Event = Event;
}

...
RuntimeExample: runtime_example::{Module, Call, Event},
...
```

## Create a Substrate Node Template

To start, create an instance of the `substrate-node-template` using the following command:

```bash
substrate-node-new substrate-example <name>
```

To extend the default implementation of the `substrate-node-template`, you will need to modify `substrate-example/runtime/src/lib.rs`.

Add these two lines after the initial declarations:

```rust
mod runtime_example;
impl runtime_example::Trait for Runtime {}
```

Modify the `construct_runtime!()` macro to include `RuntimeExample` at the end:

```rust
/// lib.rs
construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Aura: aura::{Module},
		Indices: indices,
		Balances: balances,
		Sudo: sudo,
		RuntimeExample: runtime_example::{Module, Call, Storage},
	}
);
```

Finally, you need to create a new file called `runtime_example.rs` in the same folder as `lib.rs`.

#### CACHE...
Update the runtime root `lib.rs` file to include the new `Event<T>` type under the module's `Trait` implementation

```rust
/// in root `lib.rs`
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

## Updating Your Runtime

You can paste runtime samples from this Cookbook into the `runtime_examples.rs` file and compile the new runtime binaries with:

```bash
cd substrate-example
cargo build --release
```

Delete the old chain before you start the new one

```bash
./target/release/substrate-example purge-chain --dev
./target/release/substrate-example --dev
```