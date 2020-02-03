# Run the Kitchen Node

To run the code in the recipes, `git clone` the source repository. We also want to kick-start the node compilation as it may take about 30 minutes to complete depending on your hardware.

```bash
git clone https://github.com/substrate-developer-hub/recipes.git
cd recipes/kitchen/nodes/kitchen-node
./scripts/init.sh

# This step takes a while to complete
cargo build --release
```

> **Notes**
>
> Refer to the following sections to:
>
>  * Learn more about [Substrate runtime](https://substrate.dev/docs/en/runtime/architecture-of-a-runtime)

Once the compilation is completed, you can first purge any existing blockchain data (useful to start your node from a clean state in future) and then start the node.

```bash
# Inside `recipes/kitchen` folder

# Purge any existing blockchain data. Enter `y` upon prompt.
./target/release/kitchen-node purge-chain --dev

# Start the Kitchen Node
./target/release/kitchen-node --dev
```
