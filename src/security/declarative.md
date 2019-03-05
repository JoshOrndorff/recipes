# Condition-Oriented Programming
> *Verify First, Write Last*

"As a developer building on Substrate, it is critical that you make a distinction about how you should design your runtime logic versus developing a smart contract on a platform like Ethereum.

On Ethereum, if at any point your transaction fails (error, out of gas, etc...), the state of your smart contract will be unaffected. However, on Substrate this is not the case. As soon as a transaction starts to modify the storage of the blockchain, those changes are permanent, even if the transaction would fail at a later time during runtime execution.

This is necessary for blockchain systems since you may want to track things like the nonce of a user or subtract gas fees for any computation that occurred. Both of these things actually happen in the Ethereum state transition function for failed transactions, but you have never had to worry about managing those things as a contract developer.

Now that you are a Substrate runtime developer, you will have to be conscious of any changes you make to the state of your blockchain, and ensure that it follows the "verify first, write last" pattern. We will be helping you do this throughout the tutorial." [SRC](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/tracking-all-kitties?id=quotverify-first-write-lastquot)

* [Checking for a Signed Message](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/storing-a-value?id=checking-for-a-signed-message)

### References

* [Condition-Oriented Programming](https://www.parity.io/condition-oriented-programming/)
* [Declarative Smart Contracts](https://www.tokendaily.co/blog/declarative-smart-contracts)

Put simply, COP aims to ensure that function bodies have no conditional paths or, alternatively, never mix transitions with conditions. By discouraging conditional paths from state-transitions, this approach limits the complexity of state-transitions, thereby allowing for facilitated auditability and better testing. 

More than two years later, James Prestwich published Declarative Smart Contracts reiterating the necessity of a functional approach to smart contract code patterns. In this post, Prestwich cites that "declarative contracts align the structure of the contract implementation with the reality of the chain by defining exactly what state modifications are permissible, and letting the user modify state directly. Declarative contracts prevent unintended state changes." This serves as just one of many examples in which Gavin Wood and, more broadly, the Parity team have identified the right approach to software engineering before the rest of the space.