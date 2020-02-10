# Pallet Fundamentals
*[`pallets/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/hello-substrate)*

Clone the [substrate pallet template](https://github.com/substrate-developer-hub/substrate-pallet-template):
```bash
git clone https://github.com/substrate-developer-hub/substrate-pallet-template
```

At the top of the `src/lib.rs` file, import the following from [`frame-support`](https://substrate.dev/rustdocs/master/frame_support/index.html):

```rust, ignore
use frame_support::{decl_module, decl_event, decl_storage, StorageValue, StorageMap};
use system::ensure_signed;
```

The blockchain's runtime storage is configured in [`decl_storage`](https://substrate.dev/rustdocs/master/frame_support/macro.decl_storage.html).

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as HelloWorld {
		pub LastValue get(fn last_value): u64;
		pub UserValue get(fn user_value): map T::AccountId => u64;
	}
}
```

Defined in [`decl_module`](https://substrate.dev/rustdocs/master/frame_support/macro.decl_module.html), the runtime methods specify acceptable interaction with runtime storage.

```rust, ignore
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

Events are declared in [`decl_event`](https://substrate.dev/rustdocs/master/frame_support/macro.decl_event.html). The emission of events is used to determine successful execution of the logic in the body of runtime methods.

```rust, ignore
decl_event!{
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
	{
		ValueSet(AccountId, u64),
	}
}
```

*It is also possible to declare an error type for pallets with [`decl_error`](https://substrate.dev/rustdocs/master/frame_support/macro.decl_error.html)*
