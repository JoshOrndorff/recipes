use crate::*; //{Module, Trait, RawEvent, Task, GenesisConfig};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, AtLeast32Bit},
	Perbill,
};
// it's ok, just for the testing suit, thread local variables
use rand::{rngs::OsRng, thread_rng, Rng, RngCore};
use std::cell::RefCell;
use frame_support::{
	assert_ok,
	impl_outer_event,
	impl_outer_origin,
	parameter_types,
	traits::{Get, OnInitialize, OnFinalize},
};
use frame_system as system;

// to compare expected storage items with storage items after method calls
impl<BlockNumber: AtLeast32Bit + Copy> PartialEq for Task<BlockNumber> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl<BlockNumber: Copy + AtLeast32Bit> Eq for Task<BlockNumber> {}

// Helper Methods For Testing Purposes
impl<T: Trait> Module<T> {
	fn add_member_to_council(who: T::AccountId) {
		<Council<T>>::mutate(|members| members.push(who));
	}

	fn add_member(who: T::AccountId) {
		Self::add_member_to_council(who.clone());
		let current_era = Era::get();
		// intialize with 0, filled full at beginning of next_era
		<SignalBank<T>>::insert(current_era, who, 0u32);
	}

	// Naive Execution Estimate
	//
	// emits an event parameter in `schedule_task` to tell users when
	// (which block number), the task is expected to be executed based on when it was submitted
	// - iteration makes it quite naive
	fn naive_execution_estimate(now: T::BlockNumber) -> T::BlockNumber {
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

// Random Task Generation for (Future) Testing Purposes
impl<BlockNumber: std::convert::From<u64>> Task<BlockNumber> {
	// for testing purposes
	#[allow(dead_code)]
	fn random() -> Self {
		let mut rng = thread_rng();
		let random_score: u32 = rng.gen();
		let random_block: u64 = rng.gen();
		Self {
			id: id_generate(),
			score: random_score.into(),
			proposed_at: random_block.into(),
		}
	}
}
// helper method fo task id generation (see above `random` method)
pub fn id_generate() -> TaskId {
	let mut buf = vec![0u8; 32];
	OsRng.fill_bytes(&mut buf);
	buf.into()
}

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

thread_local! {
	static SIGNAL_QUOTA: RefCell<u32> = RefCell::new(0);
	static EXECUTION_FREQUENCY: RefCell<u64> = RefCell::new(0);
	static TASK_LIMIT: RefCell<u32> = RefCell::new(0);
}

pub struct SignalQuota;
impl Get<u32> for SignalQuota {
	fn get() -> u32 {
		SIGNAL_QUOTA.with(|v| *v.borrow())
	}
}

pub struct ExecutionFrequency;
impl Get<u64> for ExecutionFrequency {
	fn get() -> u64 {
		EXECUTION_FREQUENCY.with(|v| *v.borrow())
	}
}

pub struct TaskLimit;
impl Get<u32> for TaskLimit {
	fn get() -> u32 {
		TASK_LIMIT.with(|v| *v.borrow())
	}
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
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

mod execution_schedule {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		execution_schedule<T>,
		system<T>,
	}
}

impl Trait for TestRuntime {
	type Event = TestEvent;
	type SignalQuota = SignalQuota;
	type ExecutionFrequency = ExecutionFrequency;
	type TaskLimit = TaskLimit;
}

pub type System = system::Module<TestRuntime>;
pub type ExecutionSchedule = Module<TestRuntime>;

pub struct ExtBuilder {
	signal_quota: u32,
	execution_frequency: u64,
	task_limit: u32,
}
impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			signal_quota: 100u32,
			execution_frequency: 5u64,
			task_limit: 10u32,
		}
	}
}

impl ExtBuilder {
	pub fn signal_quota(mut self, signal_quota: u32) -> Self {
		self.signal_quota = signal_quota;
		self
	}
	pub fn execution_frequency(mut self, execution_frequency: u64) -> Self {
		self.execution_frequency = execution_frequency;
		self
	}
	pub fn task_limit(mut self, task_limit: u32) -> Self {
		self.task_limit = task_limit;
		self
	}
	pub fn set_associated_consts(&self) {
		SIGNAL_QUOTA.with(|v| *v.borrow_mut() = self.signal_quota);
		EXECUTION_FREQUENCY.with(|v| *v.borrow_mut() = self.execution_frequency);
		TASK_LIMIT.with(|v| *v.borrow_mut() = self.task_limit);
	}
	pub fn build(self) -> TestExternalities {
		self.set_associated_consts();
		let t = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		// GenesisConfig::<TestRuntime> {
		//     council_members: vec![1, 2, 3, 4, 5, 6],
		// }.assimilate_storage(&mut t).unwrap();
		t.into()
	}
}

/// Auxiliary method for simulating block time passing
fn run_to_block(n: u64) {
	while System::block_number() < n {
		ExecutionSchedule::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		ExecutionSchedule::on_initialize(System::block_number() + 1);
	}
}

#[test]
fn eras_change_correctly() {
	ExtBuilder::default()
		.execution_frequency(2)
		.build()
		.execute_with(|| {
			System::set_block_number(1);
			run_to_block(13);
			assert_eq!(ExecutionSchedule::era(), 6);
			run_to_block(32);
			assert_eq!(ExecutionSchedule::era(), 16);
		})
}

#[test]
fn naive_estimator_works() {
	ExtBuilder::default()
		.execution_frequency(8)
		.build()
		.execute_with(|| {
			let current_block = 5u64;
			assert_eq!(
				ExecutionSchedule::naive_execution_estimate(current_block.into()),
				8u64.into()
			);
			assert_eq!(
				ExecutionSchedule::execution_estimate(current_block.into()),
				8u64.into()
			);
			let next_block = 67u64;
			assert_eq!(
				ExecutionSchedule::naive_execution_estimate(next_block.into()),
				72u64.into()
			);
			assert_eq!(
				ExecutionSchedule::execution_estimate(next_block.into()),
				72u64.into()
			);
		})
}

#[test]
fn estimator_works() {
	ExtBuilder::default()
		.execution_frequency(8)
		.build()
		.execute_with(|| {
			let current_block = 5u64;
			assert_eq!(
				ExecutionSchedule::execution_estimate(current_block.into()),
				8u64.into()
			);
			let next_block = 67u64;
			assert_eq!(
				ExecutionSchedule::execution_estimate(next_block.into()),
				72u64.into()
			);
		})
}

#[test]
fn schedule_task_behaves() {
	ExtBuilder::default()
		.execution_frequency(10)
		.build()
		.execute_with(|| {
			ExecutionSchedule::add_member(1);
			assert!(ExecutionSchedule::is_on_council(&1));
			System::set_block_number(2);
			let new_task = id_generate();
			assert_ok!(ExecutionSchedule::schedule_task(Origin::signed(1), new_task.clone()));

			// check storage changes
			let expected_task: Task<u64> = Task {
				id: new_task.clone(),
				score: 0u32,
				proposed_at: 2u64,
			};
			assert_eq!(
				ExecutionSchedule::pending_tasks(new_task.clone()).unwrap(),
				expected_task
			);
			assert_eq!(ExecutionSchedule::execution_queue(), vec![new_task.clone()]);

			// check event behavior
			let expected_event = TestEvent::execution_schedule(RawEvent::TaskScheduled(
				1,
				new_task,
				10,
			));
			assert!(System::events().iter().any(|a| a.event == expected_event));
		})
}

#[test]
fn priority_signalling_behaves() {
	ExtBuilder::default()
		.execution_frequency(5)
		.signal_quota(10)
		.task_limit(1)
		.build()
		.execute_with(|| {
			System::set_block_number(2u64);
			let new_task = id_generate();
			ExecutionSchedule::add_member(1);
			ExecutionSchedule::add_member(2);

			// refresh signal_quota
			run_to_block(7u64);

			assert_ok!(ExecutionSchedule::schedule_task(Origin::signed(2), new_task.clone()));

			assert_ok!(ExecutionSchedule::signal_priority(
				Origin::signed(1),
				new_task.clone(),
				2u32.into(),
			));

			// check that banked signal has decreased
			assert_eq!(
				ExecutionSchedule::signal_bank(1u32, 1),
				8u32.into()
			);

			// check that task priority has increased
			assert_eq!(
				ExecutionSchedule::pending_tasks(new_task.clone())
					.unwrap()
					.score,
				2u32.into()
			);
		})
}
