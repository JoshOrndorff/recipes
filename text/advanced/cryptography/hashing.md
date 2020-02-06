# Tutorial Series for Using Cryptography on Substrate
> just an idea

## Hashing

Substrate provides in-built support for hashing data with BlakeTwo256 algorithm. We can get this from the `system` trait.

```
use runtime_primitives::traits::Hash;
use frame_support::{dispatch::Result};
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
