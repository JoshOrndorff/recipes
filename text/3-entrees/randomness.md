# Generating Randomness
*[pallets/randomness](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/randomness/)*

Randomness is useful because... but it's hard to come by because ... Some techniques have been developed... Substrate abstracts this away from the pallet author.

## Disclaimer

All of the randomness sources described here have limitations on their usefullness and security. This recipe shows how to use these randomness sources and makes an effort to explain their trade-offs. However, the author of this recipe is a blockchain chef, not a trained cryptographer. It is your responsibility to understand the security implications of using any of the techniques described in this recipe, before putting them to use. When in doubt, consult a trustworthy cryptographer.

These resources may also be helpful in assessing the security and limitations of these randomness sources.

* https://github.com/paritytech/ink/issues/57
* https://wiki.polkadot.network/docs/en/learn-randomness

## Randomness Trait

## Collective Coin Flipping

Substrate uses a safe mixing algorithm to generate randomness using the entropy of previous blocks. Because it is dependent on previous blocks, it can take many blocks for the seed to change.

```rust, ignore
let random_seed = <system::Module<T>>::random_seed();
```

**To increase entropy**, we can introduce a nonce and a user-specified property. This provides us with a basic RNG on Substrate:
```rust, ignore
let random_seed = <system::Module<T>>::random_seed();
let nonce = <Nonce>::get();
let new_random = (random_seed, nonce)
    .using_encoded(|b| Blake2Hasher::hash(b))
    .using_encoded(|mut b| u64::decode(&mut b))
    .expect("Hash must be bigger than 8 bytes; Qed");
let new_nonce = <Nonce>::get() + 1;
<Nonce<T>>::put(new_nonce);
```

## Babe VRF Output
