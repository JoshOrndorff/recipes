# Robust Path Handling

In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the code utilizes an [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) in order to manage a data race scenario in which a transaction could arrive before some transactions that it `require`s. 

*If Alice sends Bob 100 units and Bob sends Eve 80 of those units. In our example, let's say that Bob's transaction was dependent upon Alice's transaction. If Alice's transaction takes a few more seconds to arrive, we do not want to throw out Bob's transaction. Instead of panicking, we should place Bob's transaction in some queue and lock it for some defined time period.*

To see this pattern in action, consider the `check_transaction` runtime function:

```rust
pub fn check_transaction(transaction: &Transaction) -> CheckResult<'_>
```

This function  returns `CheckResult<'_>`. If we check the type signature of `CheckResult<T>`, we find that it is 

```rust
pub type CheckResult<'a> = rstd::result::Result<CheckInfo<'a>, &'static str>;
```

If we look up the type signature of `CheckInfo<T>`.

```rust
/// Information collected during transaction verification
pub enum CheckInfo<'a> {
    /// Combined value of all inputs and outputs
    Totals { input: Value, output: Value },

    /// Some referred UTXOs were missing
    MissingInputs(Vec<&'a H256>),
}
```

This reveals that in the event of a successful call, it returns either the `Total`s struct that can be easily decomposed to calculate leftover value and distribute it evenly among the authorities OR returns a wrapper around the missing UTXOs which were necessary for verification. Here's the code in `check_transaction` that expresses this logic:

```rust
if missing_utxo.is_empty() {
    ensure!(
        total_input >= total_output,
        "output value must not exceed input value"
    );
    Ok(CheckInfo::Totals {
        input: total_input,
        output: total_input,
    })
} else {
    Ok(CheckInfo::MissingInputs(missing_utxo))
}
```

This pattern demonstrates a *trick* that can be utilized to safely handle the common data race that occurs when a conditional transaction arrives in the transaction pool before the arrival of a transaction that it `require`s. *We can easily extract this pattern to more safely handle common paths in our code for which we may not want to panic, but it may also be preferrable to pause processing.*