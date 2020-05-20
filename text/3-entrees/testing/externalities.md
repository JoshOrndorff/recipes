# Custom Test Environment

[`execution-schedule`](../execution-schedule.md)'s configuration trait has three [configurable constants](../constants.md). For this mock runtime, the `ExtBuilder` defines setters to enable the `TestExternalities` instance for each unit test to configure the local test runtime environment with different value assignments. For context, the `Trait` for `execution-schedule`,

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

The mock runtime environment extends the [previously discussed](./mock.md) `ExtBuilder` pattern with fields for each configurable constant and a default implementation.

> This completes the [builder](https://youtu.be/geovSK3wMB8?t=729) pattern by defining a default configuraton to be used in a plurality of test cases while also providing setter methods to overwrite the values for each field.

```rust, ignore
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
```

The setter methods for each configurable constant are defined in the `ExtBuilder` methods. This allows each instance of `ExtBuilder` to set the constant parameters for the unit test in question.

```rust, ignore
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
    // more methods e.g. build()
}
```

To allow for separate copies of the constant objects to be used in each thread, the variables assigned as constants are declared as [`thread_local!`](https://substrate.dev/rustdocs/v2.0.0-alpha.8/thread_local/index.html),

```rust, ignore
thread_local! {
    static SIGNAL_QUOTA: RefCell<u32> = RefCell::new(0);
    static EXECUTION_FREQUENCY: RefCell<u64> = RefCell::new(0);
    static TASK_LIMIT: RefCell<u32> = RefCell::new(0);
}
```

Each configurable constant type also maintains unit structs with implementation of `Get<T>` from the type `T` assigned to the pallet constant in the mock runtime implementation.

```rust, ignore
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
```
The build method on `ExtBuilder` sets the associated constants before building the default storage configuration.

```rust, ignore
impl ExtBuilder {
    // setters
    pub fn set_associated_consts(&self) {
        SIGNAL_QUOTA.with(|v| *v.borrow_mut() = self.signal_quota);
        EXECUTION_FREQUENCY.with(|v| *v.borrow_mut() = self.execution_frequency);
        TASK_LIMIT.with(|v| *v.borrow_mut() = self.task_limit);
    }
    // build()
}
```

To build the default test environment, the syntax looks like

```rust, ignore
#[test]
fn fake_test() {
    ExtBuilder::default()
        .build()
        .execute_with(|| {
            // testing logic and checks
        })
}
```

To configure a test environment in which the `execution_frequency` is set to `2`, the `eras_change_correctly` test invokes the `execution_frequency` setter declared in as a method on `ExtBuilder`,

```rust, ignore
#[test]
fn fake_test2() {
    ExtBuilder::default()
        .execution_frequency(2)
        .build()
        .execute_with(|| {
            // testing logic and checks
        })
}
```

The test environment mocked above is actually used for the cursory and incomplete test `eras_change_correctly`. This test guided the structure of the if condition in `on_initialize` to periodically reset the `SignalBank` and increment the `Era`.

For more examples of the mock runtime scaffolding pattern used in [`execution-schedule`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/execution-schedule), see `balances/mock.rs` and `contract/tests.rs`.
