# Generic Structs

In Rust, a `struct`, or structure, is a custom a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a `struct` is like an object’s data attributes (read more in [The Rust Book](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)).

To define a custom struct for the runtime, the following syntax may be used:

```rust
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct MyStruct<A, B> {
    some_number: u32,
    some_generic: A,
    some_other_generic: B,
}
```

In the code snippet above, the [derive macro](https://doc.rust-lang.org/rust-by-example/trait/derive.html) is declared to ensure `MyStruct` conforms to shared behavior according to the specified [traits](https://doc.rust-lang.org/book/ch10-02-traits.html): `Encode, Decode, Default, Clone, PartialEq`

To use the `Encode` and `Decode` traits, it is necessary to import them from `support::codec`:

```rust
use support::codec::{Encode, Decode};
```

By storing types in `MyStruct` as generics, it is possible to access custom Substrate types like `AccountId`, `Balance`, and `Hash`. 

For example, to store a mapping from `AccountId` to `MyStruct` with `some_generic` as the `Balance` type and `some_other_generic` as the `Hash` type:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyMap: map T::AccountId => MyStruct<T::Balance, T::Hash>;
    }
}
```

## Basic Interaction

To push values and modify the map

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_struct(origin, number: u32, balance: T::Balance, secret: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;

            let new_struct = MyStruct {
                some_number: number,
                some_generic: balance,
                some_other_generic: secret,
            };

            <MyMap<T>>::insert(sender, new_struct);
            Ok(())
        }
    }
}
```

## Nested Structs

This basic runtime shows how to store custom, nested structs using a combination of Rust primitive types and Substrate specific types via generics.

```rust
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

decl_storage! {
    trait Store for Module<T: Trait> as RuntimeExampleStorage {
        Value get(value): map u32 => Thing<T::Hash, T::Balance>;
        SuperValue get(super_value): map u32 => SuperThing<T::Hash, T::Balance>;
    }
}
```

For more information, see the [Substrate TCR](https://github.com/parity-samples/substrate-tcr/blob/master/runtime/src/tcr.rs) and the [full tutorial](https://docs.substrate.dev/docs/building-a-token-curated-registry-dappchain-using-substrate)