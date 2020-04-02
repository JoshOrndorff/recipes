# Manual Seal
*[`nodes/manual-seal`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/manual-seal)*

This recipe demonstrates a Substrate node using the [Manual Seal consensus](https://substrate.dev/rustdocs/master/sc_consensus_manual_seal/index.html). Unlike the other consensus engines included with Substrate, manual seal does not create blocks on a regular basis. Rather, it waits for an RPC call telling to create a block. This recipe also demonstrates the Instant Seal engine which creates a block as soon as a transaction is ready in the pool.

## Using Manual Seal

TODO
it's a lot like the basic-pow

## Wiring the RPC

TODO
it's a lot like our custom RPC

## Manual vs Instant
TODO

## Manually Sealing Blocks
Once your node is running, you will see that it just sits there idly. It will accept transactions to the pool, but it will not author blocks on its own. In manual seal, the node does not author a block until we explicitly tell it to. We can tell it to author a block by calling the `engine_createBlock` RPC.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_createBlock",
      "params": [true, false, null]
    }'
```

This call takes three parameters, each of which are worth exploring.

### Create Empty
`create_empty` is a Boolean value indicating whether empty blocks may be created. Setting `create-empty` to true does not mean that an empty block will necessarily be created. Rather it means that the engine should go ahead creating a block even if no transaction are present. If transactions are present in the queue, they will be included regardless of `create_empty`'s value.'

### Finalize
`finalize` is a Boolean indicating whether the block (and its ancestors, recursively) should be finalized after creation. Manually controlling finality is interesting, but also dangerous. If you attempt to author and finalize a block that does not build on the best finalized chain, the block will not be imported. If you finalize one block in one node, and a conflicting block in another node, you will cause a safety violation when the nodes synchronize.

TODO Actually the current state of the code is that if you start a multinode network, the nodes mysteriously finalize blocks intermittently. Not sure about that.

### Parent Hash
`parent_hash` is an optional hash of a block to use as a parent. To set the parent, use the format `"0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764"`. To omit the parent, use `null`. When the parent is omitted the block is built on the current best block. Manually specifying the parent is useful for constructing fork scenarios and demonstrating chain reorganizations.

### Finalizing Blocks Later
In addition to finalizing blocks while creating them, they can be finalized later by using the second provided RPC call, `engine_finalizeBlock`.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_createBlock",
      "params": ["0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764", null]
    }'
```

The two parameters are:
* The hash of the block to finalize.
* A Justification. TODO what is the justification and why might I want to use it?

## Instantly Sealing Blocks

In addition to the manual seal mechanism we've explored so far, this node also provides an option to seal blocks instantly when transactions are received into the pool. To run the node in this mode use the TODO flag.
