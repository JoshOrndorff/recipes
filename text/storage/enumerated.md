# Lists: Maps vs Linked Maps
*[`pallets/linked-map`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/linked-map)*

Substrate does not natively support a list type since it may encourage dangerous habits. Unless explicitly guarded against, a list will add unbounded `O(n)` complexity to an operation that will only charge `O(1)` fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)). This opens an economic attack vector on your chain.

Emulate a list with a mapping and a counter like so:

```rust, ignore
use support::{StorageValue, StorageMap};

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        TheList get(fn the_list): map u32 => T::AccountId;
        LargestIndex get(fn largest_index): u32;
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

If the contiguousness of the list is not relevant (i.e. you are willing to allow gaps and check for them at access time), the implementation is straightforward. To add an `AccountId`, increment the `LargestIndex` and insert an `AccountId` at that index:

```rust, ignore
fn add_member(origin) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Note: We use a 1-based (instead of 0-based) list here
    // Note: Handle overflow here in production code!
    let new_count = <LargestIndex>::get() + 1;
    // insert new member past the end of the list
    <TheList<T>>::insert(new_count, &who);
    // store the incremented count
    <LargestIndex>::put(new_count);

    Self::deposit_event(RawEvent::MemberAdded(who));

    Ok(())
}
```

To remove an `AccountId`, call the `remove` (to drop it) or `take` (to use it) method for the `StorageMap` type at the relevant index. In this case, it isn't necessary to update the indices of other `member`s; contiguousness is not relevant.

```rust, ignore
fn remove_member_discontiguous(origin, index: u32) -> DispatchResult {
    let _ = ensure_signed(origin)?;

    // verify existence
    ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");
    // use take for event emission, use remove to drop value
    let removed_member = <TheList<T>>::take(index);

    Self::deposit_event(RawEvent::MemberRemoved(removed_member));

    Ok(())
}
```

Because the code doesn't update the indices of other `AccountId`s in the map, it is necessary to verify an `AccountId`'s existence before removing it, mutating it, or performing any other operation.

## Swap and Pop for Ordered Lists <a name = "swappop"></a>

To preserve contiguousness so that the list does not contain gaps even after removing elements, invoke the **swap and pop** algorithm:
1. swap the element to be removed with the element at the head of the *list* (the element with the highest index in the map)
2. remove the element recently placed at the highest index
3. decrement the `LargestIndex` value.

Use the *swap and pop* algorithm to remove elements from the list.

```rust, ignore
fn remove_member_contiguous(origin, index: u32) -> DispatchResult {
    let _ = ensure_signed(origin)?;

    ensure!(<TheList<T>>::exists(index), "an element doesn't exist at this index");

    let largest_index = <LargestIndex>::get();
    // swap
    if index != largest_index {
        <TheList<T>>::swap(index, largest_index);
    }
    // pop, uses `take` to return the member in the event
    let removed_member = <TheList<T>>::take(largest_index);
    <LargestIndex>::mutate(|count| *count - 1);

    Self::deposit_event(RawEvent::MemberRemoved(removed_member));

    Ok(())
}
```

### Linked Map <a name = "linkedmap"></a>

If you are willing to trade some storage overhead for *relatively* simple code, use the `linked_map` data structure. By implementing [`StorageLinkedMap`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageLinkedMap.html) in addition to [`StorageMap`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageMap.html), `linked_map` provides a method `head` which yields the head of the *list*, thereby making it unnecessary to also store the `LargestIndex`. The `enumerate` method also returns an `Iterator` ordered according to when `(key, value)` pairs were inserted into the map.

To use `linked_map`, import `StorageLinkedMap`. Here is the new declaration in the `decl_storage` block:

```rust, ignore
use frame_support::{StorageMap, StorageLinkedMap}; // no StorageValue necessary

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        TheLinkedList get(fn linked_list): linked_map u32 => T::AccountId;
    }
}
```

The method adding members is no different than the previously covered method, but the `remove_member_linked` method expresses shows that you don't need "swap and pop" any more:

```rust, ignore
fn remove_member_linked(origin, index: u32) -> DispatchResult {
    let _ = ensure_signed(origin)?;

    ensure!(<TheLinkedList<T>>::exists(index), "A member does not exist at this index");
    let removed_member = <TheLinkedList<T>>::take(index);

    Self::deposit_event(RawEvent::MemberRemoved(removed_member));

    Ok(())
}
```

This implementation incurs some storage costs (vs solely using `StorageMap` and `StorageValue`) because `linked_map` stores `previous` and `next` key for every value.
