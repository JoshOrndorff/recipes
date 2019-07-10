# Higher Order Arrays with Tuples and Maps
*[naive social network recipe below](#naive) below*

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

Patterns that use mappings to emulate higher order data structures are common when managing runtime storage on Substrate. 

## Naive Social Network

We can use this pattern to manage [whitelists and blacklists](https://stackoverflow.com/questions/1453285/what-is-whitelist-and-blacklist-data). This is especially useful in the context of social networks for adding/removing friends and blocking unfriendly participants.

The relevant state transitions are encoded in the `decl_event` block

```rust
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
      NewFriend(AccountId),
      FriendRemoved(AccountId),
      Blocked(AccountId),
      UnBlocked(AccountId),
    }
);
```

Our storage items contain a higher order array represented by the items described previously. 

```rust
decl_storage! {
  trait Store for Module<T: Trait> as SocialNetwork {
    MyFriend get(my_friend): map (T::AccountId, u32) => T::AccountId;
    FriendsCount get(friends_count): map T::AccountId => u32;
    AllFriends get(all_friends): map T::AccountId => Vec<T::AccountId>;
    Blocked get(blocked): map T::AccountId => Vec<T::AccountId>;
  }
}
```

We also include two vectors for a user's friends and the participants that they have blocked. These vectors are convenient when paired with runtime methods for verifying membership.

```rust
impl<T: Trait> Module<T> {
  pub fn friend_exists(current: T::AccountId, friend: T::AccountId) -> bool {
    // search for friend in AllFriends vector
    <AllFriends<T>>::get(current).iter()
		  .any(|&ref a| a == &friend)
  }

  pub fn is_blocked(current: T::AccountId, other_user: T::AccountId) -> bool {
    // search for friend in Blocked vector
    <Blocked<T>>::get(current).iter()
		  .any(|&ref a| a == &other_user)
  }
}
```

By returning `bool`, we can easily use these methods in `ensure!` statements to verify relevant state conditions before making requests in the main runtime methods. For example, in the `remove_friend` runtime method, we need to ensure that the friend to be removed is an existing friend.

```rust
ensure!(Self::friend_exists(user.clone(), old_friend.clone()), "old friend is not a friend");
```

Similarly, when we block a user, we should check that the user isn't already blocked.

```rust
ensure!(!Self::is_blocked(user.clone(), blocked_user.clone()), "user is already blocked");
```

The full logic for this sample can be found in the [kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen) in `storage/arrays`.
<!-- **TODO: update link once pushed** -->

*To see another example of how to use tuplies to emulate higher order arrays, see the [Substrate Collectables Tutorial](https://shawntabrizi.github.io/substrate-collectables-workshop/#/2/owning-multiple-kitties?id=using-tuples-to-emulate-higher-order-arrays).*

**NOTE**: [DoubleMap](https://crates.parity.io/srml_support/storage/trait.StorageDoubleMap.html) is a map with two keys; this storage item may also be useful for implementing higher order arrays