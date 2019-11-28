# Execution Schedule

This module abstracts away the scheduling of task execution and governs the priority of tasks using a council of `AccountId`s stored in runtime storage. 

Although we could add more nuanced governance to the `Council`, the purpose of this example is to demonstrate how and when to use `on_initialize` and `on_finalize`.

This example also demonstrates
* generous usage of type aliases
* sorting tasks by priority in `on_finalize`
* signalling priority by council in the runtime (`signal_priority`)
* preparing on-chain state in `on_initialize`
* using a `double_map`
* unit testing with a mock runtime

The context of the example is scheduling the execution of tasks in a queue stored in the runtime. Tasks are structs declared like

```rust
pub struct Task<BlockNumber> {
    id: TaskId,
    priority_score: PriorityScore,
    proposed_at: Timestamp,
    expected_execution: BlockNumber,
}
```

# stuck

Right now, it isn't compiling -- I'm getting this error and I believe it's hiding many more errors. I've been refactoring code around it but I need to compile at some point. the path forward might be rewriting it from scratch and incrementally compiling because I tested the syntax used in `balances` for the `VestingSchedule` in `struct-storage` and it worked perfectly.

> (I can't isolate the error, but I don't think I need to serde serialize because `balances` doesn't for `VestingScheduler`)

```rust
error[E0463]: can't find crate for `serde`
   --> modules/execution-schedule/src/lib.rs:87:1
    |
87  | / decl_storage! {
88  | |     trait Store for Module<T: Trait> as ExecutionSchedule {
89  | |         /// Outstanding tasks getter
90  | |         PendingTask get(fn pending_task): map TaskId => Option<Task<T::BlockNumber>>;
...   |
111 | |     }
112 | | }
    | |_^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `execution-schedule`.

To learn more, run the command again with --verbose.
```