# Module Fundamentals

Clone the [substrate module template](https://github.com/shawntabrizi/substrate-module-template):
```bash
git clone https://github.com/shawntabrizi/substrate-module-template
```

For an in-depth explanation of using this template, see *[Creating a Substrate Runtime Module](https://substrate.dev/docs/en/tutorials/creating-a-runtime-module)*.

## Hello Substrate

Import the following from [`srml-support`](https://crates.parity.io/srml_support/index.html):

```rust
use support::{decl_module, decl_event, decl_storage, StorageValue, StorageMap};
use system::ensure_signed;
```

The blockchain's runtime storage is configured in [`decl_storage`](https://crates.parity.io/srml_support/macro.decl_storage.html). 

```rust
decl_storage! {
	trait Store for Module<T: Trait> as HelloWorld {
		pub LastValue get(last_value): u64; 
		pub UserValue get(user_value): map T::AccountId => u64;
	}
}
```

The runtime methods defined in [`decl_module`](https://crates.parity.io/srml_support/macro.decl_module.html) are used to define permissions for interacting with runtime storage.

```rust
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn set_value(origin, value: u64) {
			let sender = ensure_signed(origin)?;
			LastValue::put(value);
			UserValue::<T>::insert(&sender, value);
			Self::deposit_event(RawEvent::ValueSet(sender, value));
		}
	}
}
```

Events are declared in [`decl_event`](https://crates.parity.io/srml_support/macro.decl_event.html). The emission of events is used to determine successful execution of the logic in the body of runtime methods.

```rust
decl_event!{
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
	{
		ValueSet(AccountId, u64),
	}
}
```

*It is also possible to declare an error type for runtime modules with [`decl_error`](https://crates.parity.io/srml_support/macro.decl_error.html)*
