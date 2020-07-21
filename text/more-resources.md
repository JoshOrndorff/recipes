# More Resources

## Substrate

Learn more about Substrate from these resources:

-   [Substrate Developer Hub Home](https://substrate.dev) - Landing page of the official Substrate
    documentation.
-   [Conceptual Docs](https://substrate.dev/docs) - Explanation of Substrate's architecture at a
    high level of abstraction.
-   [Reference Docs](https://substrate.dev/rustdocs) - Documentation on specific Substrate APIs with
    little abstraction.
-   [Substrate Tutorials](https://substrate.dev/tutorials) - Step by step guides to accomplish
    specific tasks with Substrate.

<!-- Reminder: There is a _lot_ more potential content for this section in drafts/dessert.md -->

## Rust

Once you've got the fundamentals of Substrate, it can only help to know more rust. Here is a
collection of helpful docs and blog posts to take you down the rabbit hole.

### API Design

To become more familiar with commmon design patterns in Rust, the following links might be helpful:

-   [Official Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
-   [Rust Unofficial Design Patterns](https://github.com/rust-unofficial/patterns)
-   [Elegant Library API Guidelines](https://deterministic.space/elegant-apis-in-rust.html)

### Optimizations

To optimize runtime performance, Substrate developers should make use of iterators, traits, and
Rust's other "_zero cost_ abstractions":

-   [Abstraction without overhead: traits in Rust](https://blog.rust-lang.org/2015/05/11/traits.html),
    [related conference talk](https://www.youtube.com/watch?v=Sn3JklPAVLk)
-   [Effectively Using Iterators in Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
-   [Achieving Warp Speed with Rust](http://troubles.md/posts/rust-optimization/)

### Concurrency

-   **[Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html)**
    a high-level overview of concurrency in Rust.
-   **[Rayon](https://github.com/rayon-rs/rayon)** splits your data into distinct pieces, gives each
    piece to a thread to do some kind of computation on it, and finally aggregates results. Its goal
    is to distribute CPU-intensive tasks onto a thread pool.
-   **[Tokio](https://github.com/tokio-rs/tokio)** runs tasks which sometimes need to be paused in
    order to wait for asynchronous events. Handling tons of such tasks is no problem. Its goal is to
    distribute IO-intensive tasks onto a thread pool.
-   **[Crossbeam](https://github.com/crossbeam-rs/crossbeam)** is all about low-level concurrency:
    atomics, concurrent data structures, synchronization primitives. Same idea as the `std::sync`
    module. Its goal is to provide tools on top of which libraries like Rayon and Tokio can be
    built.

### Asynchrony

[Are we `async` yet?](https://areweasyncyet.rs/)

**Conceptual**

-   [Introduction to Async/Await Programming (withoutboats/wakers-i)](https://boats.gitlab.io/blog/post/wakers-i/)
-   [Futures (by Aaron Turon)](https://aturon.github.io/tech/2016/08/11/futures/)
-   [RustLatam 2019 - Without Boats: Zero-Cost Async IO](https://www.youtube.com/watch?v=skos4B5x7qE)

**Projects**

-   [Rust Asynchronous Ecosystem Working Group](https://github.com/rustasync)
-   [romio](https://github.com/withoutboats/romio)
-   [Tokio Tutorials](https://tokio.rs/tokio/tutorial)

### Concurrency

**Conceptual**

-   [Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html)
-   [Crossbeam Research Meta-link](https://github.com/crossbeam-rs/rfcs/wiki)
-   [Rust Concurrency Explained](https://www.youtube.com/watch?v=Dbytx0ivH7Q)

**Projects**

-   [sled](https://github.com/spacejam/sled)
-   [servo](https://github.com/servo/servo)
-   [TiKV](https://github.com/tikv/tikv)
