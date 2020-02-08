# Cache Multiple Calls
*[`pallets/storage-cache`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/storage-cache)*

Calls to runtime storage have an associated cost. With this in mind, multiple calls to storage values should be avoided when possible.

```rust, ignore
decl_storage! {
    trait Store for Module<T: Trait> as StorageCache {
        // copy type
        SomeCopyValue get(fn some_copy_value): u32;

        // clone type
        KingMember get(fn king_member): T::AccountId;
        GroupMembers get(fn group_members): Vec<T::AccountId>;
    }
}
```


For [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) types, it is easy to reuse previous storage calls by simply reusing the value (which is automatically cloned upon reuse). With this in mind, the second call in the following code is unnecessary:

```rust, ignore
fn swap_value_no_cache(origin, some_val: u32) -> Result {
    let _ = ensure_signed(origin)?;
    let original_call = <SomeCopyValue>::get();
    let some_calculation = original_call + some_val;
    // this next storage call is unnecessary and is wasteful
    let unnecessary_call = <SomeCopyValue>::get();
    // should've just used first_call here because u32 is copy
    let another_calculation = some_calculation + unnecessary_call;
    <SomeCopyValue>::put(another_calculation);
    let now = <system::Module<T>>::block_number();
    Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
    Ok(())
}
```

Instead, the initial call value should be reused. In this example, the `SomeCopyValue` value is [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) so we should prefer the following code without the unnecessary second call to storage:

```rust, ignore
fn swap_value_w_copy(origin, some_val: u32) -> Result {
    let _ = ensure_signed(origin)?;
    let original_call = <SomeCopyValue>::get();
    let some_calculation = original_call + some_val;
    // uses the original_call because u32 is copy
    let another_calculation = some_calculation + original_call;
    <SomeCopyValue>::put(another_calculation);
    let now = <system::Module<T>>::block_number();
    Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
    Ok(())
}
```

If the type was not `Copy`, but was [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), then it is still preferred to clone the value in the method than to make another call to runtime storage.

```rust, ignore
decl_storage! {
    trait Store for Module<T: Trait> as StorageCache {
        // ...<copy type here>...
        // clone type
        KingMember get(fn king_member): T::AccountId;
        GroupMembers get(fn group_members): Vec<T::AccountId>;
    }
}
```

The runtime methods enable the calling account to swap the `T::AccountId` value in storage if
1. the existing storage value is not in `GroupMembers` AND
2. the calling account is in `Group Members

The first implementation makes a second unnecessary call to runtime storage instead of cloning the call for `existing_key`:
```rust, ignore
fn swap_king_no_cache(origin) -> Result {
    let new_king = ensure_signed(origin)?;
    let existing_king = <KingMember<T>>::get();

    // only places a new account if
    // (1) the existing account is not a member &&
    // (2) the new account is a member
    ensure!(!Self::is_member(existing_king), "is a member so maintains priority");
    ensure!(Self::is_member(new_king.clone()), "not a member so doesn't get priority");

    // BAD (unnecessary) storage call
    let old_king = <KingMember<T>>::get();
    // place new king
    <KingMember<T>>::put(new_king.clone());

    Self::deposit_event(RawEvent::InefficientKingSwap(old_king, new_king));
    Ok(())
}
```

If the `existing_key` is used without a `clone` in the event emission instead of `old_king`, then the compiler returns the following error

```bash
error[E0382]: use of moved value: `existing_king`
  --> src/lib.rs:93:63
   |
80 |             let existing_king = <KingMember<T>>::get();
   |                 ------------- move occurs because `existing_king` has type `<T as frame_system::Trait>::AccountId`, which does not implement the `Copy` trait
...
85 |             ensure!(!Self::is_member(existing_king), "is a member so maintains priority");
   |                                      ------------- value moved here
...
93 |             Self::deposit_event(RawEvent::InefficientKingSwap(existing_king, new_king));
   |                                                               ^^^^^^^^^^^^^ value used here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: Could not compile `storage-cache`.

To learn more, run the command again with --verbose.
```

Fixing this only requires cloning the original call to storage before it is moved:

```rust, ignore
fn swap_king_with_cache(origin) -> Result {
    let new_king = ensure_signed(origin)?;
    let existing_king = <KingMember<T>>::get();
    // clone before existing_king is moved
    let old_king = existing_king.clone();

    // existing king is moved next
    ensure!(!Self::is_member(existing_king), "is a member so maintains priority");
    ensure!(Self::is_member(new_king.clone()), "not a member so doesn't get priority");

    // <no (unnecessary) storage call here>
    // place new king
    <KingMember<T>>::put(new_king.clone());

    // use cached old_king value here
    Self::deposit_event(RawEvent::BetterKingSwap(old_king, new_king));
    Ok(())
}
```

Not all types implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) or [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), so it is important to discern other patterns that minimize and alleviate the cost of calls to storage.
