# Prerequisites
If you do not have `substrate` installed on your machine, run:

```bash
curl https://getsubstrate.io -sSf | bash
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

## Updating Your Runtime

You can paste any of the runtime samples below into that `runtime_examples.rs` file and compile the new runtime binaries with:

```bash
cd substrate-example
cargo build --release
```

Delete the old chain before you start the new one

```bash
substrate purge-chain --dev
./target/release/substrate-example --dev
```