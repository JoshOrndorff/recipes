//! Execution Schedule
//! - demonstrate `on_finalize`
//! - demonstrate `on_initialize`
//! - should reference off-chain workers recipe (wip)
//! Related Recipes
//! - `smpl-treasury` schedules execution of transfers by batching them every `UserSpend` number of blocks

#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::{Decode, Encode};
use rstd::prelude::*;
use runtime_primitives::traits::{Hash, Zero};
use support::traits::Get;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageMap, StorageValue};
use system::ensure_signed;

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

    // how frequently are tasks batch executed
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

        /// Schedule Task for Batch Execution
        ///
        /// this method instantiates a `Task` and estimates when it will be executed next
        /// before adding it to the TaskQ
        fn schedule_task(origin, priority_score: PriorityScore) -> Result {
            let proposer = ensure_signed(origin)?;

            // get current time
            let proposed_at = <system::Module<T>>::block_number();

            let expected_execution_time = Self::execution_estimate(proposed_at);

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

    /// Execution Estimate
    ///
    /// emits an event parameter in `schedule_task` to tell users when
    /// (which block number), the task will be executed based on when it was submitted
    fn execution_estimate(now: T::BlockNumber) -> T::BlockNumber {
        // the frequency with which tasks are batch executed
        let batch_frequency = T::ExecutionFrequency::get();
        let mut expected_execution_time = now;
        loop {
            // the expected execution time is the next block number divisible by `ExecutionFrequency`
            if (expected_execution_time % batch_frequency).is_zero() {
                break;
            } else {
                expected_execution_time += 1.into();
            }
        }
        expected_execution_time
    }
}

// TODO: 
// - finish the testing docs and get them merged...
// - add scaffolding
// - add tests for `on_finalize` and existing code above && push
// - add `on_initialize` and tests
// - write docs for how the testing goes
// - push to merge all of this
#[cfg(test)]
mod tests {
    use crate::*; //{Module, Trait, RawEvent, SpendRequest, GenesisConfig};
    use primitives::H256;
    use runtime_io;
    use runtime_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup, OnFinalize},
        Perbill,
    };
    use support::{assert_err, impl_outer_event, impl_outer_origin, parameter_types, traits::Get};
    use system::ensure_signed;

    impl_outer_origin! {
        pub enum Origin for TestRuntime {}
    }

    // Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct TestRuntime;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: u32 = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::one();
    }
    impl system::Trait for TestRuntime {
        type Origin = Origin;
        type Index = u64;
        type Call = ();
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = TestEvent;
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
    }

    mod execution_schedule {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            execution_schedule<T>,
        }
    }

    parameter_types!{
        pub const ExecutionFrequency: u64 = 10;
    }
    impl Trait for TestRuntime {
        type Event = TestEvent;
        type ExecutionFrequency = ExecutionFrequency;
    }

    pub type System = system::Module<TestRuntime>;
    pub type ExecutionSchedule = Module<TestRuntime>;

    #[test]
    fn task_schedule_estimator_works() {
        //todo
        assert!(true);
    }
}