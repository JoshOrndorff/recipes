# Economic Security, Best Practices

Developing in the context Substrate's runtime can *feel* foreign to many developers from other platforms and languages because it *is* very different. Following Rust conventions, Substrate optimizes for performance while also providing modularity and extensibility. At the same time, the fine-grain control provided by Substrate requires increased attention from the developer who can often solve problems in more than one way.

* modules are indepdent and should not have external dependencies
* they optimize for performance and trade composability

* [Stay Aware of Runtime Costs](#costs)

## Programming for Conditional Paths

* Using `ensure!` as much as possible
* sometimes doing checks in their own scopes is best -- link to the `unique.md` file

* do all the checks before all the function calls
* make sure a panic does not and cannot occur before a function change
    * use the qed with `.expect()` to really show that this is done correctly

In late June 2016, Gavin Wood published a post on [Condition-Oriented Programming (COP)](https://www.parity.io/condition-oriented-programming/), a hybrid approach between functional and imperative programming. Put simply, COP aims to ensure that function bodies have no conditional paths or, alternatively, never mix transitions with conditions. By discouraging conditional paths from state-transitions, this approach limits the complexity of state-transitions, thereby allowing for facilitated auditability and better testing. More than two years later, James Prestwich published [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts) reiterating the necessity of a *functional* approach to smart contract code patterns. In this post, Prestwich cites that "declarative contracts align the structure of the contract implementation with the reality of the chain by defining exactly what state modifications are permissible, and letting the user modify state directly. Declarative contracts prevent unintended state changes." 

## Robust In-Module Fee Structure <a name = "costs"></a>

Substrate developers need to **stay cognizant of the price paid for resource usage** within the runtime. This can be unintuitive for smart contract developers coaxed into abstracting out the cost of individual operations by modern smart contract development platforms. Indeed, Substrate optimizes for performance. Likewise, Substrate;s in-module fee structure is delicate, and any resource used by a transaction must explicitly be paid for within the module. For more details, check out [the tcr tutorial's best practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices): *If the resources used might be dependent on transaction parameters or pre-existing on-chain state, then your in-module fee structure must adapt accordingly.*

So how do we design a robust in-module fee structure? In the [`utxo-workshop`](https://github.com/nczhu/utxo-workshop), the difference between inputs and outputs for valid transactions is distributed evenly among the authority set. This pattern demonstrates one approach for incentivizing validation via a floating transaction fee which varies in cost according to the value of the native currency and the relative size/activity of the validator set.

