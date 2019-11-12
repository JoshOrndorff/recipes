# Off-Chain Optimizations
* off-chain workers by Gautam (TODO: ask him for help with this)




# **OLD**::{Offline Interaction via Inherent}
*[Substrate Inherents Sample](https://github.com/gautamdhameja/substrate-inherents-sample)*

Often times, it might be necessary to incorporate off-chain data as inputs for processes validated on-chain. In these situations, it is useful to be familiar with [`substrate-inherents`](https://crates.parity.io/substrate_inherents/index.html).

Once `substrate-inherents`is added as a dependency in the `cargo.toml` file and the module is declared publicly, the `Inherent` parameter should be added to the custom module definition in the `construct_runtime!`macro.

```rust
/// in lib.rs
construct_runtime!(
    pub enum ...
    {
        // ...
        Template: template::{Module, Call, Storage, Inherent},
    }
)
```

Thereafter, the `InherentIdentifier` needs to be set. This parameter is the unique identifier for the module's inherent data in the `InherentData` storage -- it should be unique across the runtime.

```rust
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"tknusd00";
pub type InherentType = u64;
```

In [`aura`](https://github.com/paritytech/substrate/blob/master/srml/aura/src/lib.rs), this looks like the following:

```rust
/// The aura inherent identifier.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"auraslot";

/// The type of the aura inherent.
pub type InherentType = u64;
```

[`aura`](https://github.com/paritytech/substrate/blob/master/srml/aura/src/lib.rs) also maintains an auxiliary trait to extract the Aura consensus inherent data:

```rust
impl AuraInherentData for InherentData {
	fn aura_inherent_data(&self) -> result::Result<InherentType, RuntimeString> {
		self.get_data(&INHERENT_IDENTIFIER)
			.and_then(|r| r.ok_or_else(|| "Aura inherent data not found".into()))
	}

	fn aura_replace_inherent_data(&mut self, new: InherentType) {
		self.replace_data(INHERENT_IDENTIFIER, &new);
	}
}
```

According to runtime convention, the module must also define an [`InherentDataProvider`](https://crates.parity.io/substrate_inherents/struct.InherentDataProviders.html) type and implement the [`ProvideInherentData`](https://crates.parity.io/substrate_inherents/trait.ProvideInherentData.html) trait. This implementation defines how the consensus engine specifies the inherent data to the runtime before block production time. Specifically, [`ProvideInherent`](https://crates.parity.io/substrate_inherents/trait.ProvideInherent.html) indicates the required logic in `provide_inherent_data` to calculate the inherent data and store it in the [`InherentData`](https://crates.parity.io/substrate_inherents/struct.InherentData.html) storage. For the `aura` module, the `InherentDataProvider` is declared with the slot duration inherent data for Aura consensus. 

```rust
/// Provides the slot duration inherent data for `Aura`.
#[cfg(feature = "std")]
pub struct InherentDataProvider {
	slot_duration: u64,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
	pub fn new(slot_duration: u64) -> Self {
		Self {
			slot_duration
		}
	}
}
```

The implementation of `ProvideInherentData` is provided for the `InherentDataProvider`; it essentially calculates the slot number as the timestamp over the slot duration (see body of `provide_inherent_data`):

```rust
#[cfg(feature = "std")]
impl ProvideInherentData for InherentDataProvider {
	fn on_register(
		&self,
		providers: &InherentDataProviders,
	) -> result::Result<(), RuntimeString> {
		if !providers.has_provider(&timestamp::INHERENT_IDENTIFIER) {
			// Add the timestamp inherent data provider, as we require it.
			providers.register_provider(timestamp::InherentDataProvider)
		} else {
			Ok(())
		}
	}

	fn inherent_identifier(&self) -> &'static inherents::InherentIdentifier {
		&INHERENT_IDENTIFIER
	}

	fn provide_inherent_data(
		&self,
		inherent_data: &mut InherentData,
	) -> result::Result<(), RuntimeString> {
		let timestamp = inherent_data.timestamp_inherent_data()?;
		let slot_num = timestamp / self.slot_duration;
		inherent_data.put_data(INHERENT_IDENTIFIER, &slot_num)
	}

	fn error_to_string(&self, error: &[u8]) -> Option<String> {
		RuntimeString::decode(&mut &error[..]).map(Into::into)
	}
}
```

*For a more comprehensive introduction, see the [Substrate inherents sample](https://github.com/gautamdhameja/substrate-inherents-sample)*