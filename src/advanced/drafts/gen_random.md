# Generating Randomness

## PRF

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


## VRF
* creating some on-chain random generation MPC implementation could be interesting
* just create an API for accessing a Rust VRF implementation