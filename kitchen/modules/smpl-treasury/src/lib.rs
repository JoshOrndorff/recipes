//! Batching Balance Transfers through an Automated Account
//!
//! # thinking about economic design while building modules
//!
//! 
#![cfg_attr(not(feature = "std"), no_std)]

use runtime_primitives::traits::{AccountIdConversion, Zero};
use runtime_primitives::ModuleId;
use support::traits::{Currency, Get, ReservableCurrency};
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
use system::{self, ensure_signed};
use rstd::prelude::*;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

const MODULE_ID: ModuleId = ModuleId(*b"example ");

pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The staking balance.
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    /// Minimum spend allowed for batched transfer
    type MinimumSpend: Get<BalanceOf<Self>>;
    /// The collateral required for each request (could change to Perbill::percent)
    type RequestCollateral: Get<BalanceOf<Self>>;
    /// Period between successive spends.
    type SpendPeriod: Get<Self::BlockNumber>;
}

#[derive(Encode, Decode)]
pub struct SpendRequest<AccountId, Balance> {
    to: AccountId,
    amount: Balance,
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        const MinimumSpend: BalanceOf<T> = T::MinimumSpend::get();
        const RequestCollateral: BalanceOf<T> = T::RequestCollateral::get();
        const SpendPeriod: T::BlockNumber = T::SpendPeriod::get();

        /// Request to Schedule Batch Transfer 
        fn request_batched_transfer(origin, dest: T::AccountId, amount: BalanceOf<T>) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(amount >= T::MinimumSpend::get(), "spend must be at least MinimumSpend");

            // if `value` > RequestCollateral {reserve RequestCollateral}
            // else reserve `value`
            let bond = Self::calculate_bond(value);
			T::Currency::reserve(&proposer, bond)
				.map_err(|_| "Proposer's balance too low")?;

            let requested_spend = SpendRequest {
                to: dest.clone(),
                amount: amount.clone(),
            };
            <SpendQ<T>>::append(&[requested_spend])?;
            Self::deposit_event(RawEvent::TransferRequest(dest, amount));
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::SpendPeriod::get()).is_zero() {
                // could reorder and reprioritize spends here
                Self::spend_funds();
            }
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Treasury {
        SpendQ get(fn spend_q): Vec<SpendRequest<T::AccountId, BalanceOf<T>>>;
    }
}

decl_event!(
	pub enum Event<T>
	where
		Balance = BalanceOf<T>,
		<T as system::Trait>::AccountId
	{
		/// New spend request (from, to, amount)
		TransferRequest(AccountId, AccountId, Balance),
        /// New spend execution (from, to, amount)
        SpendExecute(AccountId, AccountId, Balance),
	}
);

impl<T: Trait> Module<T> {
    // Add public immutables and private mutables.

    /// The account ID of the treasury pot.
    ///
    /// This actually does computation. If you need to keep using it, then make sure you cache the
    /// value and only call this once.
    pub fn account_id() -> T::AccountId {
        MODULE_ID.into_account()
    }

    fn pot() -> BalanceOf<T> {
        T::Currency::free_balance(&Self::account_id())
    }

    fn calculate_bond(value: BalanceOf<T>) -> BalanceOf<T> {
        // request collateral is set as x s.t. 0 < x =< `T::RequestCollateral` depending on 
        T::RequestCollateral::get().min(value);
    } 

    fn spend_funds() {
        let mut budget_remaining = Self::pot();
        // TODO: take iteration out of runtime and place in offchain worker
        <SpendQ<T>>::get().into_iter().for_each(|request| {
            if request.1 <= budget_remaining {
                budget_remaining -= request.1;
                let _ = T::Currency::transfer(&Self::account_id(), &request.0, &request.1);
                Self::deposit_event(RawEvent::SpendExecute(&request.0, &request.1));
            }
        });
    }
}

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