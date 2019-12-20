# Set Storage and Iteration
*[`kitchen/pallets/vec-set`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/pallets/vec-set)*

Storing a vector in the runtime can often be useful for managing groups and verifying membership. This recipe discusses common patterns encounted when storing vectors in runtime storage.

* [verifying group membership](#group)
* [Append vs Mutate](#append)
* [Iteration in the Runtime](#iterate)

## Verifying Group Membership <a name = "group"></a>

To maintain a set of `AccountId` to establish group ownership of decisions, it is straightforward to store a vector in the runtime of `AccountId`.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
        Members get(fn members): Vec<T::AccountId>;
	}
}
```

It is easy to add the following helper method to verify membership elsewhere in the runtime.

```rust, ignore
impl<T: Trait> Module<T> {
    fn is_member(who: &T::AccountId) -> bool {
        <Members<T>>::get().contains(who)
    }
}
```

This helper method can be placed in other runtime methods to restrict certain changes to runtime storage to privileged groups. Depending on the incentive structure of the network/chain, the members in these groups may have earned membership and the subsequent access rights through loyal contributions to the system.

```rust, ignore
// use support::ensure
fn member_action(origin) -> Result {
    let member = ensure_signed(origin)?;
    ensure!(Self::is_member(&member), "not a member => cannot do action");
    // <action && || storage change>
    Ok(())
}
```

In this example, the helper method facilitates isolation of runtime storage access rights according to membership. In general, **place `ensure!` checks at the top of each runtime function's logic to verify that all of the requisite checks pass before performing any storage changes.**

> NOTE: *[child trie](https://github.com/substrate-developer-hub/recipes/issues/35) storage provides a more efficient data structure for tracking group membership*

## Append vs. Mutate

```rust, ignore
decl_storage! {
	trait Store for Module<T: Trait> as VecMap {
	    CurrentValues get(fn current_values): Vec<u32>;
        NewValues get(fn new_values): Vec<u32>;
	}
}
```

Before [3071](https://github.com/paritytech/substrate/pull/3071) was merged, it was necessary to call [`mutate`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageValue.html#tymethod.mutate) to push new values to a vector stored in runtime storage.

```rust, ignore
fn mutate_to_append(origin) -> Result {
    let user = ensure_signed(origin)?;

    // this decodes the existing vec, appends the new values, and re-encodes the whole thing
    <CurrentValues>::mutate(|v| v.extend_from_slice(&Self::new_values()));
    Self::deposit_event(RawEvent::MutateToAppend(user));
    Ok(())
}
```

For vectors stored in the runtime, mutation can be relatively expensive. This follows from the fact that `mutate` entails decoding the vector, making changes, and re-encoding the whole vector. It seems wasteful to decode the entire vector, push a new item, and then re-encode the whole thing. This provides sufficient motivation for [`append`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageValue.html#tymethod.append):


```rust, ignore
fn append_new_entries(origin) -> Result {
    let user = ensure_signed(origin)?;

    // this encodes the new values and appends them to the already encoded existing evc
    let mut current_values = Self::current_values();
    current_values.append(&mut Self::new_values());
    Self::deposit_event(RawEvent::AppendVec(user));
    Ok(())
}
```

`append` encodes the new values, and pushes them to the already encoded vector without decoding the existing entries. This method removes the unnecessary steps for decoding and re-encoding the unchanged elements.

## Iteration in the Runtime <a name = "iterate"></a>

In general, iteration in the runtime should be avoided. *In the future*, [offchain-workers](https://github.com/substrate-developer-hub/recipes/issues/45) may provide a less expensive way to iterate over runtime storage items. Moreover, *[child tries](https://github.com/substrate-developer-hub/recipes/issues/35)* enable cheap inclusion proofs without the same lookup costs associated with vectors.

Even so, there are a few tricks to alleviate the costs of iterating over runtime storage items like vectors. For example, it is [cheaper to iterate over a slice](https://twitter.com/heinz_gies/status/1121490424739303425) than a vector. With this in mind, store items in the runtime as vectors and transform them into slices after making storage calls. [3041](https://github.com/paritytech/substrate/pull/3041) introduced `insert_ref` and `put_ref` in order to allow equivalent reference-style types to be placed without copy (e.g. a storage item of `Vec<AccountId>` can now be written from a `&[AccountId]`). This enables greater flexibility when working with slices that are associated with vectors stored in the runtime.
