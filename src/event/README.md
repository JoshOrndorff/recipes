# Event

In Substrate, [transaction](https://docs.substrate.dev/docs/glossary#section-transaction) finality does not guarantee the execution of functions dependent on the given transaction. To verify that functions have executed successfully, emit an [event](https://docs.substrate.dev/docs/glossary#section-events) at the bottom of the function body.

> **Events** notify the off-chain world of successful state transitions

To declare an event, use the [`decl_event`](https://crates.parity.io/srml_support/macro.decl_event.html) macro.

## Recipes

* [Simple Adding Machine](./adder.md)
* [Membership Authentication](./permissioned.md)
* [Blockchain Event Loop](./loop.md)

## More Resources

* [`decl_event` wiki docs](https://wiki.parity.io/decl_event)
* [Substrate Collectables Tutorial: Creating Events](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/creating-an-event)