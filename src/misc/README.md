# Misc Substrate Patterns
> need to be categorized and placed somewhere else!

[Other Code Patterns](#other)
* [Generating Randomness](#random)
* [Hashing Data](#hash)
* [Balance Transfer](#transfer)
* [Print a Message](https://docs.substrate.dev/docs/substrate-runtime-recipes#section-print-a-message)


* [`impl_stubs!`](https://wiki.parity.io/impl_stubs)

* [Accessing Substrate Specific Types](./type.md)

## Generating Randomness <a name = "random"></a>
Substrate uses a safe mixing algorithm to generate randomness using the entropy of previous blocks. Because it is dependent on previous blocks, it can take many blocks for the seed to change. 

```
let random_seed = <system::Module<T>>::random_seed();
```

**To increase entropy**, we can introduce a nonce and a user-specified property. This provides us with a basic RNG on Substrate: 
```
let sender = ensure_signed(origin)?;
let nonce = <Nonce<T>>::get();
let random_seed = <system::Module<T>>::random_seed();

let random_hash = (random_seed, sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

<Nonce<T>>::mutate(|n| *n += 1);
```

## Hashing Data <a name = "hash"></a>

Substrate provides in-built support for hashing data with BlakeTwo256 algorithm. We can get this from the `system` trait. 

```
use runtime_primitives::traits::Hash;
use srml_support::{dispatch::Result};
use {system::{self}};

pub trait Trait: system::Trait {}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    pub fn get_hash(_origin, data: Vec<u8>) -> Result {
      let _digest = <<T as system::Trait>::Hashing as Hash>::hash(&data);
      Ok(())
    }
  }
}
```

The Hashing type under the system trait expoises a function called `hash`. This function takes a reference of a byte array (`Vec<u8>`) and produces a BlakeTwo256 hash digest of it.

The code from above contained a function `get_hash` which takes a `Vec<u8>` parameter `data` and calls the `hash` function on it.

## Making a Balance Transfer <a name = "transfer"></a>
> need to add more on the idiosyncrasies of making a valid balance transfer

```
use srml_support::dispatch::Result;
use system::ensure_signed;

pub trait Trait: balances::Trait {}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn transfer_proxy(origin, to: T::AccountId, value: T::Balance) -> Result {
      let sender = ensure_signed(origin)?;
      <balances::Module<T>>::make_transfer(&sender, &to, value)?;

      Ok(())
    }
  }
}
```
