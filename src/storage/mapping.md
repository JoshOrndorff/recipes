# Mapping

Mappings are a very powerful primitive. A *stateful* cryptocurrency might store a mapping between accounts and balances (see [simple token](#ex)). Likewise, mappings prove useful when representing *owned* data. By tracking ownership with maps, we can easily manage permissions for modifying values specific to individual users or groups.

Within a specific module, a key-value mapping (between `u32` types) can be stored with this syntax:

```rust
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		MyMap: map u32 => u32;
	}
}
```

## Basic Interaction

To interact with a storage map, it is necessary to import the `support::StorageMap` type. Functions used to access a `StorageValue` are defined in [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs):

```rust
/// Get the prefix key in storage.
fn prefix() -> &'static [u8];

/// Get the storage key used to fetch a value corresponding to a specific key.
fn key_for(x: &K) -> Vec<u8>;

/// true if the value is defined in storage.
fn exists<S: Storage>(key: &K, storage: &S) -> bool {
    storage.exists(&Self::key_for(key)[..])
}

/// Load the value associated with the given key from the map.
fn get<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Take the value under a key.
fn take<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Store a value to be associated with the given key from the map.
fn insert<S: Storage>(key: &K, val: &V, storage: &S) {
    storage.put(&Self::key_for(key)[..], val);
}

/// Remove the value under a key.
fn remove<S: Storage>(key: &K, storage: &S) {
    storage.kill(&Self::key_for(key)[..]);
}

/// Mutate the value under a key.
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(key: &K, f: F, storage: &S) -> R;
```

To insert a `(key, value)` pair into a `StorageMap` named `MyMap`:

```rust
<MyMap<T>>::insert(key, value);
```

To query `MyMap` for the `value` corresponding to a `key`:

```rust
let value = <MyMap<T>>::get(key);
```

## Simple Token <a name = "ex"></a>

If we want to implement a simple token transfer with Substrate, we need to 
1. set total supply
2. establish ownership upon configuration of circulating tokens
3. coordinate token transfers with our runtime functions

```rust
decl_storage! {
  trait Store for Module<T: Trait> as Template {
    pub TotalSupply get(total_supply): u64 = 21000000;

    pub BalanceOf get(balance_of): map T::AccountId => u64;

    Init get(is_init): bool;
  }
}
```

We should also set an event for when token transfers occur to notify clients

```rust
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        // event for transfer of tokens
        // from, to, value
        Transfer(AccountId, AccountId, u64),
    }
);
```

To integrate this logic into our module, we could write the following code:

```rust
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      // initialize the default event for this module
      fn deposit_event<T>() = default;

      // initialize the token
      // transfers the total_supply amout to the caller
      fn init(origin) -> Result {
        let sender = ensure_signed(origin)?;
        ensure!(Self::is_init() == false, "Already initialized.");

        <BalanceOf<T>>::insert(sender, Self::total_supply());

        <Init<T>>::put(true);

        Ok(())
      }

      // transfer tokens from one account to another
      fn transfer(_origin, to: T::AccountId, value: u64) -> Result {
        let sender = ensure_signed(_origin)?;
        let sender_balance = Self::balance_of(sender.clone());
        ensure!(sender_balance >= value, "Not enough balance.");

        let updated_from_balance = sender_balance.checked_sub(value).ok_or("overflow in calculating balance")?;
        let receiver_balance = Self::balance_of(to.clone());
        let updated_to_balance = receiver_balance.checked_add(value).ok_or("overflow in calculating balance")?;
        
        // reduce sender's balance
        <BalanceOf<T>>::insert(sender.clone(), updated_from_balance);

        // increase receiver's balance
        <BalanceOf<T>>::insert(to.clone(), updated_to_balance);

        Self::deposit_event(RawEvent::Transfer(sender, to, value));
        
        Ok(())
      }
  }
}
```

The full code from this example can be found [here]([gautamdhameja/substrate-demo](https://github.com/gautamdhameja/substrate-demo/blob/master/runtime/src/template.rs))