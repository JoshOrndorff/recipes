# Off-chain Workers

> Before learning how to build your own off-chain worker, you may want to learn about what off-chain workers are, why you want to use them, and what kinds of problems they can solve best. These topics are covered in [our guide](https://substrate.dev/docs/en/conceptual/core/off-chain-workers). Here, we will focus on using off-chain workers in Substrate.

Off-chain workers contain a set of powerful tools allowing your Substrate node to offload tasks that take too long or too much CPU / memory resources to compute, or have non-deterministic result. In particular we have a set of helpers allowing fetching of HTTP requests and using a community-contributed tool for parsing the returned JSON. It also provides its own storage that is unique to the particular off-chain worker node and not synchronized across the network.

Once the off-chain computation is completed, off-chain workers can submit either signed or unsigned transactions back on-chain.

We will deep-dive into each of the topics below.

- [Signed and Unsigned Transactions](./transactions.md)
- [HTTP fetching and JSON parsing](./http-json.md)
- Storage in Off-chain Workers (wip)
