# Learn Rust

To be productive with [substrate](https://github.com/substrate) requires some familiarity with Rust. Fortunately, the Rust community is known for comprehensive documentation and tutorials. The most common resource for initially learning Rust is [The Rust Book](https://doc.rust-lang.org/book/index.html). To see examples of popular crate usage patterns, [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) is also convenient.

## API Design

To become more familiar with commmon design patterns in Rust, the following links might be helpful:
* [Official Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
* [Rust Unofficial Design Patterns](https://github.com/rust-unofficial/patterns)
* [Elegant Library API Guidelines](https://deterministic.space/elegant-apis-in-rust.html)

## Optimizations

To optimize runtime performance, Substrate developers should make use of iterators, traits, and Rust's other "*zero cost* abstractions":
* [Abstraction without overhead: traits in Rust](https://blog.rust-lang.org/2015/05/11/traits.html), [related conference talk](https://www.youtube.com/watch?v=Sn3JklPAVLk)
* [Effectively Using Iterators in Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
* [Achieving Warp Speed with Rust](http://troubles.md/posts/rust-optimization/)

It is not (immediately) necessary to become familiar with multithreading because the runtime operates in a [single-threaded context](https://www.tutorialspoint.com/single-threaded-and-multi-threaded-processes). Even so, the runtime might take advantage of the [offchain workers API](https://substrate.dev/docs/en/next/overview/off-chain-workers) to minimize the computation executed on-chain. Effectively using these features requires increased familiarity with advanced Rust.

For a high-level overview of concurrency in Rust, Stjepan Glavina provides the following descriptions in [Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html):
* **[Rayon](https://github.com/rayon-rs/rayon)** splits your data into distinct pieces, gives each piece to a thread to do some kind of computation on it, and finally aggregates results. Its goal is to distribute CPU-intensive tasks onto a thread pool.
* **[Tokio](https://github.com/tokio-rs/tokio)** runs tasks which sometimes need to be paused in order to wait for asynchronous events. Handling tons of such tasks is no problem. Its goal is to distribute IO-intensive tasks onto a thread pool.
* **[Crossbeam](https://github.com/crossbeam-rs/crossbeam)** is all about low-level concurrency: atomics, concurrent data structures, synchronization primitives. Same idea as the `std::sync` module, but bigger. Its goal is to provide tools on top of which libraries like Rayon and Tokio can be built.

### Asynchrony
[Are we `async` yet?](https://areweasyncyet.rs/)

**Conceptual**
* [Introduction to Async/Await Programming (withoutboats/wakers-i)](https://boats.gitlab.io/blog/post/wakers-i/)
* [Futures (by Aaron Turon)](https://aturon.github.io/tech/2016/08/11/futures/)
* [RustLatam 2019 - Without Boats: Zero-Cost Async IO](https://www.youtube.com/watch?v=skos4B5x7qE)

**Projects**
* [Rust Asynchronous Ecosystem Working Group](https://github.com/rustasync)
* [romio](https://github.com/withoutboats/romio)
* [Tokio Docs](https://tokio.rs/docs/overview/)

### Concurrency

**Conceptual**
* [Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html)
* [Crossbeam Research Meta-link](https://github.com/crossbeam-rs/rfcs/wiki)
* [Rust Concurrency Explained](https://www.youtube.com/watch?v=Dbytx0ivH7Q)

**Projects**
* [sled](https://github.com/spacejam/sled)
* [servo](https://github.com/servo/servo)
* [TiKV](https://github.com/tikv/tikv)
