# Using Vectors as Sets

`pallets/vec-set`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fvec-set%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/vec-set/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

A [Set](<https://en.wikipedia.org/wiki/Set_(abstract_data_type)>) is an unordered data structure
that stores entries without duplicates. Substrate's storage API does not provide a way to declare
sets explicitly, but they can be implemented using either vectors or maps.

This recipe demonstrates how to implement a storage set on top of a vector, and explores the
performance of the implementation. When implementing a set in your own runtime, you should compare
this technique to implementing a [`map-set`](./map-set.md).

In this pallet we implement a set of `AccountId`s. We do not use the set for anything in this
pallet; we simply maintain the set. Using the set is demonstrated in the recipe on
[pallet coupling](./pallet-coupling.md). We provide dispatchable calls to add and remove members,
ensuring that the number of members never exceeds a hard-coded maximum.

```rust, ignore
/// A maximum number of members. When membership reaches this number, no new members may join.
pub const MAX_MEMBERS: usize = 16;
```

## Storage Item

We will store the members of our set in a Rust
[`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html). A `Vec` is a collection of elements that
is ordered and may contain duplicates. Because the `Vec` provides more functionality than our set
needs, we are able to build a set from the `Vec`. We declare our single storage item as so

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config> as VecSet {
		// The set of all members. Stored as a single vec
		Members get(fn members): Vec<T::AccountId>;
	}
}
```

In order to use the `Vec` successfully as a set, we will need to manually ensure that no duplicate
entries are added. To ensure reasonable performance, we will enforce that the `Vec` always remains
sorted. This allows for quickly determining whether an item is present using a
[binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm).

## Adding Members

Any user may join the membership set by calling the `add_member` dispatchable, providing they are
not already a member and the membership limit has not been reached. We check for these two
conditions first, and then insert the new member only after we are sure it is safe to do so. This is
an example of the mnemonic idiom, "**verify first write last**".

```rust, ignore
pub fn add_member(origin) -> DispatchResult {
	let new_member = ensure_signed(origin)?;

	let mut members = Members::<T>::get();
	ensure!(members.len() < MAX_MEMBERS, Error::<T>::MembershipLimitReached);

	// We don't want to add duplicate members, so we check whether the potential new
	// member is already present in the list. Because the list is always ordered, we can
	// leverage the binary search which makes this check O(log n).
	match members.binary_search(&new_member) {
		// If the search succeeds, the caller is already a member, so just return
		Ok(_) => Err(Error::<T>::AlreadyMember.into()),
		// If the search fails, the caller is not a member and we learned the index where
		// they should be inserted
		Err(index) => {
			members.insert(index, new_member.clone());
			Members::<T>::put(members);
			Self::deposit_event(RawEvent::MemberAdded(new_member));
			Ok(())
		}
	}
}
```

If it turns out that the caller is not already a member, the binary search will fail. In this case
it still returns the index into the `Vec` at which the member would have been stored had they been
present. We then use this information to insert the member at the appropriate location, thus
maintaining a sorted `Vec`.

## Removing a Member

Removing a member is straightforward. We begin by looking for the caller in the list. If not
present, there is no work to be done. If the caller is present, the search algorithm returns her
index, and she can be removed.

```rust, ignore
fn remove_member(origin) -> DispatchResult {
	let old_member = ensure_signed(origin)?;

	let mut members = Members::<T>::get();

	// We have to find out where, in the sorted vec the member is, if anywhere.
	match members.binary_search(&old_member) {
		// If the search succeeds, the caller is a member, so remove her
		Ok(index) => {
			members.remove(index);
			Members::<T>::put(members);
			Self::deposit_event(RawEvent::MemberRemoved(old_member));
			Ok(())
		},
		// If the search fails, the caller is not a member, so just return
		Err(_) => Err(Error::<T>::NotMember.into()),
	}
}
```

## Performance

Now that we have built our set, let's analyze its performance in some common operations.

### Membership Check

In order to check for the presence of an item in a `vec-set`, we make a single storage read, decode
the entire vector, and perform a binary search.

DB Reads: O(1) Decoding: O(n) Search: O(log n)

### Updating

Updates to the set, such as adding and removing members as we demonstrated, requires first
performing a membership check. It also requires re-encoding the entire `Vec` and storing it back in
the database. Finally, it still costs the normal
[amortized constant time](https://stackoverflow.com/q/200384/4184410) associated with mutating a
`Vec`.

DB Writes: O(1) Encoding: O(n)

### Iteration

Iterating over all items in a `vec-set` is achieved by using the `Vec`'s own
[`iter` method](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter). The entire set can
be read from storage in one go, and each item must be decoded. Finally, the actual processing you do
on the items will take some time.

DB Reads: O(1) Decoding: O(n) Processing: O(n)

Because accessing the database is a relatively slow operation, reading the entire list in a single
read is a big win. If you need to iterate over the data frequently, you may want a `vec-set`.

### A Note on Weights

It is always important that the weight associated with your dispatchables represent the actual time
it takes to execute them. In this pallet, we have provided an upper bound on the size of the set,
which places an upper bound on the computation - this means we can use constant weight annotations.
Your set operations should either have a maximum size or a [custom weight function](./weights.md)
that captures the computation appropriately.
