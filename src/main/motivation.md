# Getting Started

If you're here, you probably already know about [Substrate](https://github.com/paritytech/substrate) and are ready to start building. If not, I recommend visiting the [official documentation](https://docs.substrate.dev/docs). For a high level overview, read these blog posts:

* [What is Substrate?](https://www.parity.io/what-is-substrate/)
* [Substrate in a nutshell](https://www.parity.io/substrate-in-a-nutshell/)
* [A brief summary of everything Substrate and Polkadot](https://www.parity.io/a-brief-summary-of-everything-substrate-polkadot/)

I've also written about Substrate on my [personal blog](https://4meta5.github.io/posts/ysubstrate)
## How to Use This Book

You can read the book chronologically or jump around. Personally, I prefer jumping around, but people learn in different ways :)

Regardless of the approach you take, it is useful to recognize that [coding is all about abstraction](https://youtu.be/05H4YsyPA-U?t=1789). To accelerate your progress, I recommend skimming the patterns in this book, composing them into interesting projects, and abstracting your own unique recipes. Feel free to reach out to me at <amar@parity.io> for guidance or, better yet, direct specific questions to the [Substrate technical channel](https://riot.im/app/#/room/#substrate-technical:matrix.org).
## Chef's Recommendations

My favorite recipes include

* [Mapping](../storage/mapping.md)
* [Structs](../storage/structs.md)
* [Safety First](../advanced/safety.md)
* [Incentive Design](../advanced/incentive.md)
* [Optimization Tricks](../advanced/optimizations.md)
### Notable Substrate Tutorials and Projects

Before anything else, I'd recommend starting with [Shawn's Collectables tutorial](https://github.com/shawntabrizi/substrate-collectables-workshop); it'll help you hit the ground running with an interactive sample project.

If you're interested in token-based mechanisms, look no further than [Gautam's Substrate TCR](https://github.com/parity-samples/substrate-tcr). The [full tutorial](https://docs.substrate.dev/docs/building-a-token-curated-registry-dappchain-using-substrate) will teach you Substrate best practices.

If you want to learn more about how to build novel blockchains with Substrate, check out [Nicole's utxo-workshop](https://github.com/nczhu/utxo-workshop). Preparing for [the workshop at Sub0](https://youtu.be/Q3hjtHaB3rA?t=7) taught me a lot of useful coding patterns in the context of Substrate (which are included in the advanced section e.g. [Incentive Design](../advanced/incentive.md), [Scheduling Collateralization](../advanced/lock.md), [Transaction Ordering](../advanced/ordering.md), and [Robust Conditional Paths](../advanced/conditionals.md)).

If you share an interest in cryptoeconomic mechanisms, check out [SunshineDAO](https://github.com/4meta5/SunshineDAO), a fund coordination mechanism on Substrate. The runtime demonstrates some interesting governance patterns such as those covered in the [Incentive Design](../advanced/incentive.md) recipe. Although the project is undergoing heavy refactoring, I am happy to onboard anyone interested in learning more (hmu at <amar@parity.io>). *You can also check out my talk at Sub0: [Building DAOs with Substrate](https://www.youtube.com/watch?v=eguDIG11nW8).*