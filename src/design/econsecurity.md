# Efficiency => Security in Substrate <a name = "sec"></a>

An algorithm is considered to be *efficient* if its running time is polynomial in the size of the input, and *highly efficient* if its running time is linear in the size of the input. **It is important for all on-chain algorithms to be highly efficient, because they must scale linearly as the size of the Polkadot network grows**. In contrast, off-chain algorithms are only required to be efficient. - [Web3 Research](http://research.web3.foundation/en/latest/polkadot/NPoS/1.intro/)

Moreover, any resources used by a transaction must explicitly be paid for within the module. If the resources used might be dependent on transaction parameters or pre-existing chain state, the in-module fee structure must adapt accordingly. Specifically, measuring the balance between **resources used** and **price paid** is an important design activity for runtime security.

*Indeed, mispriced EVM operations have shown how operations that underestimate cost can open economic DOS attack vectors: [Onwards; Underpriced EVM Operations](https://www.parity.io/onwards/), [Under-Priced DOS Attacks on Ethereum](https://www4.comp.polyu.edu.hk/~csxluo/DoSEVM.pdf)*

<!-- ## todo

* existing transaction fee

* more fee structure examples -->