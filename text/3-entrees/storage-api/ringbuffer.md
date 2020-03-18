# Ringbuffer Queue
*[`pallets/ringbuffer-queue`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue)*
> Building a transient adapter on top of storage.

This pallet provides a trait and implementation for a [ringbuffer](https://en.wikipedia.org/wiki/Circular_buffer) that abstracts over storage items and presents them as a [FIFO](https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics)) queue.

When building more sophisticated pallets there might develop a need for more complex data structures stored in storage. This recipe shows how to build a transient storage adapter by walking through the implementation of a ringbuffer FIFO queue. The adapter in this recipe manages a queue that is persisted as a `StorageMap` and a `(start, end)` range in storage.

## Defining the RingBuffer Trait <a name = "trait"></a>
First we define the queue interface we want to use:

```rust, ignore
pub trait RingBufferTrait<Item>
where
	Item: Codec + EncodeLike,
{
	/// Store all changes made in the underlying storage.
	fn commit(&self);
	/// Push an item onto the end of the queue.
	fn push(&mut self, i: Item);
	/// Pop an item from the start of the queue.
	fn pop(&mut self) -> Option<Item>;
	/// Return whether the queue is empty.
	fn is_empty(&self) -> bool;
}
```

It defines the usual `push`, `pop` and `is_empty` functions we expect from a queue as well as a `commit` function that will be used to sync the changes made to the underlying storage.

## Implementing the RingBuffer Transient <a name = "transient"></a>
Now we want to add an implementation of the trait.
```rust, ignore
pub struct RingBufferTransient<Index>
where
	Index: Codec + EncodeLike + Eq + Copy,
{
	start: Index,
	end: Index,
}
```

## The WrappingOps Trait <a name = "wrapping_ops"></a>
The ringbuffer should be agnostic to the concrete `Index` type used. In order to be able to decrement and increment the start and end index, though, any concrete type needs to implement `wrapping_add` and `wrapping_sub`. Because `std` does not provide such a trait we need another way to require this behavior. One possbility would be using the [`num_traits` crate](https://crates.io/crates/num-traits), but to keep things simple here we just implement our own trait `WrappingOps` for the types we want to support (`u8`, `u16`, `u32` and `u64`).

## Typical Usage