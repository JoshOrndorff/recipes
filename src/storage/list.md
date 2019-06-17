# Implementing Lists with Maps

Substrate does not natively support a list type since it may encourage dangerous habits. Unless explicitly guarded against, a list will add unbounded `O(n)` complexity to an operation that will only charge `O(1)` fees ([Big O notation refresher](https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/)). This opens an economic attack vector on your chain. *To learn more about economic security, see [Safety First](../advanced/safety.md).*

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

**Note**: it is important to properly handle [overflow/underflow](../advanced/safety.md#overunder) and verify [other relevant conditions](../advanced/safety.md#check) when invoking this recipe

## Adding/Removing Elements in an Unbounded List <a name = "unbounded"></a>

If the size of the list is not relevant, the implementation is straightforward. 

For example, let's say that there is a list of `proposal`s (maybe defined as a struct in the runtime). When a `proposal` expires, remove it from the list, but it is not necessary to update the indices of other `proposal`s that have been added. *Note that it is still necessary to [verify the existence](../advanced/safety.md#collision) of proposals in the map before attempting access.* 

Store the `proposal`s in a key-value mapping

```rust
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
struct Proposal<Hash> {
    details: Hash,
}

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        Proposals get(proposals): map u32 => Proposal<T::Hash>;
        LargestIndex get(largest_index): u32;
    }
}
```

To add a `proposal`, increment the `largest_index` and insert a `proposal` at that index:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // other methods

        fn add_proposal(hash: Hash) -> Result {
            // any necessary checks here

            // instantiate new proposal
            let prop = Proposal { details: hash.clone() };

            // increment largest index
            <LargestIndex<T>>::mutate(|count| count + 1);

            // add a proposal at largest_index
            let largest_index = Self::largest_index::get();
            <Proposals<T>>::insert(largest_index, prop);

            Ok(());
        }
    }
}
```

To remove a `proposal`, call the `remove` method for the `StorageMap` type at the relevant index. In this case, it isn't necessary to update the indices of other `proposal`s; order is not relevant.

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // other methods

        fn remove_proposal(index: u32) -> Result {
            // any necessary checks here

            // remove proposal at the given index
            <Proposals<T>>::remove(index);

            Ok(());
        }
    }
}
```

Because the code doesn't update the indices of other `proposal`s in the map, it is necessary to verify a proposal's existence before removing it, mutating it, or performing any other operation.

```rust
// index is the `u32` that corresponds to the proposal in the `<Proposals<T>>` map
ensure!(<Proposals<T>>::exists(index), "proposal does not exist at the provided index");
```

## Swap and Pop for Ordered Lists <a name = "swappop"></a>

To preserve storage so that the list doesn't continue growing even after removing elements, invoke the **swap and pop** algorithm:
1. swap the element to be removed with the element at the head of the *list* (the element with the highest index in the map)
2. remove the element recently placed at the highest index
3. decrement the `LargestIndex` value. 

Use the *swap and pop* algorithm to remove elements from the list.

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // other methods

        fn remove_proposal(index: u32) -> Result {
            // check that a proposal exists at the given index
            ensure!(<Proposals<T>>::exists(index), "A proposal does not exist at this index");

            let largest_index = Self::largest_index::get();
            let proposal_to_remove = <Proposals<T>>::take(index);
            // swap
            if index != largest_index {
                let temp = <Proposals<T>>::take(largest_index);
                <Proposals<T>>::insert(index, temp);
                <Proposals<T>>::insert(largest_index, proposal_to_remove);
            }
            // pop
            <Proposals<T>>::remove(largest_index);
            <LargestIndex<T>>::mutate(|count| count - 1);

            Ok(());
        }
    }
}
```

*Keep the same logic for inserting proposals (increment `LargestIndex` and insert the entry at the head of the list)* 

### Linked Map <a name = "linkedmap"></a>

To trade performance for simpler code, utilize the `linked_map` data structure. By implementing [`EnumarableStorageMap`](https://crates.parity.io/srml_support/storage/trait.EnumerableStorageMap.html) in addition to [`StorageMap`](https://crates.parity.io/srml_support/storage/trait.StorageMap.html), `linked_map` provides a method `head` which yields the head of the *list*, thereby making it unnecessary to also store the `LargestIndex`. The `enumerate` method also returns an `Iterator` ordered according to when `(key, value)` pairs were inserted into the map.

To use `linked_map`, import `EnumerableStorageMap`. Here is the new declaration in the `decl_storage` block:

```rust
use support::{StorageMap, EnumerableStorageMap}; // no StorageValue necessary

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        Proposals get(proposals): linked_map u32 => Proposal<T::Hash>;
        // no largest_index value necessary
    }
}
```

Here is the new `remove_proposal` method:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // other methods

        fn remove_proposal(index: u32) -> Result {
            // check that a proposal exists at the given index
            ensure!(<Proposals<T>>::exists(index), "A proposal does not exist at this index");

            let head_index = Self::proposals::head();
            let proposal_to_remove = <Proposals<T>>::take(index);
            <Proposals<T>>::insert(index, <Proposals<T>>::get(head_index));
            <Proposals<T>>::remove(head_index);

            Ok(());
        }
    }
}
```

The only caveat is that this implementation incurs some performance costs (vs solely using `StorageMap` and `StorageValue`) because `linked_map` heap allocates the entire map as an iterator in order to implement the [`enumerate` method](https://crates.parity.io/srml_support/storage/trait.EnumerableStorageMap.html#tymethod.enumerate).