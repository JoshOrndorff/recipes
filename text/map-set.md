# Using Maps as Sets

`pallets/map-set`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fmap-set%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/map-set/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

A [Set](<https://en.wikipedia.org/wiki/Set_(abstract_data_type)>) is an unordered data structure
that stores entries without duplicates. Substrate's storage API does not provide a way to declare
sets explicitly, but they can be implemented using either vectors or maps.

This recipe shows how to implement a storage set on top of a map, and explores the performance of
the implementation. When implementing a set in your own runtime, you should compare this technique
to implementing a [`vec-set`](./vec-set.md).

In this pallet we implement a set of `AccountId`s. We do not use the set for anything in this
pallet; we simply maintain its membership. Using the set is demonstrated in the recipe on
[pallet coupling](./pallet-coupling.md). We provide dispatchable calls to add and remove members,
ensuring that the number of members never exceeds a hard-coded maximum.

```rust, ignore
/// A maximum number of members. When membership reaches this number, no new members may join.
pub const MAX_MEMBERS: u32 = 16;
```

## Storage Item

We will store the members of our set as the keys in one of Substrate's
[`StorageMap`](https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageMap.html)s. There is also
a recipe specifically about [using storage maps](./storage-maps.md). The storage map itself does not
track its size internally, so we introduce a second storage value for this purpose.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config> as VecMap {
		// The set of all members.
		Members get(fn members): map hasher(blake2_128_concat) T::AccountId => ();
		// The total number of members stored in the map.
		// Because the map does not store its size, we must store it separately
		MemberCount: u32;
	}
}
```

The _value_ stored in the map is `()` because we only care about the keys.

## Adding Members

Any user may join the membership set by calling the `add_member` dispatchable, so long as they are
not already a member and the membership limit has not been reached. We check for these two
conditions first, and then insert the new member only after we are sure it is safe to do so.

```rust, ignore
fn add_member(origin) -> DispatchResult {
	let new_member = ensure_signed(origin)?;

	let member_count = MemberCount::get();
	ensure!(member_count < MAX_MEMBERS, Error::<T>::MembershipLimitReached);

	// We don't want to add duplicate members, so we check whether the potential new
	// member is already present in the list. Because the membership is stored as a hash
	// map this check is constant time O(1)
	ensure!(!Members::<T>::contains_key(&new_member), Error::<T>::AlreadyMember);

	// Insert the new member and emit the event
	Members::<T>::insert(&new_member, ());
	MemberCount::put(member_count + 1); // overflow check not necessary because of maximum
	Self::deposit_event(RawEvent::MemberAdded(new_member));
	Ok(())
}
```

When we successfully add a new member, we also manually update the size of the set.

## Removing a Member

Removing a member is straightforward. We begin by looking for the caller in the list. If not
present, there is no work to be done. If the caller is present, we simply remove them and update the
size of the set.

```rust, ignore
fn remove_member(origin) -> DispatchResult {
	let old_member = ensure_signed(origin)?;

	ensure!(Members::<T>::contains_key(&old_member), Error::<T>::NotMember);

	Members::<T>::remove(&old_member);
	MemberCount::mutate(|v| *v -= 1);
	Self::deposit_event(RawEvent::MemberRemoved(old_member));
	Ok(())
}
```

## Performance

Now that we have built our set, let's analyze its performance in some common operations.

### Membership Check

In order to check for the presence of an item in a map set, we make a single storage read. If we
only care about the presence or absence of the item, we don't even need to decode it. This constant
time membership check is the greatest strength of a map set.

DB Reads: O(1)

### Updating

Updates to the set, such as adding and removing members as we demonstrated, requires first
performing a membership check. Additions also require encooding the new item.

DB Reads: O(1) Encoding: O(1) DB Writes: O(1)

If your set operations will require a lot of membership checks or mutation of individual items, you
may want a `map-set`.

### Iteration

Iterating over all items in a `map-set` is achieved by using the
[`IterableStorageMap` trait](https://substrate.dev/rustdocs/latest/frame_support/storage/trait.IterableStorageMap.html),
which iterates `(key, value)` pairs (although in this case, we don't care about the values). Because
each map entry is stored as an individual trie node, iterating a map set requires a database read
for each item. Finally, the actual processing of the items will take some time.

DB Reads: O(n) Decoding: O(n) Processing: O(n)

Because accessing the database is a relatively slow operation, returning to the database for each
item is quite expensive. If your set operations will require frequent iterating, you will probably
prefer a [`vec-set`](./vec-set.md).

### A Note on Weights

It is always important that the weight associated with your dispatchables represent the actual time
it takes to execute them. In this pallet, we have provided an upper bound on the size of the set,
which places an upper bound on the computation - this means we can use constant weight annotations.
Your set operations should either have a maximum size or a [custom weight function](./weights.md)
that captures the computation appropriately.
