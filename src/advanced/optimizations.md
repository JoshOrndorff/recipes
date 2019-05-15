# Runtime Optimization Tricks and Tips

> **This section is under heavy construction!**

Because Substrate is written in Rust, writing optimized Rust code reduces runtime overhead (costs) for Substrate deployments. Likewise, it is important to write clean, high-performance Rust code. Here, we include a few tips.

* [Technical Debt](#debt)
* [Efficiency => Security](#sec)
* [Ownership in Rust](#ownership)
* [Cooking with `unsafe`](#unsafe)
* [Concurrency vs Parallelism vs Asynchronous](#more)

**Inspired by and Pulling Heavily from**
* [Achieving Warp Speed with Rust]() by Jack Fransham, [`troubles.md`]()
* [High Performance Rust]() by ____

## Technical Debt <a name = "debt"></a>

* first thing's first => build a simple, readable implementation with sound logic; then consider optimization...

* important to mention that managing technical debt is a design criteria
* the world changes and code changes; readability => maintainability => scalability of the codebase...

* [Rust API Guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html)
* [Elegant Library API Guidelines]() -- CITE PASCAL's SCRIBBLES

## Efficiency => Security in Substrate <a name = "sec"></a>

We call an algorithm *efficient* if its running time is polynomial in the size of the input, and *highly efficient* if its running time is linear in the size of the input. It is important for all on-chain algorithms to be highly efficient, because they must scale linearly as the size of the Polkadot network grows. In contrast, off-chain algorithms are only required to be efficient. [src](http://research.web3.foundation/en/latest/polkadot/NPoS/1.intro/)

* [Attack on Cheap Operations on Ethereum; September 2016]()
* [Recent Attack Post-Mortem]()

* [Iterate Through a Slice Rather than a `vec!`](#iter)

### Iterate Through a Slice Rather than a Vec! <a name = "iter"></a> 

It's noticeably faster to iterate over a slice rather than a `vec! `.
* `.iter.map(|x| x.0.into()).collect`

## Ownership in Rust <a name = "ownership"></a> 

> "The *Rust learning curve* is [learning] borrow checking." ~[Rust at Speed -- Building a Fast, Concurrent Database](https://www.youtube.com/watch?v=s19G6n0UjsM) by Jon Gjengset

If you *own* something, you are responsible for that thing; if you own memory, you are responsible for freeing that memory when it is responsible to do so. Rust extends this notion of ownership such that if you own something, you get to choose who has access to that resource and how.

For some type `T`, you can have: 
1. `T` (**owned**)
2. `&mut T` (**exclusive**)
3. `&T` (**shared**)

Rust will at compile time check that you haven't violated the contracts. For any two variables, you cannot have a mutable reference and immutable reference at the same time. You also can't have multiple mutable references.

Likewise, the compiler requires proof that you don't have data races. **If you never have something modify a thing while it's being read or modified, you cannot have data races.** So, you'll either have multiple readers or a single writer.

You also guarantee that you can only free things because the owner is responsible for freeing things and you only have one owner. The borrowchecker also checks that you haven't used anything after you've gotten rid of it.

The borrowchecker does this by adding this notion of a lifetime. If you borrow any `T`, that borrow of `T` is assigned a lifetime. You can think of the lifetime as how long you're allowed to access `T` for -- for example, if `T` lives on the stack, the lifetime that's given out for any borrow of `T` is going to be tied to the stack frame of the thing that has `T`. The compiler will check that when `T` goes away => that stack frame is popped and `T` is freed, there are no oustanding references to `T`. So if you tried to take a reference to something stored to the stack and give it to another thread, the compiler would say no, that thread can live longer than this stack frame so your program is not safe and will not compile.

We can frame concurrency problems in terms of ownership. Rust forces us to use synchronization.

**Lifetimes** are just regions of code, and regions of code can be partially ordered with the *contains* (outlives) relationship. For lifetimes, the bigger region is a subtype of the smaller region (because the subtype has at minimum the lifetime of its supertype). The `'static` lifetime is a subtype of every lifetime (because it outlives everything).

In the end, using lifetimes in Rust refers to efficient management of the scope of references. When we see a `struct`, `closure`, or `enum` with lifetime parameters, the lifetimes refer to the fields of the given data structure, not the structure itself.

A few useful things to know
* multiple lifetimes can satisfy some given lifetime parameter and the compiler will take the minimal one
* simple Rust types do not have *subtyping* unless they have lifetime parameters

Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

1. each parameter that is a reference gets its own lifetime parameter
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters

### Borrowing Degradations <a name = "degrade"></a>

Rust's borrow checker has three simple rules:
1. each binding (`name => value`) will have an owner
2. there can only one owner for a binding
3. when the owner goes out of scope, the binding is dropped

With this in mind, it is possible to do 3 things with variables when passing them to a function: 
1. send a reference (borrow)
2. give the new function control of the variable (own)
3. copy/clone the variable to send it to a function

Therefore, we offer these rules of thumb
* if you no longer require ownership of the variable, transfer it to the function
* if you still require it, send a reference
* if you require it and the API only accepts ownership, clone it

For specific function inputs, we can invoke the following rules of thumb:
* *If it's <= size(`usize`) => copy*
* *if `usize` < `_size_` < 10 * `usize` => probably copy*
* bigger than that => reference

**NOTE**: the greater the cyclomatic complexity of the code, the more difficult it is for the compiler to optimize the logic -- it is *recommended* to create functions with no greater than 20-25 branches each (each branch represents a conditional like an `if` or `match` or `?`).

## Hell's Kitchen: Using `unsafe` <a name = "unsafe"></a>

> Rust Romicon....introduce it here...

We'll go over one common pattern.

If we have to get an element in a specific position, then we should use the `get()` method. This maintains a double bound check.

```rust
for bar in barray_of_arrays {
    if let Some(foo) = bar.iter().get(173) {
        println!("{}", foo);
    }
}
```

However, this `.get()` call has a double bound check. It will first check if the index is correct to return a `Some(foo)` or `None`, and then the final check will verify that the returned element is `Some` or `None`.

If we have verified bound checking independently for the call, we can use `.getunchecked()` to get the element. Although this is unsafe to use, it is exactly equivalent to the C/C++ indexing operation, thereby allowing for higher performance when we know the element's location. Indeed, if we don't verify what we feed to get `unchecked`, an attacker could hypothetically access whatever is stored in the location even if it was a memory address outside the slice.

```rust
for bar in barray_of_arrays {
    // verify independently that 173 is before the end of the array
    println!("{}", unsafe { bar.iter().getunchecked(173)});
}
```

## Concurrency vs Parallelism vs Asynchronous <a name = "more"></a>
> where did I get this from? Maybe the Lock-Free rust in 2019 post; cite it?

...use that post by `aturon` to introduce these concepts...

* **Rayon** splits your data into distinct pieces, gives each piece to a thread to do some kind of computation on it, and finally aggregates results. Its goal is to distribute CPU-intensive tasks onto a thread pool.
* **Tokio** runs tasks which sometimes need to be paused in order to wait for asynchronous events. Handling tons of such tasks is no problem. Its goal is to distribute IO-intensive tasks onto a thread pool.
* **Crossbeam** is all about low-level concurrency: atomics, concurrent data structures, synchronization primitives. Same idea as the `std::sync` module, but bigger. Its goal is to provide tools on top of which libraries like Rayon and Tokio can be built.

**Asynchronous**
* [Introduction to Async/Await Programming (withoutboats/wakers-i):](https://boats.gitlab.io/blog/post/wakers-i/)
* [Aaron Turon](http://aturon.github.io/2016/08/11/futures/)