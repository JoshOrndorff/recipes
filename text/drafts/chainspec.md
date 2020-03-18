# Minimal Blockchain Configuration

[Substate Node](https://github.com/paritytech/substrate/tree/master/node) is Substrate's pre-baked blockchain client. By creating and modifying a Substrate Node chain specification file, it is easy to configure a new chain and launch a corresponding testnet.

To use the default chain that comes pre-configured with Substrate Node, enter the following command:

```bash
substrate build-spec --chain=staging > ~/chainspec.json
```

Now, it is simple to modify `~/chainspec.json` in your editor. There are a lot of individual fields for each pallet, and one very large one which contains the WASM code blob for this chain. The most intuitive field to edit is the block `period`. Change it to 10 (seconds):

```json
    "timestamp": {
        "period": 10
    },
```

With this new chainspec file, the "raw" chain definition can be built for the new chain

```bash
substrate build-spec --chain ~/chainspec.json --raw > ~/mychain.json
```

To feed this into Substrate

```bash
substrate --chain ~/mychain.json
```

Until a validator starts producing blocks, noting will happen. To start producing blocks, pass the `--validator` option alongside the seed for the account(s) that are configured as initial authorities.

```bash
substrate --chain ~/mychain.json --validator --key ...
```

Now, distribute `mychain.json` to the relevant authorities to synchronize and, depending on the list of authorities, validate the chain.
