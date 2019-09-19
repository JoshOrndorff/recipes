# Generating Randomness

Substrate uses a safe mixing algorithm to generate randomness using the entropy of previous blocks. Because it is dependent on previous blocks, it can take many blocks for the seed to change. 

```rust
let random_seed = <system::Module<T>>::random_seed();
```

**To increase entropy**, we can introduce a nonce and a user-specified property. This provides us with a basic RNG on Substrate: 
```rust
let random_seed = <system::Module<T>>::random_seed();
let nonce = <Nonce>::get();
let new_random = (random_seed, nonce)
    .using_encoded(|b| Blake2Hasher::hash(b))
    .using_encoded(|mut b| u64::decode(&mut b))
    .expect("Hash must be bigger than 8 bytes; Qed");
let new_nonce = <Nonce>::get() + 1;
<Nonce<T>>::put(new_nonce);
```

**also see...**
* [code in kitchen](https://github.com/substrate-developer-hub/recipes/blob/master/kitchen/random/src/lib.rs)
* https://github.com/paritytech/ink/issues/57

**[Back to Recipes](https://substrate.dev/recipes/)**