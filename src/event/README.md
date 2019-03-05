# Event

In Substrate, [transaction](https://docs.substrate.dev/docs/glossary#section-transaction) finality does not guarantee the execution of functions dependent on the given transaction. To verify that functions have executed successfully, emit an [event](https://docs.substrate.dev/docs/glossary#section-events) at the end of the function.

> **Events** notify the off-chain world of successful state transitions

To declare an event, use the [`decl_event`](https://github.com/paritytech/substrate/blob/HEAD/srml/example/src/lib.rs#L78) macro. Check out the [Dummy](./event/basic.md) event for more information regarding file structure and necessary inclusions in the runtime root `lib.rs`.

## Recipes <a name ="recipes"></a>

* [Dummy Event](./event/basic.md)
* [Adding Machine](./event/adder.md)
* [Permissioned Generic Event](./event/permissioned.md)

## Examples in the <a href="https://github.com/paritytech/substrate/tree/master/srml">SRML Source Code</a>

* [SRML EXAMPLES HERE](https://wiki.parity.io/decl_event)

> [Creating an Event](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/creating-an-event)

> TCR relevant code

### TODO

* clean up existing examples and format in a coherent way

*Other Patterns* (use sourcegraph)
* check TCR
* check Collectables
* check SRML
* off-chain patterns (Substrate event listener)

*Use SourceGraph*
* include page on the srml examples of `decl_event!`
* annotate a few and link a bunch of others