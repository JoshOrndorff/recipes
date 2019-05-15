# Scheduling Collateralization

In the [incentive design recipe](./incentive.md#sun), we covered a common bonding pattern also found in the [`srml/staking`](https://github.com/paritytech/substrate/tree/master/srml/staking) and [`srml/council`](https://github.com/paritytech/substrate/tree/master/srml/council) modules which bonds capital via the `reserve => unreserve (=>) transfer` pattern. This pattern works, but there is another way to lock up capital for a defined period of time when building with Substrate.

In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), unspent outputs can be locked up until a defined future block. A similar pattern is exercised in the [`collateral`](https://github.com/nczhu/collateral) sample. *S/O [`nczhu`](https://github.com/nczhu) for mastering and applying the pattern in both of these examples*

* [UTXO Locking](#lock)
* [Managing Collateral](#collatz)

## UTXO Locking <a name = "lock"></a>

First, define an enum to distinguish between locked and unlocked UTXOs.

```rust
/// utxo-workshop/runtime/src/utxo.rs
/// A UTXO can be locked indefinitely or until a certain block height
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Hash)]
pub enum LockStatus<BlockNumber> {
    Locked,
    LockedUntil(BlockNumber),
}
```

In `decl_storage`, define a map for specifying the locked UTXOs. This maps an unspent outputs public key (`H256`) to its `LockStatus`.

```rust
/// utxo-workshop/runtime/src/utxo.rs
decl_storage {
    trait Store for Module<T: Trait> as Utxo {
        /// All UTXO that are locked
        LockedOutputs: map H256 => Option<LockStatus<T::BlockNumber>>;
    }
}
```

Specify a runtime function for adding UTXOs to the mapping. Before inserting an unspent output into the storage mapping, check that the UTXO exists and is not already locked.

```rust
/// utxo-workshop/runtime/src/utxo.rs
impl<T: Trait> Module<T> {
    pub fn lock_utxo(hash: &H256, until: Option<T::BlockNumber>) -> Result {
        ensure!(!<LockedOutputs<T>>::exists(hash), "utxo is already locked");
        ensure!(<UnspentOutputs<T>>::exists(hash), "utxo does not exist");

        if let Some(until) = until {
            ensure!(
                until > <system::Module<T>>::block_number(),
                "block number is in the past"
            );
            <LockedOutputs<T>>::insert(hash, LockStatus::LockedUntil(until));
        } else {
            <LockedOutputs<T>>::insert(hash, LockStatus::Locked);
        }

        Ok(())
    }
}
```

Next, add a runtime function to unlock UTXOs.

```rust
/// utxo-workshop/runtime/src/utxo.rs
impl<T: Trait> Module<T> {
    pub fn unlock_utxo(hash: &H256) -> Result {
        ensure!(!<LockedOutputs<T>>::exists(hash), "utxo is not locked");
        <LockedOutputs<T>>::remove(hash);
        Ok(())
    }
}
```

Next, verify that all of the unspent outputs claimed by transaction inputs are not locked in the `check_transaction` runtime function.

```rust
/// utxo-workshop/runtime/src/utxo.rs
impl<T: Trait> Module<T> {
    pub fn check_transaction(transaction: &Transaction) -> CheckResult<'_> {
        for input in transaction.inputs.iter() {
            // Fetch UTXO from the storage
            if let Some(output) = <UnspentOutputs<T>>::get(&input.parent_output) {
                ensure!(
                    !<LockedOutputs<T>>::exists(&input.parent_output),
                    "utxo is locked"
                );
            }
        }
    }
}
```

## Collateral Management <a name = "collatz"></a>

*in progress :) -- see [`collateral`](https://github.com/nczhu/collateral) for a sneak peek of the recipe*