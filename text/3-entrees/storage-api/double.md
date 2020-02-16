# Efficent Subgroup Removal by Subkey: Double Maps
*[`pallets/double-map`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/double-map)*

For some runtimes, it may be necessary to remove a subset of values in a key-value mapping. If the subset maintain an associated identifier type, this can be done in a clean way with the [`double_map`](https://substrate.dev/rustdocs/master/frame_support/storage/trait.StorageDoubleMap.html) via the `remove_prefix` api.

```rust, ignore
pub type GroupIndex = u32; // this is Encode (which is necessary for double_map)

decl_storage! {
	trait Store for Module<T: Trait> as Dmap {
        // member score (double map)
        MemberScore: double_map GroupIndex, twox_128(T::AccountId) => u32;
        // get group ID for member
        GroupMembership get(fn group_membership): map T::AccountId => GroupIndex;
        // for fast membership checks, see check-membership recipe for more details
        AllMembers get(fn all_members): Vec<T::AccountId>;
	}
}
```

For the purposes of this example,  store the scores of each members in a map that associates this `u32` value with two keys: (1) the hash of the member's `AccountId` and (2) a `GroupIndex` identifier. This allows for efficient removal of all values associated with a specific `GroupIndex` identifier.

```rust, ignore
fn remove_group_score(origin, group: GroupIndex) -> Result {
    let member = ensure_signed(origin)?;

    let group_id = <GroupMembership<T>>::get(member);
    // check that the member is in the group (could be improved by requiring n-of-m member support)
    ensure!(group_id == group, "member isn't in the group, can't remove it");

    // allows us to remove all group members from MemberScore at once
    <MemberScore<T>>::remove_prefix(&group_id);

    Self::deposit_event(RawEvent::RemoveGroup(group_id));
    Ok(())
}
```

**Note**: It is necessary for one of the two keys to be hashed; *[TODO](https://github.com/substrate-developer-hub/recipes/issues/46)*
