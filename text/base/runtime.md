# Runtime Configuration
*[`kitchen/runtimes/super-runtime`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/runtimes/super-runtime)* *[`kitchen/runtimes/weight-fee-runtime`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/runtimes/weight-fee-runtime)*

when using FRAME, the runtime gives context to the pallets. Indeed, each pallet is configured in the runtime with explicit assignment of the types declared in its [Configuration `Trait`](https://substrate.dev/rustdocs/master/pallet_example/trait.Trait.html).

Each runtime in the kitchen demonstrates configuring and composing pallets in this way.
