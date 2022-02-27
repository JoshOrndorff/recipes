# Efficent Subgroup Removal by Subkey: Double Maps

`pallets/double-map`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fdouble-map%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/double-map/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

For some runtimes, it may be necessary to remove a subset of values in a key-value mapping. If the
subset maintain an associated identifier type, this can be done in a clean way with the
[`double_map`](https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageDoubleMap.html) via the
`remove_prefix` api.

```rust, ignore
pub type GroupIndex = u32; // this is Encode (which is necessary for double_map)

#[pallet::storage]
#[pallet::getter(fn member_score)]
pub(super) type MemberScore<T: Config> = StorageDoubleMap<
  _,
  Blake2_128Concat,
  GroupIndex,
  Blake2_128Concat,
  T::AccountId,
  u32,
  ValueQuery,
>;

#[pallet::storage]
#[pallet::getter(fn group_membership)]
pub(super) type GroupMembership<T: Config> =
  StorageMap<_, Blake2_128Concat, T::AccountId, GroupIndex, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn all_members)]
pub(super) type AllMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;
```

For the purposes of this example, store the scores of each member in a map that associates this
`u32` value with two keys: (1) a `GroupIndex` identifier, and (2) the member's `AccountId`. This
allows for efficient removal of all values associated with a specific `GroupIndex` identifier.

```rust, ignore
/// Remove group score
#[pallet::weight(10_000)]
pub fn remove_group_score(
  origin: OriginFor<T>,
  group: GroupIndex,
) -> DispatchResultWithPostInfo {
  let member = ensure_signed(origin)?;

  let group_id = <GroupMembership<T>>::get(member);
  // check that the member is in the group
  ensure!(
    group_id == group,
    "member isn't in the group, can't remove it"
  );

  // remove all group members from MemberScore at once
  <MemberScore<T>>::remove_prefix(&group_id);

  Self::deposit_event(Event::RemoveGroup(group_id));
  Ok(().into())
}
```
