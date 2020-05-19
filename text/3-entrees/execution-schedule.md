# Execution Schedule
*[`pallets/execution-schedule`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/execution-schedule)*

<!-- TODO This content may need updated to match the actual pallet -->

Blockchain-native mechanisms may use the block number as a proxy for time to schedule task execution. Although scheduled task execution through council governance is minimal in this example, it is not too hard to imagine tasks taking the form of subscription payments, grant payouts, or any other scheduled *task* execution.

This pallet demonstrates a permissioned task scheduler, in which members of a `council: Vec<AccountId>` can schedule tasks, which are stored in a vector in the runtime storage (`decl_storage`).

Members of the `council` vote on the tasks with `SignalQuota` voting power which is doled out equally to every member every `ExecutionFrequency` number of blocks.

Tasks with support are prioritized during execution every `ExecutionFrequency` number of blocks. More specifically, every `ExecutionFrequency` number of blocks, a maximum of `TaskLimit` number of tasks are executed. The priority of tasks is decided by the signalling of the council members.

The module's `Trait`:

```rust, ignore
// other type aliases
pub type PriorityScore = u32;

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
```

The task object is a struct,

```rust, ignore
pub type TaskId = Vec<u8>;
pub type PriorityScore = u32;

pub struct Task<BlockNumber> {
    id: TaskId,
    score: PriorityScore,
    proposed_at: BlockNumber,
}
```

The runtime method for proposing a task emits an event with the expected execution time. The calculation of the expected execution time was first naively to basically iterate the block number from the current block number until it was divisible by `T::ExecutionFrequency::get()`. While this is correct, it is clearly not the most efficient way to find the next block in which tasks are executed.

> A more complex engine for predicting task execution time may run off-chain instead of in a runtime method.

Before adding a runtime method to estimate the `execution_time`, implement a naive implementation that iterates the global `BlockNumber` until it is divisible by `ExecutionFrequency` (which implies execution in `on_finalize` in this block).

```rust, ignore
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
```

This naive implementation unsurprisingly worked...

```rust, ignore
#[test]
fn naive_estimator_works() {
    // should use quickcheck to cover entire range of checks
    ExtBuilder::default()
        .execution_frequency(8)
        .build()
        .execute_with(|| {
            let current_block = 5u64;
            assert_eq!(
                ExecutionSchedule::naive_execution_estimate(current_block.into()),
                8u64.into()
            );
            let next_block = 67u64;
            assert_eq!(
                ExecutionSchedule::naive_execution_estimate(next_block.into()),
                72u64.into()
            );
        })
}
```


...but it is obvious that there is a better way. If execution is scheduled every constant `ExecutionFrequency` number of blocks, then it should be straightforward to calculate the next execution block without this slow iterate and check modulus method. My first attempt at a better implementation of `execution_estimate(n: T::BlockNumber) -> T::BlockNumber` was

```rust, ignore
fn execution_estimate(n: T::BlockNumber) -> T::BlockNumber {
        let batch_frequency = T::ExecutionFrequency::get();
        let miss = n % batch_frequency;
        (n + miss) - batch_frequency
    }
```

The above code failed the `estimator_works` unit test

```rust, ignore
#[test]
fn estimator_works() {
    ExtBuilder::default()
        .execution_frequency(8)
        .build()
        .execute_with(|| {
            assert_eq!(
                ExecutionSchedule::execution_estimate(current_block.into()),
                8u64.into()
            );
            assert_eq!(
                ExecutionSchedule::execution_estimate(next_block.into()),
                72u64.into()
            );
        })
}
```

The error helped me catch the logic mistake and change it to

```rust, ignore
fn execution_estimate(n: T::BlockNumber) -> T::BlockNumber {
    let batch_frequency = T::ExecutionFrequency::get();
    let miss = n % batch_frequency;
    n + (batch_frequency - miss)
}
```

This makes more sense. Current block number `% T::ExecutionFrequency::get()` is, by definition of modulus, the number of blocks that the current block is past when tasks were last executed. To return the next block at which task execution is scheduled, the estimator adds the difference between `T::ExecutionFrequency::get()` and the modulus. This makes sense AND passes the `estimators_work()` test.

## on_initialize updates vote data and round information

Each period of task proposals and voting is considered a round, expressed as `RoundIndex: u32` such that the global round is stored in the runtime storage as `Era`.

```rust, ignore
pub type RoundIndex = u32;

decl_storage! {
    trait Store for Module<T: Trait> as ExecutionSchedule {
        Era get(fn era): RoundIndex;
    }
}
```

This storage value acts as a global counter of the round, which is also used as the `prefix_key` of a `double_map` that tracks the member's remaining voting power in the `SignalBank` runtime storage item. This map and the round counter are updated in the `on_initialize` hook.

```rust, ignore
// in on_initialize
let last_era = <Era>::get();
<SignalBank<T>>::remove_prefix(&last_era);
let next_era: RoundIndex = last_era + 1;
<Era>::put(next_era);
// see next code back
```

The `SignalBank` tracks the signalling power of each member of the `council`. By using a `double-map` with the prefix as the round number, it is straightforward to perform batch removal of state related to signalling in the previous round.

```rust, ignore
<SignalBank<T>>::remove_prefix(&last_era);
```

In practice, this organization of logic uses something like a ring buffer; the `on_initialize` both batch deletes all signalling records from the previous round while, in the same code block, doling out an equal amount of voting power to all members for the next round.

```rust, ignore
// ...continuation of last code block
let signal_quota = T::SignalQuota::get();
<Council<T>>::get().into_iter().for_each(|member| {
    <SignalBank<T>>::insert(next_era, &member, signal_quota);
});
```

The aforementioned ring buffer is maintained in the `on_initialize` block. The maintenance code is kept in an if statement that limits its invocation to blocks `x` that follow blocks `y` for which `y % ExecutionFrequency == 0`.

This is a common way of only exercising expensive batch execution functions every periodic number of blocks. Still, the second to last statement is confusing. The first time I encountered the problem, I placed the following in the `on_initialize` if statement that controls the maintenance of the `SignalBank` and `Era` storage values,

```rust, ignore
// in on_initialize(n: T::BlockNumber)
if (n % (T::ExecutionFrequency + 1.into())).is_zero() {
    //changing and repopulating of `Era` and `SignalBank`
}
```

I only noticed this mistake while testing whether eras progress as expected. Specifically, the following test failed

```rust, ignore
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
```

The test failed with an error message claiming that the first `assert_eq!` left side was 4 which does not equal 6. This error message caused me to inspect the if condition, which I realized should be changed to (the current implementation),

```rust, ignore
// in on_initialize(n: T::BlockNumber)
if ((n - 1.into()) % T::ExecutionFrequency).is_zero() {
    //changing and repopulating of `Era` and `SignalBank`
}
```

With this change, the `eras_change_correctly` test passes.

## on_finalize execution priority <a name = "priority"></a>

* this pattern of sorting the tasks in `on_finalize` is inspired by the `scored-pool` pallet which should be referenced
* when we schedule and reprioritize elements in this way, order of execution becomes extremely important

* we execute tasks in `on_finalize` when `n % T::ExecutionFrequency == 0`. I should ensure that n != 0 as well, but I assume this is the case. The limit is maximum `TaskLimit`.
* An improvement would be to also ensure that their is some minimum amount of `score`. It would be nice to write abstractions that have a more native sense of the collective voting power of all members

* this lends itself to a follow up off-chain workers example for how it fits between `on_finalize` of the last block and `on_initialize` of the next block `=>` there is this whole `execution-schedule` :p
