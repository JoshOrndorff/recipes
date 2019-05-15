# Transaction Ordering

To customize the transaction ordering logic for a Substrate blockchain, use the `TaggedTransactionQueue` trait to specify the transaction queue logic and mitigate race conditions. The [docs](https://crates.parity.io/substrate_client/runtime_api/trait.TaggedTransactionQueue.html?search=) reveal that it is necessary to implement following function

```rust
fn validate_transaction(
    &self,
    at: &BlockId<Block>,
    tx: <Block as BlockT>::Extrinsic
) -> Result<TransactionValidity, Error>
```

This function signature reveals that, in the event of a successful call, the return type must be `TransactionValidity`. Upon looking this type up [in the docs](https://crates.parity.io/sr_primitives/transaction_validity/enum.TransactionValidity.html), it is clear that this enum has three variants

```rust
pub enum TransactionValidity {
    Invalid(i8),
    Valid {
        priority: TransactionPriority,
        requires: Vec<TransactionTag>,
        provides: Vec<TransactionTag>,
        longevity: TransactionLongevity,
    },
    Unknown(i8),
}
```

In the context of the `utxo-workshop`, specify the hashes of required transactions (`missing_utxos`) in the `requires` field while also specifying the list of transactions for which this utxo satisfies `requires` in the `provides` field. The `longevity` field is set somewhat arbitrarily, and the `priority` field serves simply to enforce an ordering on the set of transactions.  

```rust
impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
        use support::IsSubType;
        use runtime_primitives::{
            traits::Hash,
            transaction_validity::{TransactionLongevity, TransactionPriority, TransactionValidity},
        };

        // Special handling for extrinsics representing UTXO transactions
        if let Some(&utxo::Call::execute(ref transaction)) = IsSubType::<utxo::Module<Runtime>>::is_aux_sub_type(&tx.function) {
            // List of tags to require
            let requires;

            // Transaction priority to assign
            let priority;

            const INVALID_UTXO: i8 = -99;

            match <utxo::Module<Runtime>>::check_transaction(&transaction) {
                // Transaction verification failed
                Err(e) => {
                    runtime_io::print(e);
                    return TransactionValidity::Invalid(INVALID_UTXO);
                }

                // Transaction is valid and verified
                Ok(utxo::CheckInfo::Totals {input, output}) => {
                    // All input UTXOs were found, so we consider input conditions to be met
                    requires = Vec::new();

                    // Priority is based on a transaction fee that is equal to the leftover value
                    let max_priority = utxo::Value::from(TransactionPriority::max_value());
                    priority = max_priority.min(input - output) as TransactionPriority;
                }
                
                // Transaction is missing inputs
                Ok(utxo::CheckInfo::MissingInputs(missing)) => {
                    // Since some referred UTXOs were not found in the storage yet,
                    // we tag current transaction as requiring those particular UTXOs
                    requires = missing
                        .iter()         // copies itself into a new vec
                        .map(|hash| hash.as_fixed_bytes().to_vec())
                        .collect();

                    // Transaction could not be validated at this point,
                    // so we have no sane way to calculate the priority    
                    priority = 0;
                }
            }

            // Output tags this transaction provides
            let provides = transaction.outputs
                .iter()
                .map(|output| BlakeTwo256::hash_of(output).as_fixed_bytes().to_vec())
                .collect();

            return TransactionValidity::Valid {
                requires,
                provides,
                priority,
                longevity: TransactionLongevity::max_value(),
            };
        }

        // Fall back to default logic for non UTXO::execute extrinsics
        Executive::validate_transaction(tx)
    }
}
```

*Read more about the [transaction lifecycle](https://docs.substrate.dev/docs/transaction-lifecycle-in-substrate) in Substrate*