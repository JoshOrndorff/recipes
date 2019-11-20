```rust

#[cfg(test)]
mod tests {
    use crate::{Module, Trait};
    use super::RawEvent;
    use primitives::H256;
    use runtime_io;
    use runtime_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use balances;
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

        pub const ExistentialDeposit: u64 = 0;
        pub const TransferFee: u64 = 0;
        pub const CreationFee: u64 = 0;

        pub const SpendPeriod: u64 = 5;
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

    impl balances::Trait for Test {
        type Balance = u64;
        type OnFreeBalanceZero = ();
        type OnNewAccount = ();
        type Event = ();
        type TransferPayment = ();
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type TransferFee = TransferFee;
        type CreationFee = CreationFee;
    }
    
    mod treasury {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            treasury<T>,
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
        type Currency = balances::Module<Self>;
        type SpendPeriod = SpendPeriod;
    }

    pub type System = system::Module<TestRuntime>;
    pub type Balances = balances::Module<TestRuntime>;
    pub type Treasury = Module<TestRuntime>;

    // An alternative to `ExtBuilder` which includes custom configuration
    pub fn new_test_ext() -> runtime_io::TestExternalities {
        let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
        balances::GenesisConfig::<Test> {
            balances: vec![
                (1, 13),
                (2, 11),
                (3, 8),
                (4, 3),
                (5, 19),
                (6, 23),
                (7, 17),
            ],
            vesting: vec![],
        }.assimilate_storage(&mut t).unwrap();
        t.into()
    }

    /// Auxiliary method for simulating block time passing
    fn run_to_block(n: u64) {
        while System::block_number() < n {
            MyModule::on_finalize(System::block_number());
            System::on_finalize(System::block_number());
            System::set_block_number(System::block_number() + 1);
            System::on_initialize(System::block_number());
            MyModule::on_initialize(System::block_number());
        }
    }

    #[test]
    fn proxy_transfer_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(9);
            let first = ensure_signed(Origin::signed(1));
            let second = ensure_signed(Origin::signed(2));
            Treasury::proxy_transfer(Origin::signed(1), 2, 11);

            let expected_event = TestEvent::treasury(RawEvent::ProxyTransfer(first, second));
            assert!(System::events().iter().any(|a| a.event == expected_event));

            run_to_block(11);

            let expected_event = TestEvent::treasury(RawEvent::SpendExecute(second, 11));
            assert!(System::events().iter().any(|a| a.event == expected_event));

        })
    }
}
```