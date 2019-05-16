# Optimization Tricks

Runtime overhead in Substrate corresponds to the efficiency of the underlying Rust code. Therefore, it is essential to use clean, efficient Rust patterns for performance releases. This section introduces common approaches for optimizing Rust code in general and links to resources that may guide further investigation.

* [Premature Optimization](#premature)
* [Efficiency => Security](#sec)
* [Zero-Cost Abstractions](#zero)
* [Entering `unsafe` Waters 🏴‍☠️](#unsafe)
* [Fearless Concurrency && Asynchrony](#more)

**This section was inspired by and pulls heavily from**
* [Achieving Warp Speed with Rust](http://troubles.md/posts/rust-optimization/) by Jack Fransham, [`troubles.md`](http://troubles.md/)
* [High Performance Rust](https://www.packtpub.com/application-development/rust-high-performance) by Iban Eguia Moraza

## Premature Optimization <a name = "premature"></a>

*Programmers waste enormous amounts of time thinking about, or worrying about, the speed of noncritical parts of their programs, and these attempts at efficiency actually have a strong negative impact when debugging and maintenance are considered. We should forget about small efficiencies, say about 97% of the time: premature optimization is the root of all evil.* - Page 268 of [Structured Programming with `goto` Statements](http://wiki.c2.com/?StructuredProgrammingWithGoToStatements) by Donald Knuth

Before worrying about performance optimizations, focus on *optimizing* for readability, simplicity, and maintainability. The first step when building anything is achieving basic functionality. Only after establishing a minimal viable sample is it appropriate to consider performance-based enhancements. With that said, severe inefficiency does open attack vectors for Substrate runtimes (*see [the next section](#sec)*). Moreover, the tradeoff between optimization and simplicity is not always so clear... 

*A common misconception is that optimized code is necessarily more complicated, and that therefore optimization always represents a trade-off. However, in practice, better factored code often runs faster and uses less memory as well. In this regard, optimization is closely related to refactoring, since in both cases we are paying into the code so that we may draw back out again later if we need to.* - http://wiki.c2.com/?PrematureOptimization

**Rust API Guidelines**
* [Official Rust API Guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html)
* [Rust Unofficial Design Patterns](https://github.com/rust-unofficial/patterns)
* [Elegant Library API Guidelines](https://deterministic.space/elegant-apis-in-rust.html) by Pascal Hertleif

Also, use [clippy](https://github.com/rust-lang/rust-clippy)!

## Efficiency => Security in Substrate <a name = "sec"></a>

We call an algorithm *efficient* if its running time is polynomial in the size of the input, and *highly efficient* if its running time is linear in the size of the input. It is important for all on-chain algorithms to be highly efficient, because they must scale linearly as the size of the Polkadot network grows. In contrast, off-chain algorithms are only required to be efficient. - [Web3 Research](http://research.web3.foundation/en/latest/polkadot/NPoS/1.intro/)

*See [Substrate Best Practices](https://docs.substrate.dev/docs/tcr-tutorial-best-practices) for more details on how efficiency influences the runtime's economic security.*

**Related Reading**
* [Onwards; Underpriced EVM Operations](https://www.parity.io/onwards/), September 2016
* [Under-Priced DOS Attacks on Ethereum](https://www4.comp.polyu.edu.hk/~csxluo/DoSEVM.pdf)

## Rust Zero-Cost Abstractions <a name = "zero"></a>

Substrate developers should take advantage of Rust's zero cost abstractions.

*Articles*
* [Abstraction without overhead: traits in Rust](https://rust-embedded.github.io/book/static-guarantees/zero-cost-abstractions.html)
* [Effectively Using Iterators in Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
* [Type States](https://rust-embedded.github.io/book/static-guarantees/zero-cost-abstractions.html)

*Tweets*
* [iterate over a slice rather than a `vec!`](https://twitter.com/heinz_gies/status/1121490424739303425)

*Video*
* [An introduction to structs, traits, and zero-cost abstractions](https://www.youtube.com/watch?v=Sn3JklPAVLk)

## Entering `unsafe` Waters 🏴‍☠️  <a name = "unsafe"></a>

*Please read [The Rustonomicon](https://doc.rust-lang.org/nomicon/) before experimenting with the dark magic that is `unsafe`*

To access an element in a specific position, use the `get()` method. This method performs a double bound check.

```rust
for arr in array_of_arrays {
    if let Some(elem) = arr.iter().get(1738) {
        println!("{}", elem);
    }
}
```

The `.get()` call performs two checks:
1. checks that the index will return `Some(elem)` or `None`
2. checks that the returned element is of type `Some` or `None`

If bound checking has already been performed independently of the call, we can invoke `.getunchecked()` to access the element. Although this is `unsafe` to use, it is equivalent to C/C++ indexing, thereby improving performance when we already know the element's location.

```rust
for arr in array_of_arrays {
    println!("{}", unsafe { arr.get_unchecked(1738) })
}
```

**NOTE**: if we don't verify the input to `.getunchecked()`, the caller may access whatever is stored in the location even if it is a memory address outside the slice

## Fearless Concurrency && Asynchrony <a name = "more"></a>

As a systems programming language, Rust provides significant flexibility with respect to low-level optimizations. Specifically, Rust provides fine-grain control over how you perform computation, delegate said computation to the OS's threads, and schedule state transitions within a given thread. There isn't space in this book to go into significant detail, but I'll try to provide resources/reading that have helped me get up to speed. For a high-level overview, Stjepan Glavina provides the following descriptions in [Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html):

* **[Rayon](https://github.com/rayon-rs/rayon)** splits your data into distinct pieces, gives each piece to a thread to do some kind of computation on it, and finally aggregates results. Its goal is to distribute CPU-intensive tasks onto a thread pool.
* **[Tokio](https://github.com/tokio-rs/tokio)** runs tasks which sometimes need to be paused in order to wait for asynchronous events. Handling tons of such tasks is no problem. Its goal is to distribute IO-intensive tasks onto a thread pool.
* **[Crossbeam](https://github.com/crossbeam-rs/crossbeam)** is all about low-level concurrency: atomics, concurrent data structures, synchronization primitives. Same idea as the `std::sync` module, but bigger. Its goal is to provide tools on top of which libraries like Rayon and Tokio can be built.

To dive deeper down these 🐰 holes
* [Asynchrony](#async)
* [Concurrency](#concurrency)

### Asynchrony <a name = "async"></a>
[Are we `async` yet?](https://areweasyncyet.rs/)

**Conceptual**
* [RustLatam 2019 - Without Boats: Zero-Cost Async IO](https://www.youtube.com/watch?v=skos4B5x7qE)
* [Introduction to Async/Await Programming (withoutboats/wakers-i):](https://boats.gitlab.io/blog/post/wakers-i/)
* [Futures (by Aaron Turon)](http://aturon.github.io/2016/08/11/futures/)

**Projects**
* [Rust Asynchronous Ecosystem Working Group](https://github.com/rustasync)
* [romio](https://github.com/withoutboats/romio)
* [Tokio Docs](https://tokio.rs/docs/overview/)

### Concurrency <a name = "concurrency"></a>

**Conceptual**
* [Rust Concurrency Explained](https://www.youtube.com/watch?v=Dbytx0ivH7Q)
* [Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html)
* [Crossbeam Research Meta-link](https://github.com/crossbeam-rs/rfcs/wiki)

**Projects**
* [sled](https://github.com/spacejam/sled)
* [servo](https://github.com/servo/servo)
* [TiKV](https://github.com/tikv/tikv)