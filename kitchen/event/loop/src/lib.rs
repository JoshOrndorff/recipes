//! scheduling execution (the blockchain event loop)
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]
// runtime imports
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "std")]
use runtime_primitives::traits::{Hash, Zero};
use support::traits::Get;
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue
};
use system::ensure_signed;

// type alias for counting actions
pub type ActionIndex = u32;

// example of a minimal action struct
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Action<BlockNumber> {
    index: ActionIndex,
    time_of_proposal: BlockNumber,
}

pub trait Trait: system::Trait {
    // overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    // how frequently proposals are passed from the dispatchQ
    type ActionFrequency: Get<Self::BlockNumber>;
}

decl_event!(
    pub enum Event<T>
    where 
        AccountId = <T as system::Trait>::AccountId,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        ActionScheduled(AccountId, BlockNumber),
        ActionExecuted(BlockNumber),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as eloop {
        /// Total actions in existence
        ActionCount get(action_count): ActionIndex;
        /// Outstanding proposals getter
        Actions get(proposals): map T::Hash => Option<Action<T::BlockNumber>>;
        /// Dispatch Queue for actions
        ActionQ get(dispatch_q): Vec<T::Hash>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        // frequency with which the ActionQ is executed
        const ActionFrequency: T::BlockNumber = T::ActionFrequency::get();

        fn add_action(origin) -> Result {
            let proposer = ensure_signed(origin)?;

            // increment actioncount
            let index: ActionIndex = <ActionCount>::get() + 1;
            <ActionCount>::put(index);
            // get current time
            let time_of_proposal = <system::Module<T>>::block_number();
            let new_action = Action { index, time_of_proposal, };
            // insert action into Q
            let hash = <T as system::Trait>::Hashing::hash_of(&new_action);
            // add to actions map
            <Actions<T>>::insert(hash, new_action);
            // add to Q for execution
            <ActionQ<T>>::mutate(|acts| acts.push(hash));
            // deposit event
            Self::deposit_event(RawEvent::ActionScheduled(proposer, time_of_proposal));

            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::ActionFrequency::get()).is_zero() {
                // execute from the dispatchQ
                Self::execute_actions(n);
            }
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn execute_actions(n: T::BlockNumber) {
        <ActionQ<T>>::get().into_iter().for_each(|h| {
            // this is where we might do something related to the action
            <Actions<T>>::remove(h); // here, we just remove it from the map
            // decrement action count
            let new_count = <ActionCount>::get() - 1;
            <ActionCount>::put(new_count);
        });
        <ActionQ<T>>::kill();
        Self::deposit_event(RawEvent::ActionExecuted(n));
    }
}