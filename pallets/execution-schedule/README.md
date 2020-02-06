# Execution Schedule

This pallet abstracts away the scheduling of task execution and governs the priority of tasks using a council of `AccountId`s stored in runtime storage. 

Although we could add more nuanced governance to the `Council`, the purpose of this example is to demonstrate how and when to use `on_initialize` and `on_finalize`.

This example also demonstrates
* generous usage of type aliases
* sorting tasks by priority in `on_finalize`
* signalling priority by council in the runtime (`signal_priority`)
* preparing on-chain state in `on_initialize`
* using a `double_map`
* unit testing with a mock runtime

The context of the example is scheduling the execution of tasks in a queue stored in the runtime. Tasks are structs declared like

```rust, ignore
pub struct Task<BlockNumber> {
    id: TaskId,
    priority_score: PriorityScore,
    proposed_at: Timestamp,
    expected_execution: BlockNumber,
}
```

I wrote more about this [here](../../../../src/testing/schedule.md).
