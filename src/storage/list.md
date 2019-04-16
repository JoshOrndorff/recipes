# Imlementing Lists with Maps

Substrate does not natively support a list type since it may encourage dangerous habits. Unless explicitly guarded against, a list will add unbounded `O(N)` complexity to an operation that will only charge `O(1)` fees. This opens an economic attack vector on your chain. **To learn more, check out [the section on economic security]().**

Fortunately, we can emulate a list with a mapping and a counter like so:

```rust
use support::{StorageValue, StorageMap};

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        TheList get(the_list): map u32 => T::AccountId;
        TheCounter get(the_counter): u32;
    }
}
```

This code allows us to store a list of participants in our runtime represented by `AccountId`s. Of course, this implementation leaves many unanswered questions such as
* How to add and remove elements?
* How to maintain order under mutating operations?
* How to verify that an element exists before removing/mutating it?

This section strives to answer those questions with snippets from relevant code samples:
* [Adding/Removing Elements in an Unordered List]()
* [Swap and Pop for Ordered Lists]()
    * [Using Linked Map for Simplified Runtime Logic]()
* [Using Double Map]()
* [Necessary Underflow/Overflow Checks]()

## Adding/Removing Elements in an Unbounded List

If the size of our list is not relevant to how we access data, the implementation is straightforward. 

For example, let's say that we have a list of `proposal`s (defined as a struct in our runtime). When a `proposal` expires, we remove it from our list, but it is not necessary to update the indices of other `proposal`s that have been added (*if* we perform checks that a proposal exists in the map before accessing it). 

We can store our `proposal`s in a key-value mapping similar to the initial example:

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

To add a `proposal`, we would increment the `largest_index` and add a `proposal` at that index:

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

            Ok();
        }
    }
}
```

To remove a `proposal`, we can simple invoke the `remove` method for the `StorageMap` type at the relevant index. In this case, we do not need to update the indices of other `proposal`s. This is because order does not matter for this sample.

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // other methods

        fn remove_proposal(index: u32) -> Result {
            // any necessary checks here

            // remove proposal at the given index
            <Proposals<T>>::remove(index);

            Ok();
        }
    }
}
```

Because we are not updating the indices of other `proposal`s in our map, we have to check that a proposal exists before removing it, mutating it, or performing any other relevant operation.

```rust
// index is the `u32` that corresponds to the proposal in the `<Proposals<T>>` map
ensure!(<Proposals<T>>::exists(index), "proposal does not exist at the provided index");
```

For a more extensive and complete example of this pattern, see [SunshineDAO](https://github.com/AmarRSingh/SunshineDAO/runtime/src/dao.rs).

## Swap and Pop for Ordered Lists

When we want to preserve storage such that our list doesn't continue growing even after we remove elements, we can invoke the **swap and pop** method:
1. swap the element to be removed with the element at the head of our *list* (the element with the highest index in our map)
2. remove the element recently placed at the highest index
3. decrement the `LargestIndex` value. 

Continuing with our example, we maintain the same logic for adding proposals (increment `LargestIndex` and insert entry at the head of our *list*).  However, we invoke the *swap and pop* algorithm when removing elements from our list:

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

            Ok();
        }
    }
}
```

### linked_map

To trade performance for simpler code, utilize the `linked_map` data structure. By implementing [`EnumarableStorageMap`](https://crates.parity.io/srml_support/storage/trait.EnumerableStorageMap.html) in addition to [`StorageMap`](https://crates.parity.io/srml_support/storage/trait.StorageMap.html), `linked_map` provides a method `head` which yields the head of the *list*, thereby making it unnecessary to also store the `LargestIndex`. The `enumerate` method also returns an `Iterator` ordered according to when (key, value) pairs were inserted into the map.

To use `linked_map`, we also need to import `EnumerableStorageMap`. Here is the new declaration in the `decl_storage` block:

```rust
use support::{StorageMap, EnumerableStorageMap}; // no StorageValue necessary

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        Proposals get(proposals): linked_map u32 => Proposal<T::Hash>;
        // no largest_index value necessary
    }
}
```

Here is our new `remove_proposal` method

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

            Ok();
        }
    }
}
```

The only caveat is that this implementation incurs some performance costs (vs solely using `StorageMap` and `StorageValue`) because `linked_map` heap allocates the entire map as an iterator in order to implement the [`enumerate` method](https://crates.parity.io/srml_support/storage/trait.EnumerableStorageMap.html#tymethod.enumerate).

<!-- * double_map -->

<!-- ## Necessary Underflow/Overflow Checks

* put in section on economic security (not sure where to put this...)
* **add something on checking for underflowing and overflowing**
* just follow Shawn's tutorial for this... -->