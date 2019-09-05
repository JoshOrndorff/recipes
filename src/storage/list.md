# Maps

To use maps in the runtime storage, first import `StorageMap` from [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/)

```rust
use support::{StorageMap};
```

With this type, a key-value mapping (between `u32` types) can be stored in runtime storage using the following syntax

```rust
decl_storage! {
	trait Store for Module<T: Trait> as Example {
		MyMap: map u32 => u32;
	}
}
```

Functions used to access a `StorageValue` are defined in [`srml/support`](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs):

```rust
/// Get the prefix key in storage.
fn prefix() -> &'static [u8];

/// Get the storage key used to fetch a value corresponding to a specific key.
fn key_for(x: &K) -> Vec<u8>;

/// true if the value is defined in storage.
fn exists<S: Storage>(key: &K, storage: &S) -> bool {
    storage.exists(&Self::key_for(key)[..])
}

/// Load the value associated with the given key from the map.
fn get<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Take the value under a key.
fn take<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Store a value to be associated with the given key from the map.
fn insert<S: Storage>(key: &K, val: &V, storage: &S) {
    storage.put(&Self::key_for(key)[..], val);
}

/// Remove the value under a key.
fn remove<S: Storage>(key: &K, storage: &S) {
    storage.kill(&Self::key_for(key)[..]);
}

/// Mutate the value under a key.
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(key: &K, f: F, storage: &S) -> R;
```

To insert a `(key, value)` pair into a `StorageMap` named `MyMap`:

```rust
<MyMap<T>>::insert(key, value);
```

To query `MyMap` for the `value` corresponding to a `key`:

```rust
let value = <MyMap<T>>::get(key);
```

## Implementing Lists with Maps

Substrate does not natively support a list type since it may encourage dangerous habits. Unless explicitly guarded against, a list will add unbounded `O(n)` complexity to an operation that will only charge `O(1)` fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)). This opens an economic attack vector on your chain.

Emulate a list with a mapping and a counter like so:

```rust
use support::{StorageValue, StorageMap};

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        TheList get(the_list): map u32 => T::AccountId;
        TheCounter get(the_counter): u32;
    }
}
```

This code allows us to store a list of participants in the runtime represented by `AccountId`s. Of course, this implementation leaves many unanswered questions such as
* How to add and remove elements?
* How to maintain order under mutating operations?
* How to verify that an element exists before removing/mutating it?

This recipe answers those questions with snippets from relevant code samples:
* [Adding/Removing Elements in an Unordered List](#unbounded)
* [Swap and Pop for Ordered Lists](#swappop)
* [Linked Map for Simplified Runtime Logic](#linkedmap)

<!-- **Note**: it is important to properly handle [overflow/underflow](../advanced/safety.md#overunder) and verify [other relevant conditions](../advanced/safety.md#check) when invoking this recipe -->

## Adding/Removing Elements in an Unbounded List <a name = "unbounded"></a>

If the size of the list is not relevant, the implementation is straightforward. *Note how it is still necessary to verify the existence of elements in the map before attempting access.* 

To add an `AccountId`, increment the `the_count` and insert an `AccountId` at that index:

```rust
// decl_module block
fn add_member(origin) -> Result {
    let who = ensure_signed(origin)?;

    // increment the counter
    <TheCounter<T>>::mutate(|count| *count + 1);

    // add member at the largest_index
    let largest_index = <TheCounter<T>>::get();
    <TheList<T>>::insert(largest_index, who.clone());

    Self::deposit_event(RawEvent::MemberAdded(who));

    Ok(())
} 
```

To remove an `AccountId`, call the `remove` method for the `StorageMap` type at the relevant index. In this case, it isn't necessary to update the indices of other `proposal`s; order is not relevant.

```rust
// decl_module block
fn remove_member_unbounded(origin, index: u32) -> Result {
    let who = ensure_signed(origin)?;

    // verify existence
    ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
    let removed_member = <TheList<T>>::get(index);
    <TheList<T>>::remove(index);

    Self::deposit_event(RawEvent::MemberRemoved(removed_member));

    Ok(())
}
```

Because the code doesn't update the indices of other `AccountId`s in the map, it is necessary to verify an `AccountId`'s existence before removing it, mutating it, or performing any other operation.

## Swap and Pop for Ordered Lists <a name = "swappop"></a>

To preserve storage so that the list doesn't continue growing even after removing elements, invoke the **swap and pop** algorithm:
1. swap the element to be removed with the element at the head of the *list* (the element with the highest index in the map)
2. remove the element recently placed at the highest index
3. decrement the `TheCount` value. 

Use the *swap and pop* algorithm to remove elements from the list.

```rust
// decl_module block
fn remove_member_ordered(origin, index: u32) -> Result {
    let who = ensure_signed(origin)?;

    ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");

    let largest_index = <TheCounter<T>>::get();
    let member_to_remove = <TheList<T>>::take(index);
    // swap
    if index != largest_index {
    let temp = <TheList<T>>::take(largest_index);
    <TheList<T>>::insert(index, temp);
    <TheList<T>>::insert(largest_index, member_to_remove.clone());
    }
    // pop
    <TheList<T>>::remove(largest_index);
    <TheCounter<T>>::mutate(|count| *count - 1);

    Self::deposit_event(RawEvent::MemberRemoved(member_to_remove.clone()));

    Ok(())
}
```

*Keep the same logic for inserting proposals (increment `TheCount` and insert the entry at the head of the list)* 

### Linked Map <a name = "linkedmap"></a>

To trade performance for *relatively* simple code, utilize the `linked_map` data structure. By implementing [`EnumarableStorageMap`](https://crates.parity.io/srml_support/storage/trait.EnumerableStorageMap.html) in addition to [`StorageMap`](https://crates.parity.io/srml_support/storage/trait.StorageMap.html), `linked_map` provides a method `head` which yields the head of the *list*, thereby making it unnecessary to also store the `LargestIndex`. The `enumerate` method also returns an `Iterator` ordered according to when `(key, value)` pairs were inserted into the map.

To use `linked_map`, import `EnumerableStorageMap`. Here is the new declaration in the `decl_storage` block:

```rust
use support::{StorageMap, EnumerableStorageMap}; // no StorageValue necessary

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        LinkedList get(linked_list): linked_map u32 => T::AccountId;
        LinkedCounter get(linked_counter): u32;
    }
}
```

The `add_member_linked` method is logically equivalent to the previous `add` method. Here is the new `remove_member_linked` method:

```rust
// decl_module block
fn remove_member_linked(origin, index: u32) -> Result {
    let who = ensure_signed(origin)?;

    ensure!(<LinkedList<T>>::exists(index), "A member does not exist at this index");

    let head_index = <LinkedList<T>>::head().unwrap();
    let member_to_remove = <LinkedList<T>>::take(index);
    let head_member = <LinkedList<T>>::get(head_index);
    <LinkedList<T>>::insert(index, head_member);
    <LinkedList<T>>::remove(head_index);

    Ok(())
}
```

The only caveat is that this implementation incurs some performance costs (vs solely using `StorageMap` and `StorageValue`) because `linked_map` heap allocates the entire map as an iterator in order to implement the [`enumerate` method](https://crates.parity.io/srml_support/storage/trait.EnumerableStorageMap.html#tymethod.enumerate).