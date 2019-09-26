# Modules

- [ ] **UPDATE WITH DESCRIPTIONS AND BUILDS SHOWN**

**Event**: effectively logging, scheduling, and reacting to events defined in `decl_event`
* [Adding Machine](./modules/adder/)
* [Simple Event (not generic)](./modules/simple-event)
* [Generic Event](./modules/generic-event)

**Storage**: managing interactions with the on-chain storage via `decl_storage`
* [Single Value Storage](./modules/value)
* [Simple Map](./modules/simple-map)
* [List](./modules/list)
* [Double Map](./modules/double-map)
* [Child Trie](./modules/child-trie)
* [Offchain Workers](./modules/offchain-workers)

**Traits and Types**: using substrate traits and types
* [Module Inheritance](./modules/inherit)
* [Configurable Module Constants](./modules/constants/)
- [Nested Structs](./nstructs)

**Examples**: usage examples of the above patterns *with context*
- Currency Types and Locking Techniques::{[lockable](./lockable), [reservable](./reservable), [imbalances](./imbalances)}
- [Token Transfer](./token)
- [Permissioned Methods](./permissioned)
- [Blockchain Event Loop](./loop)
- [Social Network](./social)