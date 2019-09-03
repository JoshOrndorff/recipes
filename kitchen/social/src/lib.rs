#![cfg_attr(not(feature = "std"), no_std)]

/// TODO:
/// this is a really poor implementation
/// brainstorm and rewrite `=>` create a profile struct for managing friend information
/// use constant getters and softmax?
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, EnumerableStorageMap,
    StorageMap, StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as SocialNetwork {
        MyFriend get(my_friend): map (T::AccountId, u32) => T::AccountId;
        FriendsCount get(friends_count): map T::AccountId => u32;
        AllFriends get(all_friends): map T::AccountId => Vec<T::AccountId>;
        Blocked get(blocked): map T::AccountId => Vec<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        NewFriend(AccountId),
        FriendRemoved(AccountId),
        Blocked(AccountId),
        UnBlocked(AccountId),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        pub fn add_friend(origin, new_friend: T::AccountId) {
            let user = ensure_signed(origin)?;

            // ensure the friend doesn't already exist
            ensure!(!Self::friend_exists(user.clone(), new_friend.clone()), "new friend is already a friend");

            // if blocked, unblock
            if Self::is_blocked(user.clone(), new_friend.clone()) {
                // unblock
                <Blocked<T>>::get(user.clone()).retain(|x| *x != new_friend.clone());
                Self::deposit_event(RawEvent::UnBlocked(new_friend.clone()));
            }

            // iterate friends count
            <FriendsCount<T>>::mutate(user.clone(), |count| *count + 1);
            let current_count = <FriendsCount<T>>::get(user.clone());
            // add new friend
            <MyFriend<T>>::insert((user.clone(), current_count), new_friend.clone());
            <AllFriends<T>>::get(user).push(new_friend.clone());

            Self::deposit_event(RawEvent::NewFriend(new_friend));
        }

        pub fn remove_friend(origin, old_friend: T::AccountId) {
            let user = ensure_signed(origin)?;

            // ensure the removed friend is an existing friend
            ensure!(Self::friend_exists(user.clone(), old_friend.clone()), "old friend is not a friend");

            // swap and pop
            let current_count = <FriendsCount<T>>::get(user.clone());
            let old_friend_index = Self::get_friend_index(user.clone(), old_friend.clone()).unwrap();
            let friend_to_remove = <MyFriend<T>>::take((user.clone(), old_friend_index));
            // swap
            if old_friend_index != current_count {
                let head_friend = <MyFriend<T>>::take((user.clone(), current_count));
                <MyFriend<T>>::insert((user.clone(), old_friend_index), head_friend);
                <MyFriend<T>>::insert((user.clone(), current_count), friend_to_remove.clone());
            }
            // pop
            <MyFriend<T>>::remove((user.clone(), current_count));
            <FriendsCount<T>>::mutate(user.clone(), |count| *count - 1);
            <AllFriends<T>>::get(user).retain(|x| *x != old_friend.clone());

            Self::deposit_event(RawEvent::FriendRemoved(old_friend));
        }

        pub fn block_user(origin, blocked_user: T::AccountId) {
            let user = ensure_signed(origin)?;

            // ensure the user isn't already blocked
            ensure!(!Self::is_blocked(user.clone(), blocked_user.clone()), "user is already blocked");

            // add to blacklist
            <Blocked<T>>::get(user).push(blocked_user.clone());

            Self::deposit_event(RawEvent::Blocked(blocked_user));
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn friend_exists(current: T::AccountId, friend: T::AccountId) -> bool {
        // search for friend in AllFriends vector
        <AllFriends<T>>::get(current).contains(&friend)
    }

    pub fn is_blocked(current: T::AccountId, other_user: T::AccountId) -> bool {
        // search for friend in Blocked vector
        <Blocked<T>>::get(current).contains(&other_user)
    }

    pub fn get_friend_index(current: T::AccountId, friend: T::AccountId) -> Option<u32> {
        // should never be called if the friend isn't a friend of current
        // if so, it doesn't return, the runtime panics `=>` yikes!
        let current_count = <FriendsCount<T>>::get(current.clone());
        for i in 0..current_count {
            if friend == <MyFriend<T>>::get((current.clone(), i)) {
                return Some(i);
            }
        }
        None
    }
}
