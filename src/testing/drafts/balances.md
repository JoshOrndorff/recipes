# Balances Genesis Mock Config

To test with balances genesis config, it is necessary to implement the mock runtime with `impl balances::Trait`; we can seed the accounts in the `ExtBuilder::build()` method. Add to that method after the `mut storage declaration`:

```rust
let _ = balances::GenesisConfig::<Runtime>{
    balances: vec![
            (1, 10),
            (2, 100),
            (3, 500),
            (4, 50),
            (10, 25),
    ],
    vesting: vec![],
}.assimilate_storage(&mut storage);
```

Up top before you gotta do this

```rust
thread_local! {
    static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct ExistentialDeposit;
impl Get<u64> for ExistentialDeposit {
    fn get() -> u64 { EXISTENTIAL_DEPOSIT.with(|v| *v.borrow()) }
}

parameter_types! {
    pub const TransferFee: Balance = 0;
    pub const CreationFee: Balance = 0;
}
impl balances::Trait for Runtime {
    type Balance = u64;
    type OnFreeBalanceZero = ();
    type OnNewAccount = ();
    type Event = ();
    type TransferPayment = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type TransferFee = ();
    type CreationFee = ();
}
```