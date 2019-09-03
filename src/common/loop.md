# Scheduling Execution

Blockchain-based applications use the block number as a proxy for time for scheduling events. For example, the [`srml/treasury`](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs) module schedules spending according to a `SpendPeriod` constant (*see [constants](../storage/constants.md) to configure module constants*). Specifically, the runtime method `spend_funds()` is executed every block in which `block_number % SpendPeriod == 0` (i.e. every `SpendPeriod` blocks, spending occurs).

```rust
// decl_module block
fn on_finalize(n: T::BlockNumber) {
    // Check to see if we should spend some funds!
    if (n % T::SpendPeriod::get()).is_zero() {
        Self::spend_funds();
    }
}
```

*this snippet can be found in the `on_finalize` method of [treasury](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs)*

## Blockchain Event Loop

This pattern feels similar to the javascript *event loop*, which is how js code handles events (often one at a time). The loop runs continually, executing any tasks queued. It can have multiple task sources to guarantee execution order with each source, but the browser gets to choose which task to execute from each source upon every turn of the loop. 

Similar to how the browser negotiates task preference according to the application logic, our Substrate runtime can encode arbitrarily complex logic for queueing tasks in a storage map and dispatching them according to code placed in [`on_initialize`](https://crates.parity.io/sr_primitives/traits/trait.OnInitialize.html#method.on_initialize) or [`on_finalize`](https://crates.parity.io/sr_primitives/traits/trait.OnFinalize.html#method.on_finalize) methods.

Similar to how the browser gives preference to certain tasks within an event loop, we can define an order for task execution in `on_finalize` based on the logic within our application.

In the associated [event loop recipe](https://github.com/substrate-developer-hub/recipes/blob/master/kitchen/loop/src/lib.rs) in the [kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/), a generic pattern is exposed for queueing `Task`s, which are defined as 

```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Task<BlockNumber> {
    // the priority of the task relative to other tasks
    priority_score: PriorityScore,
    // time at which the task is initially queued
    proposed_at: BlockNumber,
}
```

`PriorityScore` is a type alias for `u32` that provides an additional criteria for ordering task execution within a block. The `on_finalize` method specifies that the execution (`execute_tasks()`) is scheduled every `ExecutionFrequency` number of blocks.

```rust
fn on_finalize(n: T::BlockNumber) {
    if (n % T::ExecutionFrequency::get()).is_zero() {
        // execute from the dispatchQ
        Self::execute_tasks(n);
    }
}
```

The `execute_tasks()` runtime method demonstrates how tasks are scheduled for execution based on the `PriorityScore` initially assigned by the proposer. 

```rust
pub fn execute_tasks(n: T::BlockNumber) {
    let mut execute_q = Vec::new(); 
    <TaskQ<T>>::get().into_iter().for_each(|h| {
        execute_q.push(<Tasks<T>>::get(h));
        // sort based on priority score and block number
        // requires adding `Ord` and `PartialOrd` to derive attribute on `Task` struct
        execute_q.sort();
        execute_q.iter().for_each(|t| {
            // this is where each task is executed
            // -- execution occurs in order based on sort()
        });
        <Tasks<T>>::remove(h); // here, we just remove executed tasks from the map
    });
    <TaskQ<T>>::kill();
    // emit execution event
    Self::deposit_event(RawEvent::TaskExecuted(n));
}
```

Although task execution in this example is minimal, tasks could easily take the shape of subscription payments, grant payouts, or any other scheduled service provision. As was mentioned before, [`srml/treasury`](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs) uses this pattern to schedule spending according to the logic in the `spend_funds` runtime method.