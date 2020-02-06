# Lists: Maps vs Linked Maps
*[`kitchen/pallets/linked-map`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/linked-map)*

Substrate does not natively support a list type since it may encourage dangerous habits. Unless explicitly guarded against, a list will add unbounded `O(n)` complexity to an operation that will only charge `O(1)` fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)). This opens an economic attack vector on your chain.

Emulate a list with a mapping and a counter like so:

```rust, ignore
use support::{StorageValue, StorageMap};

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        TheList get(fn the_list): map u32 => T::AccountId;
        TheCounter get(fn the_counter): u32;
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
* [Linked Map for Simplified Enumeration](#linkedmap)

**Note**: it is important to properly handle [overflow/underflow](../declarative/safemath.md) and verify [other relevant conditions](../declarative) for safety

## Adding/Removing Elements in an Unbounded List <a name = "unbounded"></a>

If the size of the list is not relevant, the implementation is straightforward. To add an `AccountId`, increment the `the_counter` and insert an `AccountId` at that index:

```rust, ignore
fn add_member(origin) -> Result {
    let who = ensure_signed(origin)?;

    let new_count = <TheCounter<T>>::get() + 1;
    // insert new member at next highest index
    <TheList<T>>::insert(new_count, who.clone());
    // increment counter
    <TheCounter<T>>::put(new_count);

    Self::deposit_event(RawEvent::MemberAdded(who));

    Ok(())
}
```

To remove an `AccountId`, call the `remove` method for the `StorageMap` type at the relevant index. In this case, it isn't necessary to update the indices of other `proposal`s; order is not relevant.

```rust, ignore
fn remove_member_unbounded(origin, index: u32) -> Result {
    let who = ensure_signed(origin)?;

    // verify existence
    ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
    // for event emission
    let removed_member = <TheList<T>>::get(index);
    // remove member at provided index
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

```rust, ignore
fn remove_member_bounded(origin, index: u32) -> Result {
    let _ = ensure_signed(origin)?;

    ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");

    let largest_index = <TheCounter>::get();
    let member_to_remove = <TheList<T>>::take(index);
    // swap
    if index != largest_index {
        let temp = <TheList<T>>::take(largest_index);
        <TheList<T>>::insert(index, temp);
        <TheList<T>>::insert(largest_index, member_to_remove.clone());
    }
    // pop
    <TheList<T>>::remove(largest_index);
    <TheCounter>::mutate(|count| *count - 1);

    Self::deposit_event(RawEvent::MemberRemoved(member_to_remove.clone()));

    Ok(())
}
```

### Linked Map <a name = "linkedmap"></a>

To trade performance for *relatively* simple code, use the `linked_map` data structure. By implementing [`StorageLinkedMap`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageLinkedMap.html) in addition to [`StorageMap`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageMap.html), `linked_map` provides a method `head` which yields the head of the *list*, thereby making it unnecessary to also store the `LargestIndex` (the *counters*). The `enumerate` method also returns an `Iterator` ordered according to when `(key, value)` pairs were inserted into the map.

To use `linked_map`, import `EnumerableStorageMap`. Here is the new declaration in the `decl_storage` block:

```rust, ignore
use support::{StorageMap, EnumerableStorageMap}; // no StorageValue necessary

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        LinkedList get(fn linked_list): linked_map u32 => T::AccountId;
        LinkedCounter get(fn linked_counter): u32;
    }
}
```

The method adding members is no different than the previously covered method, but the `remove_member_linked` method expresses swap and pop in a different way

```rust, ignore
fn remove_member_linked(origin, index: u32) -> Result {
    let _ = ensure_signed(origin)?;

    ensure!(<LinkedList<T>>::exists(index), "A member does not exist at this index");

    let head_index = <LinkedList<T>>::head().unwrap();
    // swap
    let member_to_remove = <LinkedList<T>>::take(index);
    let head_member = <LinkedList<T>>::take(head_index);
    <LinkedList<T>>::insert(index, head_member);
    <LinkedList<T>>::insert(head_index, member_to_remove);
    // pop
    <LinkedList<T>>::remove(head_index);

    Ok(())
}
```

This implementation incurs some performance costs (vs solely using `StorageMap` and `StorageValue`) because `linked_map` heap allocates the entire map as an iterator in order to implement the [`enumerate` method](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageLinkedMap.html#tymethod.enumerate).
