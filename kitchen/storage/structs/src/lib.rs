/// Nested Structs

use support::{ensure, decl_module, decl_storage, decl_event, StorageMap, StorageValue, dispatch::Result};
use support::codec::{Encode, Decode};
use system::ensure_signed;

pub trait Trait: balances::Trait {}

#[derive(Encode, Decode, Default)]
pub struct Thing <Hash, Balance> {
    my_num: u32,
    my_hash: Hash,
    my_balance: Balance,
}

#[derive(Encode, Decode, Default)]
pub struct SuperThing <Hash, Balance> {
    my_super_num: u32,
    my_thing: Thing<Hash, Balance>,
}

decl_storage! {
  trait Store for Module<T: Trait> as NestedStructs {
    Value get(value): map u32 => Thing<T::Hash, T::Balance>;
    SuperValue get(super_value): map u32 => SuperThing<T::Hash, T::Balance>;
  }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn set_mapping(_origin, key: u32, num: u32, hash: T::Hash, balance: T::Balance) -> Result {
      let thing = Thing { 
                      my_num: num, 
                      my_hash: hash, 
                      my_balance: balance
                  };
      <Value<T>>::insert(key, thing);
      Ok(())
    }

    fn set_super_mapping(_origin, key: u32, super_num: u32, thing_key: u32) -> Result {
      let thing = Self::value(thing_key);
      let super_thing = SuperThing { 
                      my_super_num: super_num, 
                      my_thing: thing
                  };
      <SuperValue<T>>::insert(key, super_thing);
      Ok(())
    }
  }
}