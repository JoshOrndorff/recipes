/// Higher Order Arrays
/// To represent ownership of multiple items across multiple users, tuples can 
/// be used alongside maps in order to emulate arrays.

use support::{ensure, decl_module, decl_storage, decl_event, StorageMap, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
  trait Store for Module<T: Trait> as TokenTransfer {
    SocialNetwork get(my_friend): map (T::AccountId, u32) => T::AccountId;
    SocialNetwork get(friends_count): map T::AccountId => u32;
  }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
      // interesting
    }
);

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    // initialize the default event for this module
    fn deposit_event<T>() = default;
  }
}