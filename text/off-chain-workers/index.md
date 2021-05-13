# Off-chain Workers

> Here we focus on building off-chain workers in Substrate. To read more about what off-chain
> workers are, why you want to use them, and what kinds of problems they solve best. Please goto
> [our guide](https://substrate.dev/docs/en/knowledgebase/learn-substrate/off-chain-features#off-chain-workers).

Off-chain workers allow your Substrate node to offload tasks that take too long or too much CPU /
memory resources to compute, or have non-deterministic result. In particular there are a set of
helpers allowing fetching of HTTP requests and parsing for JSON. It also provides storage that is
specific to the particular Substrate node and not shared across the network. Off-chain workers can
also submit either signed or unsigned transactions back on-chain.

We will deep-dive into each of the topics below.

- [Signed and Unsigned Transactions](./transactions.md)
- [HTTP fetching and JSON parsing](./http-json.md)
- [Local storage in Off-chain Workers](./storage.md)
- [Off-chain Indexing](./indexing.md)
