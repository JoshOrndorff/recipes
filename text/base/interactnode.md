# Interact with the Kitchen Node

If you followed [the instructions](./runnode.md) to get your node running, you should see blocks created on the console. You can navigate to the  [Polkadot-JS Apps](https://polkadot.js.org/apps/#/settings/developer?rpc=ws://127.0.0.1:9944) user interface. This is a general purpose interface for interacting with many different Substrate-based blockchains. From now on we'll call it "Apps" for short. Before Apps will work with our blockchain, we need to give it a little chain-specific information known as the "types". You'll learn what all this means as you work through the recipes; for now just follow the instructions.

Please copy the contents of `kitchen/runtimes/super-runtime/types.json` into Apps. You should already be on the Settings -> Developer tab. If not, please navigate there.

![Screenshot: pasting types into Apps UI](../apps-types.png)

As you work through the recipes, you will use the **Chain state** tab to query the blockchain status and **Extrinsics** to send transactions to the blockchain. Feel free to play around.
