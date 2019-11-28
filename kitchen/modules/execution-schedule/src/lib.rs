#![cfg_attr(not(feature = "std"), no_std)]
//! Execution Schedule
use rstd::prelude::*;
use runtime_primitives::{
    RuntimeDebug, traits::{SimpleArithmetic, BlockNumber, Hash, Zero},
};
use support::{
    traits::Get, decl_event, decl_module, decl_storage, ensure,
    dispatch::Result, StorageDoubleMap, StorageMap, StorageValue,
    codec::{Encode, Decode},
};
use system::ensure_signed;

/// Type aliases for this modular's configuration
pub type TaskId = Vec<u8>;
/// The priority of a task
pub type PriorityScore = u32;
/// RoundIndex to manage voting with `double_map`
pub type RoundIndex = u32;

/// Generic Task Abstraction
///
/// - `id` field is a `TaskId = Vec<u8>` which could be an identifier or a hash corresponding to some data in IPFS
/// - `priority_score` indicates signalling support from members for tasks
/// - `proposed_at` indicates the time at which the task was initially proposed 
#[derive(Encode, Decode, RuntimeDebug)]
pub struct Task<BlockNumber> {
    /// A vec of bytes which could be an identifier or a hash corresponding to associated data in IPFS or something
    id: TaskId,
    /// The priority of the task relative to other tasks
    score: PriorityScore,
    /// The block number at which the task is initially queued
    proposed_at: BlockNumber,
}

impl<BlockNumber: SimpleArithmetic + Copy> Ord for Task<BlockNumber> {
    fn cmp(&self, other: Self) -> Ordering {
        self.score.cmp(&other.priority_score)
    }
}

impl<BlockNumber: SimpleArithmetic + Copy> PartialOrd for Task<BlockNumber> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self.cmp(other))
    }
}

impl<BlockNumber: SimpleArithmetic + Copy> PartialEq for Task<BlockNumber> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait Trait: system::Trait {
    /// Overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Quota for members to signal task priority every ExecutionFrequency
    type SignalQuota: Get<PriorityScore>;

    /// The frequency of batch executions for tasks (in `on_finalize`)
    type ExecutionFrequency: Get<Self::BlockNumber>;

    /// The maximum number of tasks that can be approved in an `ExecutionFrequency` period
    type TaskLimit: Get<PriorityScore>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {   
        /// Signal is refreshed for all members at this block number
        SignalRefreshed(BlockNumber),
        /// Task is scheduled by the proposer with `TaskId` and expected_execution_time
        TaskScheduled(AccountId, TaskId, BlockNumber),
        /// Signal support for a task
        SignalSupport(TaskId, PriorityScore),
        /// Task is executed with this identifier at this block number
        TaskExecuted(TaskId, BlockNumber),
        /// New expected execution time for tasks not completed within first *opportunity*
        UpdatedTaskSchedule(TaskId, BlockNumber),
    }   
);

decl_storage! {
    trait Store for Module<T: Trait> as ExecutionSchedule {
        /// Outstanding tasks getter
        PendingTask get(fn pending_task): map TaskId => Option<Task<T::BlockNumber>>;
        /// Dispatch queue for task execution
        ExecutionQueue get(fn execution_queue): Vec<TaskId>;
        /// The signalling quota left in terms of `PriorityScore` for all members of the council (until it is killed `on_initialize` on `ExecutionFrequency` blocks)
        SignalBank get(fn signal_bank): double_map RoundIndex, twox_128(T::AccountId) => PriorityScore;
        /// The (closed and static) council of members (anyone can submit tasks but only members can signal priority)
        Council get(fn council): Vec<T::AccountId>;
        /// The nonce that increments every `ExecutionFrequency` for a new `SignalBank` instantiation
        Era get(fn era): RoundIndex;
    }
    add_extra_genesis {
        build(|config| {
            // start at era 0
            <Era<T>>::put(0u32.into());
            <Council<T>>::put(&config.council);
            let signal_quota = T::SignalQuota::get();
            config.council.into_iter().for_each(|member| {
                // fill signal quota for all initial members for era 0
                <SignalBank<T>>::insert(0u32.into(), &member, signal_quota);
            });
        });
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        const SignalQuota: PriorityScore = T::SignalQuota::get();
        const ExecutionFrequency: T::BlockNumber = T::ExecutionFrequency::get();
        const TaskLimit: PriorityScore = T::TaskLimit::get();

        /// On Initialize
        ///
        /// After the last block's on_finalize, the logic expressed in this method
        /// is executed before the logic in the next block.  
        /// - This allows us to start from 0 for all tasks 
        fn on_initialize(n: T::BlockNumber) {
            let one_block_after = T::ExecutionFrequency::get() + 1.into();
            if (n % (one_block_after).is_zero()) {
                let last_era = <Era<T>>::get();
                // clean up the previous double_map with this last_era group index
                <SignalBank<T>>::remove_prefix(&last_era);
                // unlikely to overflow
                let next_era = last_era + 1u32.into();
                <Era<T>>::put(next_era);

                // get the SignalQuota for each `ExecutionFrequency` period
                let signal_quota = T::SignalQuota::get();
                // instantiate next mapping for SignalQuota with new Era
                <Council<T>>::get().into_iter().for_each(|member| {
                    // refresh signal quota for all members for the next era
                    <SignalBank<T>>::put(next_era, &member, signal_quota);
                });
                Self::deposit_event(RawEvent::SignalRefreshed(n));
            }
        }

        /// Schedule Task for Batch Execution
        ///
        /// - the task initially has no priority
        /// - only council members can schedule tasks
        fn schedule_task(origin, data: Vec<u8>) -> Result {
            let proposer = ensure_signed(origin)?;
            ensure!(Self::is_on_council(&proposer), "only members of the council can schedule tasks");

            // current block number
            let proposed_at = <system::Module<T>>::block_number();
            // use current time to estimate the expected `BlockNumber` for execution
            let expected_execution = Self::execution_estimate(proposed_at);

            let task_to_schedule = Task {
                id: data.clone().into(),
                score: 0u32.into(), 
                proposed_at,
            };
            // add tasks as values to map with `TaskId` as the key
            // note: by default overwrites any value stored at the `data.clone()` key
            <PendingTasks<T>>::insert(data.clone(), task_to_schedule);
            // add to TaskQ for scheduled execution
            <ExecutionQueue<T>>::append(&[task_to_schedule])?;

            Self::deposit_event(RawEvent::TaskScheduled(proposer, data.into(), expected_execution));
            Ok(())
        }

        /// Increase Priority for the Task
        ///
        /// - members of the council have limited voting power to increase the priority
        /// of tasks
        fn signal_priority(origin, id: TaskId, signal: PriorityScore) -> Result {
            let voter = ensure_signed(origin)?;
            ensure!(Self::is_on_council(&voter), "The voting member must be on the council");

            // get the current voting era
            let current_era = <Era<T>>::get();
            // get the voter's remaining signal in this voting era
            let mut remaining_signal = <SignalBank<T>>::get(current_era, &voter);
            ensure!(remaining_signal >= signal, "The voter cannot signal more than the remaining signal");
            if let Some(mut task) = <PendingTasks<T>>::get(id) {
                task.priority_score.checked_add(signal).ok_or("task is too popular and signal support overflowed")?;
                // don't have to checked_sub because just verified that remaining_signal >= signal
                remaining_signal -= signal;
            } else {
                return Err("the task did not exist in the PendingTasks storage map");
            }
            Self::deposit_event(RawEvent::SignalSupport(id, signal));
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::ExecutionFrequency::get()).is_zero() {
                Self::execute_tasks(n);
            }
        }
    }
}

impl<T: Trait> Module<T> {
    /// Checks whether the input member is in the council governance body
    fn is_on_council(who: &T::AccountId) -> bool {
        Self::council().contains(who)
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

    /// Execute Tasks
    ///
    /// - exhaustively executes tasks in the order of their priority
    pub fn execute_tasks(n: T::BlockNumber) {
        // task limit in terms of priority allowed to be executed every period
        let mut task_allowance = T::TaskLimit::get();
        <ExecutionQueue<T>>::get().sort_unstable().into_iter().for_each(|task| {
            if task.priority_score <= task_allowance {
                // execute task (could have more express computation here)
                // or in off-chain worker running after this block
                <PendingTasks<T>>::remove(&task.id);
                task_allowance -= task.priority_score;
                Self::deposit_event(RawEvent::TaskExecuted(task.id, n));
            } else {
                // need to explicitly deal with case of priority_score > task_allowance to ensure loop doesn't continue (aka exhaustive execution)
                break
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::*; //{Module, Trait, RawEvent, Task, GenesisConfig};
    use primitives::H256;
    use runtime_io;
    use runtime_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup, OnFinalize, OnInitialize},
        Perbill,
    };
    use rand::{thread_rng, RngCore, rngs::OsRng};
    use support::{assert_err, impl_outer_event, impl_outer_origin, parameter_types, traits::Get};
    use system::ensure_signed;

    impl<BlockNumber> Task<BlockNumber> {
        // Generate Random Tasks
        // for testing purposes (see tests further below)
        // (should move to tests and isolate rand import in tests module)
        fn random() -> Self {
            Self {
                id: generate_random_id(),
                priority_score: RngCore::next_u32().into(),
                proposed_at: RngCore::next_u64().into(),
            }
        }
    }
    
    // Generate Random TaskId for testing purposes
    pub fn generate_random_task_id(output_len: u32) -> TaskId {
        let mut buf = vec![0u8; output_len];
        OsRng.fill_bytes(&mut buf);
        buf
    }

    impl_outer_origin! {
        pub enum Origin for TestRuntime {}
    }

    thread_local! {
        static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
        static TRANSFER_FEE: RefCell<u64> = RefCell::new(0);
        static CREATION_FEE: RefCell<u64> = RefCell::new(0);
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

    pub struct ExtBuilder;

    impl ExtBuilder {
        // more setters for ExtBuilder environment

        // the values for one default configuration
        pub fn set_associated_consts(&self) {
            EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
            TRANSFER_FEE.with(|v| *v.borrow_mut() = self.transfer_fee);
            CREATION_FEE.with(|v| *v.borrow_mut() = self.creation_fee);
        }

        pub fn build() -> runtime_io::TestExternalities {
            self.set_associated_consts();
            let mut storage = system::GenesisConfig::default()
                .build_storage::<TestRuntime>()
                .unwrap();
            runtime_io::TestExternalities::from(storage)
        }
    }

    #[test]
    fn task_schedule_estimator_works() {
        //todo
        assert!(true);
    }
}