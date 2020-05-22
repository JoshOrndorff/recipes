## Using the Polkadot UI to Interact

To simplify interactions with the custom Substrate runtime, use the
[Polkadot JS UI for Substrate](https://polkadot.js.org/apps/next/).

By default, this UI is configured to interact with the public Substrate test-network BBQ Birch. To
have it connect to your local node, simply go to:

```
Settings > remote node/endpoint to connect to > Local Node (127.0.0.1:9944)
```

![A picture of the Polkadot UI Settings Tab](https://i.imgur.com/1FpB5aM.png)

If the UI connected successfully, you should be able to go to the **Explorer** tab and see the block
production process running.

![A picture of the block production process running in Explorer tab](https://i.imgur.com/TXmM0cB.png)

You can then interact with your custom functions in the **Extrinsics** tab under **runtimeExample**:

![A picture of custom functions appearing in the Extrinsics tab](https://i.imgur.com/JFXSaHw.png)

### Viewing Storage Variables

If you want to check the value of a storage variable that you created, you can go to:

```
Chain State > runtimeExampleStorage > (variable name)
```

From there you should be able to query the state of the variable. It may return `<unknown>` if the
value has not been set yet.

![A picture of viewing a storage variable in the Polkadot UI](https://i.imgur.com/JLoWxc3.png)

### Viewing Events

Some runtime examples below generate `Events` when functions are run. You can temporarily view these
events in the **Explorer** tab under **recent events** if any get generated.

![A picture of an event getting generated in the Explorer tab](https://i.imgur.com/2jUtBUk.png)

### WASM Runtime Upgrade

Rather than restarting your chain for each update, you can also do an in-place runtime upgrade using
the Polkadot UI. If you do this, you will not get runtime messages appearing in your terminal, but
you should be able to interact with the chain via the UI just fine. To perform the upgrade, go to:

```
Extrinsics > Upgrade Key > upgrade(new)
```

There, you can select the file icon and upload the wasm file generated when you run `./build.sh`

```
substrate-example/runtime/wasm/target/wasm32-unknown-unknown/release/node_runtime.compact.wasm
```

![A picture of upgrading the Substrate runtime](https://i.imgur.com/rujS3p6.png)

Once the upgrade is finalized, you should be able to refresh the UI and see your updates.
