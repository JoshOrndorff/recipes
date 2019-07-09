## Robust Path Handling

* `if let some`
* `checked_sub` pattern from recent issue (link to safe-math, but put that pattern in here)...don't link, just natural flow...reference it though

* abstract out the pattern and make it as generic as possible

### Ergonomic Enums

In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the code utilizes an [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) to manage a data race scenario in which a transaction could arrive before some transactions that it `require`s. 

*Alice sends Bob 100 units and Bob sends Eve 80 of those units. Let's assume that Bob's transaction is dependent upon Alice's transaction. If Alice's transaction takes a few more seconds to arrive, we do not want to throw out Bob's transaction. Instead of panicking, we should place Bob's transaction in a temporary queue and lock it for some defined time period.*

To see this pattern in action, see the `check_transaction` runtime function:

```rust
pub fn check_transaction(transaction: &Transaction) -> CheckResult<'_>
```

This function  returns `CheckResult<'_>`. The type signature of `CheckResult<T>`:

```rust
pub type CheckResult<'a> = rstd::result::Result<CheckInfo<'a>, &'static str>;
```

The type signature of `CheckInfo<T>`:

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

This pattern demonstrates one way to safely handle the common data race that occurs when a conditional transaction arrives in the transaction pool before the arrival of a transaction that it `require`s. *We can extract this pattern to safely handle conditional paths in our code for which panics are undesirable, but it is also preferrable to pause processing.*