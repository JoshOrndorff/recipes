# Special Field Objects


## Public vs Private

* and do we maintain access to all of its methods in the runtime if it's declared outside the `decl_module` block
* also for functions?

## [COMPACT]

When do you use compact and when do you not? What are the benefits to runtime storage and what are the implications?

## PhantomData

* when do we need this and what is it's use?
* `troubles.md`
* examples in the codebases (`Cumulus`, `Substrate`, `Polkadot`, `Rust-Libp2p`)

## Salt

Sometimes we require a unique identifier for items that may take the exact same form, but have different block numbers. In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the UTXO set included a `salt` field for UTXO output to establish uniqueness for every transaction. This ensures that, as long as the outputs are validated in different blocks, they can both be invoked independently without leaking information regarding the `TransactionInput`'s `Signature`.

```rust
TransactionOutput {
    value: Value    // u128 alias
    pubkey: H256    // public key
    salt: u32       // blocknummber
}

```

Setting `salt` to something as inconspicuous as `BlockNumber` still ensures that there arent enough of the same output in each block to open the *replay attack* vector described above.

## Encrypted Nonce Pattern

> also known as epoch reclamation

Peer to peer nodes connect over an encrypted communication channels referred to as *privatization* in protocols like Libp2p. An encrypted connection can simply be bootstrapped from simple public key cryptography.

I think we'll see this value used increasingly often in Layer 2 solutions which invoke the encrypted nonce to prove the order of off-chain messages. For this reason, I'll provide a sample here.

```rust
/// TODO
/// --> look for this code... (bootstrapping public key connections...)
```

* examples in the codebases (`Cumulus`, `Substrate`, `Polkadot`, `Rust-Libp2p`)
* `evmap` uses this pattern...go through that...