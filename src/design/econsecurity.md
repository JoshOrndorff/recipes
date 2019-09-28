# Efficiency => Security in Substrate <a name = "sec"></a>

> Basically I need to make the point somewhere that efficiency influences security

We call an algorithm *efficient* if its running time is polynomial in the size of the input, and *highly efficient* if its running time is linear in the size of the input. It is important for all on-chain algorithms to be highly efficient, because they must scale linearly as the size of the Polkadot network grows. In contrast, off-chain algorithms are only required to be efficient. - [Web3 Research](http://research.web3.foundation/en/latest/polkadot/NPoS/1.intro/)

*See [Substrate Best Practices](https://substrate.dev/docs/en/tutorials/tcr/) for more details on how efficiency influences the runtime's economic security.*

**Related Reading**
* [Onwards; Underpriced EVM Operations](https://www.parity.io/onwards/), September 2016
* [Under-Priced DOS Attacks on Ethereum](https://www4.comp.polyu.edu.hk/~csxluo/DoSEVM.pdf)