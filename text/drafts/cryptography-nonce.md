## Salt

Sometimes we require a unique identifier for items that may take the exact same form, but have
different block numbers. In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the UTXO
set included a `salt` field for UTXO output to establish uniqueness for every transaction. This
ensures that, as long as the outputs are validated in different blocks, they can both be invoked
independently without leaking information regarding the `TransactionInput`'s `Signature`.

```rust, ignore
TransactionOutput {
    value: Value    // u128 alias
    pubkey: H256    // public key
    salt: u32       // blocknummber
}
```

Setting `salt` to something as inconspicuous as `BlockNumber` still ensures that there arent enough
of the same output in each block to open the _replay attack_ vector described above.

## Encrypted Nonce Pattern

> also known as epoch reclamation

Peer to peer nodes connect over an encrypted communication channels referred to as _privatization_
in protocols like Libp2p. An encrypted connection can simply be bootstrapped from simple public key
cryptography.

I think we'll see this value used increasingly often in Layer 2 solutions which invoke the encrypted
nonce to prove the order of off-chain messages. For this reason, I'll provide a sample here.

-   examples in the codebases (`Cumulus`, `Substrate`, `Polkadot`, `Rust-Libp2p`)
-   `evmap` uses this pattern...go through that...
