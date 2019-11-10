#![cfg_attr(not(feature = "std"), no_std)]

// schedule with `OnFinalize`
use parity_scale_codec::{Decode, Encode};
use runtime_primitives::traits::{Hash, Zero};
use support::traits::Get;
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, StorageMap, StorageValue,
};
use system::ensure_signed;
use rstd::prelude::*;

// type alias ordering task execution in `on_finalize`
pub type PriorityScore = u32;

// generic task struct
#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Task<BlockNumber> {
    // the priority of the task relative to other tasks
    priority_score: PriorityScore,
    // time at which the task is initially queued
    proposed_at: BlockNumber,
}

pub trait Trait: system::Trait {
    // overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    // how frequently proposals are passed from the dispatchQ
    type ExecutionFrequency: Get<Self::BlockNumber>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        // proposer's AccountId, BlockNumber at expected execution
        TaskScheduled(AccountId, BlockNumber),
        TaskExecuted(BlockNumber),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as EventLoop {
        /// Outstanding tasks getter
        Tasks get(fn tasks): map T::Hash => Option<Task<T::BlockNumber>>;
        /// Dispatch Queue for tasks
        TaskQ get(fn task_q): Vec<T::Hash>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // frequency with which the ActionQ is executed
        const ExecutionFrequency: T::BlockNumber = T::ExecutionFrequency::get();

        fn schedule_task(origin, priority_score: PriorityScore) -> Result {
            let proposer = ensure_signed(origin)?;

            // get current time
            let proposed_at = <system::Module<T>>::block_number();

            // to emit an event regarding the expected execution time
            let cached_frequency = T::ExecutionFrequency::get();
            let mut expected_execution_time = proposed_at;
            loop {
                // the expected execution time is the next block number divisible by `ExecutionFrequency`
                if (expected_execution_time % cached_frequency).is_zero() {
                    break;
                } else {
                    expected_execution_time += 1.into();
                }
            }

            //                          PriorityScore, proposed_at BlockNumber
            let task_to_schedule = Task { priority_score, proposed_at };
            // insert action into Q
            let hash = <T as system::Trait>::Hashing::hash_of(&task_to_schedule);
            // add to tasks map
            <Tasks<T>>::insert(hash, task_to_schedule);
            // add to TaskQ for scheduled execution
            <TaskQ<T>>::mutate(|t| t.push(hash));

            Self::deposit_event(RawEvent::TaskScheduled(proposer, expected_execution_time));
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::ExecutionFrequency::get()).is_zero() {
                // execute from the dispatchQ
                Self::execute_tasks(n);
            }
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn execute_tasks(n: T::BlockNumber) {
        let mut execute_q = Vec::new();
        <TaskQ<T>>::get().into_iter().for_each(|h| {
            execute_q.push(<Tasks<T>>::get(h));
            // sort based on priority score and block number
            execute_q.sort();
            execute_q.iter().for_each(|_t| {
                // this is where each task is executed
                // -- execution occurs in order based on sort()
            });
            <Tasks<T>>::remove(h); // here, we just remove executed tasks from the map
        });
        <TaskQ<T>>::kill();
        Self::deposit_event(RawEvent::TaskExecuted(n));
    }
}
