# Pallet Development Rules

* add pallet development criteria here

## Logic Proofs <a name = "qed"></a>

Because Substrate grants bare-metal control to developers, certain code patterns can expose panics at runtime. As mentioned in (2) of [Pallet Development Criteria](#criteria), panics can cause irreversible storage changes, possibly even bricking the blockchain and rendering it useless. 

It is the responsibility of Substrate developers to ensure that the code doesn't panics after storage changes. In many cases, safety might be independently verified by the developer while writing the code. To facilitate auditability and better testing, Substrate developers should include a proof in an `.expect()` call that shows why the code's logic is safe and will not panic. Convention dictates formatting the call like so

```rust, ignore
<Object<T>>::method_call().expect("<proof of safety>; qed");
```

You can find more examples of this pattern in the [Substrate codebase](https://github.com/paritytech/substrate/search?q=expect). Indeed, including logic proofs is very important for writing readable, well-maintained code. It comes as no surprise that this pattern is also discussed in the [Substrate collectables tutorial](https://shawntabrizi.com/substrate-collectables-workshop/#/3/buying-a-kitty?id=remember-quotverify-first-write-lastquot).

> *QED stands for Quod Erat Demonstrandum which loosely translated means "that which was to be demonstrated"*
