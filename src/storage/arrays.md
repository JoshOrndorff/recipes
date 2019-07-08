# Higher Order Arrays with Tuples and Maps

To represent ownership of multiple items across multiple users, tuples can be used alongside maps in order to emulate arrays.

For example, consider a scenario in which persistent storage keeps track of a *social network graph* in which each user (represented by an `AccountId`) has a list of other friends. In this case, it would be convenient to use a 2 dimensional array like

```rust
SocialNetwork[AccountId][Index] -> AccountId
```

With this data structure, check how many friends a given `AccountId` has by calling

```rust
SocialNetwork[AccountId].length()
```

To emulate this data structure in the context of the Substrate runtime storage, use tuples and maps (declared in a `decl_storage!{}` block like previous examples):

```rust
MyFriend get(my_friend): map (T::AccountId, u32) => T::AccountId;
FriendsCount get(friends_count): map T::AccountId => u32;
```

Patterns that use mappings to emulate higher order data structures are common when managing runtime storage on Substrate. *To see this pattern in action, see the [Substrate Collectables Tutorial](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/owning-multiple-kitties?id=using-tuples-to-emulate-higher-order-arrays).*