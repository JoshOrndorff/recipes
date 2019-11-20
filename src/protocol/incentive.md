# Incentive Management <a name = "utxo"></a>

In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between transaction inputs and outputs are distributed evenly among the validator set. 

* it is useful to differentiate between the `consensus` and `incentive` layer (Rob's talk somewhere -- actually I remember Fred mentioning this in the first ZeroKnowledgeFM interview with Gavin...)

## Locking Funds at the Protocol Layer

In the [incentive design recipe](./incentive.md#sun), we covered a common bonding pattern also found in the [`srml/staking`](https://github.com/paritytech/substrate/tree/master/srml/staking) and [`srml/council`](https://github.com/paritytech/substrate/tree/master/srml/council) modules which bonds capital via the `reserve => unreserve (=>) transfer` pattern. This pattern works, but there is another way to lock up capital for a defined period of time when building with Substrate.

In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), unspent outputs can be locked up until a defined future block.

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

## Distributing Rewards

Substrate developers need to **stay cognizant of the price paid for resource usage** within the runtime. This can be unintuitive for smart contract developers who are accustomed to interacting with a sandboxed execution environment like the EVM.

Indeed, Substrate is a bit more hands-on. When storage changes occur within a runtime function, they are not automatically reverted if the function panics thereafter. For this reason, it is imperative that any resource used by a transaction must explicitly be paid for within the module. For a more comprehensive explanation, check out [the tcr tutorial's best practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices): *If the resources used might be dependent on transaction parameters or pre-existing on-chain state, then your in-module fee structure must adapt accordingly.*

So how do we design a robust in-module fee structure? In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between inputs and outputs for valid transactions is distributed evenly among the authority set. This pattern demonstrates one approach for incentivizing validation via a floating transaction fee which varies in cost according to the value of the native currency and the relative size/activity of the validator set.

To properly incentivize the ecosystem's actors through the fee structure, the leftover value is distributed evenly among the authorities in the `spend_leftover` runtime function:

```rust
/// uxto-workshop/runtime/src/utxo.rs `decl_module{}`
/// Redistribute combined leftover value evenly among chain authorities
fn spend_leftover(authorities: &[H256]) {
    let leftover = <LeftoverTotal<T>>::take();
    let share_value: Value = leftover
        .checked_div(authorities.len() as Value)
        .ok_or("No authorities")
        .unwrap();
    if share_value == 0 { return }

    let remainder = leftover
        .checked_sub(share_value * authorities.len() as Value)
        .ok_or("Sub underflow")
        .unwrap();
    <LeftoverTotal<T>>::put(remainder as Value);

    for authority in authorities {
        let utxo = TransactionOutput {
            value: share_value,
            pubkey: *authority,
            salt: <system::Module<T>>::block_number().as_(),
        };

        let hash = BlakeTwo256::hash_of(&utxo);

        if !<UnspentOutputs<T>>::exists(hash) {
            <UnspentOutputs<T>>::insert(hash, utxo);
            runtime_io::print("leftover share sent to");
            runtime_io::print(hash.as_fixed_bytes() as &[u8]);
        } else {
            runtime_io::print("leftover share wasted due to hash collision");
        }
    }
}
```