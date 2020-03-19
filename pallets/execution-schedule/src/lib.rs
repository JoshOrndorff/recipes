#![cfg_attr(not(feature = "std"), no_std)]
//! Scheduling Execution
use sp_std::prelude::*;
use sp_runtime::{traits::Zero, RuntimeDebug};
use frame_support::{
	codec::{Decode, Encode},
	decl_event, decl_module, decl_storage,
	dispatch::{DispatchResult, DispatchError},
	ensure,
	traits::Get,
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod tests;

pub type TaskId = Vec<u8>;
pub type PriorityScore = u32;
pub type RoundIndex = u32;

#[derive(Encode, Decode, RuntimeDebug)]
pub struct Task<BlockNumber> {
	/// A vec of bytes which could be an identifier or a hash corresponding to associated data in IPFS or something
	id: TaskId,
	/// The priority of the task relative to other tasks
	score: PriorityScore,
	/// The block number at which the task is initially queued
	proposed_at: BlockNumber,
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
		PendingTasks get(fn pending_tasks):
			map hasher(blake2_128_concat) TaskId => Option<Task<T::BlockNumber>>;

		/// Dispatch queue for task execution
		ExecutionQueue get(fn execution_queue): Vec<TaskId>;

		/// The signalling quota left in terms of `PriorityScore` for all members of the council (until it is killed `on_initialize` on `ExecutionFrequency` blocks)
		SignalBank get(fn signal_bank):
			double_map hasher(blake2_128_concat) RoundIndex, hasher(blake2_128_concat) T::AccountId => PriorityScore;

		/// The (closed and static) council of members (anyone can submit tasks but only members can signal priority)
		Council get(fn council): Vec<T::AccountId>;

		/// The nonce that increments every `ExecutionFrequency` for a new `SignalBank` instantiation
		Era get(fn era): RoundIndex;
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
			let batch_frequency = T::ExecutionFrequency::get();
			if ((n - 1.into()) % batch_frequency).is_zero() {
				let last_era = Era::get();
				// clean up the previous double_map with this last_era group index
				<SignalBank<T>>::remove_prefix(&last_era);
				// unlikely to overflow so no checked_add
				let next_era: RoundIndex = last_era + (1u32 as RoundIndex);
				Era::put(next_era);

				// get the SignalQuota for each `ExecutionFrequency` period
				let signal_quota = T::SignalQuota::get();
				// instantiate next mapping for SignalQuota with new Era
				<Council<T>>::get().into_iter().for_each(|member| {
					// refresh signal quota for all members for the next era
					<SignalBank<T>>::insert(next_era, &member, signal_quota);
				});
				Self::deposit_event(RawEvent::SignalRefreshed(n));
			}
		}

		/// Schedule Task for Batch Execution
		///
		/// - the task initially has no priority
		/// - only council members can schedule tasks
		fn schedule_task(origin, data: Vec<u8>) -> DispatchResult {
			let proposer = ensure_signed(origin)?;
			ensure!(Self::is_on_council(&proposer), "only members of the council can schedule tasks");

			// current block number
			let proposed_at = <system::Module<T>>::block_number();
			// use current time to estimate the expected `BlockNumber` for execution
			let expected_execution = Self::execution_estimate(proposed_at);

			let task_to_schedule = Task {
				id: data.clone(),
				score: 0u32,
				proposed_at,
			};
			// add tasks as values to map with `TaskId` as the key
			// note: by default overwrites any value stored at the `data.clone()` key
			<PendingTasks<T>>::insert(data.clone(), task_to_schedule);
			// add to TaskQ for scheduled execution
			<ExecutionQueue>::mutate(|q| q.push(data.clone()));

			Self::deposit_event(RawEvent::TaskScheduled(proposer, data, expected_execution));
			Ok(())
		}

		/// Increase Priority for the Task
		///
		/// - members of the council have limited voting power to increase the priority
		/// of tasks
		fn signal_priority(origin, id: TaskId, signal: PriorityScore) -> DispatchResult {
			let voter = ensure_signed(origin)?;
			ensure!(Self::is_on_council(&voter), "The voting member must be on the council");

			// get the current voting era
			let current_era = Era::get();
			// get the voter's remaining signal in this voting era
			let voters_signal = <SignalBank<T>>::get(current_era, &voter);
			ensure!(voters_signal >= signal, "The voter cannot signal more than the remaining signal");
			if let Some(mut task) = <PendingTasks<T>>::get(id.clone()) {
				task.score = task.score.checked_add(signal).ok_or("task is too popular and signal support overflowed")?;
				<PendingTasks<T>>::insert(id.clone(), task);
				// don't have to checked_sub because just verified that voters_signal >= signal
				let remaining_signal = voters_signal - signal;
				<SignalBank<T>>::insert(current_era, &voter, remaining_signal);
			} else {
				return Err(DispatchError::Other("the task did not exist in the PendingTasks storage map"));
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

	/// Efficient Execution Estimate
	fn execution_estimate(n: T::BlockNumber) -> T::BlockNumber {
		let batch_frequency = T::ExecutionFrequency::get();
		let miss = n % batch_frequency;
		n + (batch_frequency - miss)
	}

	/// Execute Tasks
	///
	/// - exhaustively executes tasks in the order of their priority
	pub fn execute_tasks(n: T::BlockNumber) {
		// task limit in terms of priority allowed to be executed every period
		let mut task_allowance = T::TaskLimit::get();
		let mut execution_q = <ExecutionQueue>::get().clone();
		execution_q.sort_unstable();
		execution_q.into_iter().for_each(|task_id| {
			if let Some(task) = <PendingTasks<T>>::get(&task_id) {
				if task.score <= task_allowance {
					// execute task (could have more express computation here)
					// or in off-chain worker running after this block
					task_allowance -= task.score;
					Self::deposit_event(RawEvent::TaskExecuted(task.id, n));
				} else {
					// need to explicitly end the loop when a single priority_score > task_allowance (prevent exhaustive execution)
					return;
				}
			}
			<PendingTasks<T>>::remove(&task_id);
		});
	}
}
