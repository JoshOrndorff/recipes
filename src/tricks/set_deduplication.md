# Verifying Member Uniqueness

There are certain advantageous patterns for verifying the uniqueness of an item in a set. In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), we saw in the `check_transaction` function how we could ensure there were no two of the same utxo's by collecting all of them into a BTreeMap and then checking for equality between the BTreeMap (which, like a set, does not add additional of the same element)  and the original set (which could include duplicates). This constituents a check that the set of UTXOs selected are all unique and not repeated.

However, this pattern can easily be extracted and applied in all situations for which membership uniqueness needs to be checked for some vector. In the context of the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), we have:

```rust
{
    let input_set: BTreeMap<_, ()> =
        transaction.inputs.iter().map(|input| (input, ())).collect();

    ensure!(
        input_set.len() == transaction.inputs.len(),
        "each input must only be used once"
    );
}

{
    let output_set: BTreeMap<_, ()> = transaction
        .outputs
        .iter()
        .map(|output| (output, ()))
        .collect();

    ensure!(
        output_set.len() == transaction.outputs.len(),
        "each output must be defined only once"
    );
}
```

The use of separation into scopes also minimizes the lifetime of the initialized variables within the given scopes. This should logically be done because there is no need to store the `BTreeMap<T>` for longer than is necessary.