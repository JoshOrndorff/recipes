# Ringbuffer Queue

`pallets/ringbuffer-queue`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fringbuffer-queue%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

> Building a transient adapter on top of storage.

This pallet provides a trait and implementation for a
[ringbuffer](https://en.wikipedia.org/wiki/Circular_buffer) that abstracts over storage items and
presents them as a [FIFO](<https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics)>) queue.

When building more sophisticated pallets you might notice a need for more complex data structures
stored in storage. This recipe shows how to build a transient storage adapter by walking through the
implementation of a ringbuffer FIFO queue. The adapter in this recipe manages a queue that is
persisted as a `StorageMap` and a `(start, end)` range in storage.

The
[`ringbuffer-queue/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue/src/lib.rs)
file contains the [usage](#usage) of the transient storage adapter while
[`ringbuffer-queue/src/ringbuffer.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue/src/ringbuffer.rs)
contains the implementation.

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

It defines the usual `push`, `pop` and `is_empty` functions we expect from a queue as well as a
`commit` function that will be used to sync the changes made to the underlying storage.

## Specifying the RingBuffer Transient <a name = "transient"></a>

Now we want to add an implementation of the trait. We will be storing the start and end of the
ringbuffer separately from the actual items and will thus need to store these in our struct:

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

In order to access the underlying storage we will also need to include the bounds (we will call the
type `B`) and the item storage (whose type will be `M`). In order to specify the constraints on the
storage map (`M`) we will also need to specify the `Item` type. This results in the following struct
definition:

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

The bounds `B` will be a `StorageValue` storing a tuple of indices `(Index, Index)`. The item
storage will be a `StorageMap` mapping from our `Index` type to the `Item` type. We specify the
associated `Query` type for both of them to help with type inference (because the value returned can
be different from the stored representation).

The [`Codec`](https://docs.rs/parity-scale-codec/1.3.0/parity_scale_codec/trait.Codec.html) and
[`EncodeLike`](https://docs.rs/parity-scale-codec/1.3.0/parity_scale_codec/trait.EncodeLike.html)
type constraints make sure that both items and indices can be stored in storage.

We need the [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) in order
to "hold on to" the types during the lifetime of the transient object.

### The Complete Type

There are two more alterations we will make to our struct to make it work well:

```rust, ignore
type DefaultIdx = u16;
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

We specify a default type for `Index` and define it as `u16` to allow for 65536 entries in the
ringbuffer per default. We also add the `WrappingOps` and `From<u8>` type bounds to enable the kind
of operations we need in our implementation. More details in the [implementation](#implementation)
section, especially in the [`WrappingOps`](#wrapping_ops) subsection.

## Implementation of the RingBuffer <a name = "implementation"></a>

Now that we have the type definition for `RingBufferTransient` we need to write the implementation.

### Instantiating the Transient

First we need to specify how to create a new instance by providing a `new` function:

```rust, ignore
impl<Item, B, M, Index> RingBufferTransient<Item, B, M, Index>
where // ... same where clause as the type, elided here
{
	pub fn new() -> RingBufferTransient<Item, B, M, Index> {
		let (start, end) = B::get();
		RingBufferTransient {
			start, end, _phantom: PhantomData,
		}
	}
}
```

Here we access the bounds stored in storage to initialize the transient.

> **Aside**: Of course we could also provide a `with_bounds` function that takes the bounds as a
> parameter. Feel free to add that function as an exercise.

> **Second Aside**: This `B::get()` is one of the reasons for specifying the `Query` associated type
> on the `StorageValue` type constraint.

### Implementing the `RingBufferTrait`

We will now implement the `RingBufferTrait`:

```rust, ignore
impl<Item, B, M, Index> RingBufferTrait<Item> for RingBufferTransient<Item, B, M, Index>
where // same as the struct definition
	Item: Codec + EncodeLike,
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	M: StorageMap<Index, Item, Query = Item>,
	Index: Codec + EncodeLike + Eq + WrappingOps + From<u8> + Copy,
{
	fn commit(&self) {
		B::put((self.start, self.end));
	}
```

`commit` just consists of putting the potentially changed bounds into storage. You will notice that
we don't update the bounds' storage when changing them in the other functions.

```rust, ignore
	fn is_empty(&self) -> bool {
		self.start == self.end
	}
```

The `is_empty` function just checks whether the start and end bounds have the same value to
determine whether the queue is empty, thus avoiding expensive storage accesses. This means we need
to uphold the corresponding invariant in the other (notably the `push`) functions.

```rust, ignore
	fn push(&mut self, item: Item) {
		M::insert(self.end, item);
		// this will intentionally overflow and wrap around when bonds_end
		// reaches `Index::max_value` because we want a ringbuffer.
		let next_index = self.end.wrapping_add(1.into());
		if next_index == self.start {
			// queue presents as empty but is not
			// --> overwrite the oldest item in the FIFO ringbuffer
			self.start = self.start.wrapping_add(1.into());
		}
		self.end = next_index;
	}
```

In the `push` function, we insert the pushed `item` into the map and calculate the new bounds by
using the `wrapping_add` function. This way our ringbuffer will wrap around when reaching
`max_value` of the `Index` type. This is why we need the `WrappingOps` type trait for `Index`.

The `if` is necessary because we need to keep the invariant that `start == end` means that the queue
is empty, otherwise we would need to keep track of this state separately. We thus "toss away" the
oldest item in the queue if a new item is pushed into a full queue by incrementing the start index.

> ##### Note: The `WrappingOps` Trait <a name = "wrapping_ops"></a>
>
> The ringbuffer should be agnostic to the concrete `Index` type used. In order to decrement and
> increment the start and end index, though, any concrete type needs to implement `wrapping_add` and
> `wrapping_sub`. Because `std` does not provide such a trait, we need another way to require this
> behavior. We just implement our own trait `WrappingOps` for the types we
> want to support (`u8`, `u16`, `u32` and `u64`).

The last function we implement is `pop`:

```rust, ignore
	fn pop(&mut self) -> Option<Item> {
		if self.is_empty() {
			return None;
		}
		let item = M::take(self.start);
		self.start = self.start.wrapping_add(1.into());

		item.into()
	}
```

We can return `None` on `is_empty` because we are upholding the invariant. If the queue is not empty
we `take` the value at `self.start` from storage, i.e. the first value is removed from storage and
passed to us. We then increment `self.start` to point to the new first item of the queue, again
using the `wrapping_add` to get the ringbuffer behavior.

### Implementing Drop

In order to make the usage more ergonomic and to avoid synchronization errors (where the storage map
diverges from the bounds) we also implement the
[`Drop` trait](https://doc.rust-lang.org/std/ops/trait.Drop.html):

```rust, ignore
impl<Item, B, M, Index> Drop for RingBufferTransient<Item, B, M, Index>
where // ... same where clause elided
{
	fn drop(&mut self) {
		<Self as RingBufferTrait<Item>>::commit(self);
	}
}
```

On `drop`, we `commit` the bounds to storage. With this implementation of `Drop`, `commit` is called
when our transient goes out of scope, making sure that the storage state is consistent for the next
call to the using pallet.

## Typical Usage <a name = "usage"></a>

The
[`lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue/src/lib.rs)
file of the pallet shows typical usage of the transient.

```rust, ignore
impl<T: Config> Module<T> {
	fn queue_transient() -> Box<dyn RingBufferTrait<ValueStruct>> {
		Box::new(RingBufferTransient::<
			ValueStruct,
			<Self as Store>::BufferRange,
			<Self as Store>::BufferMap,
			BufferIndex,
		>::new())
	}
}
```

First we define a constructor function (`queue_transient`) so we don't have to specify the types
every time we want to access the transient. This function constructs a ringbuffer transient and
returns it as a boxed trait object. See the Rust book's section on
[trait objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)
for an explanation of why we need a boxed trait object (defined with the syntax `dyn TraitName`)
when using dynamic dispatch.

The `add_multiple` function shows the actual typical usage of our transient:

```rust, ignore
pub fn add_multiple(origin, integers: Vec<i32>, boolean: bool) -> DispatchResult {
	let _user = ensure_signed(origin)?;
	let mut queue = Self::queue_transient();
	for integer in integers {
		queue.push(ValueStruct{ integer, boolean });
	}
	Ok(())
} // commit happens on drop
```

Here we use the `queue_transient` function defined above to get a `queue` object. We then `push`
into it repeatedly with `commit` happening on `drop` of the `queue` object at the end of the
function. `pop` works analogously and can of course be intermixed with `push`es.
