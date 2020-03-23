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

## Specifying the RingBuffer Transient <a name = "transient"></a>
Now we want to add an implementation of the trait. We will be storing the start and end of the ringbuffer separately from the actual items and will thus need to store these in our struct:

```rust, ignore
pub struct RingBufferTransient<Index>
where
	Index: Codec + EncodeLike + Eq + Copy,
{
	start: Index,
	end: Index,
}
```

### Defining the Storage Interface
In order to access the underlying storage we will also need to include the bounds (we will call the type `B`) and the item storage (whose type will be `M`). In order to specify the constraints on the storage map (`M`) we will also need to specify the `Item` type.
This results in the following struct definition:

```rust, ignore
pub struct RingBufferTransient<Item, B, M, Index>
where
	Item: Codec + EncodeLike,
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	M: StorageMap<Index, Item, Query = Item>,
	Index: Codec + EncodeLike + Eq + Copy,
{
	start: Index,
	end: Index,
	_phantom: PhantomData<(Item, B, M)>,
}
```

The bounds `B` will be a `StorageValue` storing a tuple of indices `(Index, Index)`. The item storage will be a `StorageMap` mapping from our `Index` type to the `Item` type. We specify the associated `Query` type for both of them to help with type inference (because the value returned can be different from the stored representation).

The `Codec` and `EncodeLike` type constraints make sure that both items and indices can be stored in storage.

We need the `PhantomData` in order to "hold on to" the types during the lifetime of the transient object.

### The Complete Type
There are two more alterations we need to make to our struct to make it work well:

```rust, ignore
type DefaultIdx = u16;
/// Transient backing data that is the backbone of the trait object.
pub struct RingBufferTransient<Item, B, M, Index = DefaultIdx>
where
	Item: Codec + EncodeLike,
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	M: StorageMap<Index, Item, Query = Item>,
	Index: Codec + EncodeLike + Eq + WrappingOps + From<u8> + Copy,
{
	start: Index,
	end: Index,
	_phantom: PhantomData<(Item, B, M)>,
}
```

We specify a default type for `Index` and define it as `u16` to allow for 65536 entries in the ringbuffer per default. We also add the `WrappingOps` and `From<u8>` type bounds to enable the kind of operations we need in our implementation. More details in the [`WrappingOps`](#wrapping_ops) and [implementation](#implementation) sections.

### The WrappingOps Trait <a name = "wrapping_ops"></a>
The ringbuffer should be agnostic to the concrete `Index` type used. In order to be able to decrement and increment the start and end index, though, any concrete type needs to implement `wrapping_add` and `wrapping_sub`. Because `std` does not provide such a trait we need another way to require this behavior. One possbility would be using the [`num_traits` crate](https://crates.io/crates/num-traits), but to keep things simple here we just implement our own trait `WrappingOps` for the types we want to support (`u8`, `u16`, `u32` and `u64`).

## Implementation of the RingBuffer <a name = "implementation"></a>
Now that we have the type definition for `RingBufferTransient` we need to write the implementation.

### Instantiating the Transient
First we need to specify how to create a new instance by providing a `new` function:

```rust, ignore
impl<Item, B, M, Index> RingBufferTransient<Item, B, M, Index>
where // ... same where clause as the type, elided here
{
	/// Create a new `RingBufferTransient` that backs the ringbuffer implementation and initializes itself from the bounds storage `B`.
	pub fn new() -> RingBufferTransient<Item, B, M, Index> {
		let (start, end) = B::get();
		RingBufferTransient {
			start, end, _phantom: PhantomData,
		}
	}
}
```

Here we access the bounds stored in storage to initialize the transient.

_Aside: Of course we could also provide a `with_bounds` function that takes the bounds as a parameter. Feel free to add that function as an exercise._

_Second Aside: This `B::get()` is the reason for the specifying the `Query` associated type on the `StorageValue` type constraint._

### Implementing the `RingBufferTrait`
We will now implement the `RingBufferTrait`
```rust, ignore
impl<Item, B, M, Index> RingBufferTrait<Item> for RingBufferTransient<Item, B, M, Index>
where
	Item: Codec + EncodeLike,
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	M: StorageMap<Index, Item, Query = Item>,
	Index: Codec + EncodeLike + Eq + WrappingOps + From<u8> + Copy,
```

## Typical Usage