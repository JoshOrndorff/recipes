//! A Simple Charity which holds and governs a pot of funds.
//!
//! The Charity has a pot of funds. The Pot is unique because unlike other token-holding accounts,
//! it is not controlled by a cryptographic keypair. Rather it belongs to the pallet itself.
//! Funds can be added to the pot in two ways:
//! * Anyone can make a donation through the `donate` extrinsic.
//! * An imablance can be absorbed from somewhere else in the runtime.
//! Funds can only be allocated by a root call to the `allocate` extrinsic/
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use sp_runtime::{
    traits::{AccountIdConversion},
    ModuleId,
};

use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath, OnUnbalanced, Imbalance};
use frame_support::{
	decl_event,
	decl_module,
	decl_storage,
	dispatch::{DispatchResult, DispatchError},
};
use frame_system::{self as system, ensure_signed, ensure_root};

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

/// Hardcoded pallet ID; used to create the special Pot Account
/// Must be exactly 8 characters long
const PALLET_ID: ModuleId = ModuleId(*b"Charity!");

pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The currency type that the charity deals in
    type Currency: Currency<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as SimpleTreasury {
		// No storage items of our own, but we still need decl_storage to initialize the pot
	}
    add_extra_genesis {
        build(|_config| {
            // Create the charity's pot of funds, and ensure it has the minimum required deposit
            let _ = T::Currency::make_free_balance_be(
                &<Module<T>>::account_id(),
                T::Currency::minimum_balance(),
            );
        });
    }
}

decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        <T as system::Trait>::AccountId,
    {
		/// Donor has made a charitable donation to the charity
		DonationReceived(AccountId, Balance, Balance),
        /// An imbalance from elsewhere in the runtime has been absorbed by the Charity
		ImbalanceAbsorbed(Balance, Balance),
		/// Charity has allocated funds to a cause
		FundsAllocated(AccountId, Balance, Balance),
        /// For testing purposes, to impl From<()> for TestEvent to assign `()` to balances::Event
        NullEvent(u32), // u32 could be aliases as an error code for mocking setup
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Donate some funds to the charity
        fn donate(
            origin,
            amount: BalanceOf<T>
        ) -> DispatchResult {
            let donor = ensure_signed(origin)?;

            T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

            Self::deposit_event(RawEvent::DonationReceived(donor, amount, Self::pot()));
            Ok(())
        }

        /// Allocate the Charity's funds
		///
        /// Take funds from the Charity's pot and send them somewhere. This cal lrequires root origin,
		/// which means it must come from a governance mechanism such as Substrate's Democracy pallet.
        fn allocate(
            origin,
            dest: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

			// Make the transfer requested
			T::Currency::transfer(
				&Self::account_id(),
				&dest,
				amount,
				AllowDeath,
			).map_err(|_| DispatchError::Other("Can't make allocation"))?;

			//TODO what about errors here??

            Self::deposit_event(RawEvent::FundsAllocated(dest, amount, Self::pot()));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// The account ID that holds the Charity's funds
    pub fn account_id() -> T::AccountId {
        PALLET_ID.into_account()
    }

    /// The Charity's balance
    fn pot() -> BalanceOf<T> {
        T::Currency::free_balance(&Self::account_id())
    }
}

// This implementation allows the charity to be the recipient of funds that are burned elsewhere in
// the runtime. For eample, it could be transaction fees, consensus-related slashing, or burns that
// align incentives in other pallets.
impl<T: Trait> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
		let numeric_amount = amount.peek();

		// Must resolve into existing but better to be safe.
		let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

		Self::deposit_event(RawEvent::ImbalanceAbsorbed(numeric_amount, Self::pot()));
	}
}

#[cfg(test)]
mod tests {
    use crate::*;
    use balances;
    use sp_core::H256;
    use sp_io;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use frame_support::{assert_ok, assert_err, impl_outer_event, impl_outer_origin, parameter_types};
	use frame_system::RawOrigin;

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
    }

    impl balances::Trait for TestRuntime {
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

    mod charity {
        pub use crate::Event;
    }

    impl_outer_event! {
        pub enum TestEvent for TestRuntime {
            charity<T>,
        }
    }

    impl std::convert::From<()> for TestEvent {
        fn from(_unit: ()) -> Self {
            TestEvent::charity(RawEvent::NullEvent(6))
        }
    }

    impl Trait for TestRuntime {
        type Event = TestEvent;
        type Currency = balances::Module<Self>;
    }

    pub type System = system::Module<TestRuntime>;
    pub type Balances = balances::Module<TestRuntime>;
    pub type Charity = Module<TestRuntime>;

    // An alternative to `ExtBuilder` which includes custom configuration
    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = system::GenesisConfig::default()
            .build_storage::<TestRuntime>()
            .unwrap();
        balances::GenesisConfig::<TestRuntime> {
			// Provide some initial balances
            balances: vec![
                (1, 13),
                (2, 11),
                (3, 1),
                (4, 3),
                (5, 19),
            ],
            vesting: vec![],
        }
        .assimilate_storage(&mut t)
        .unwrap();
        t.into()
    }

    /// Verifying correct behavior of boilerplate
    #[test]
    fn new_test_ext_behaves() {
        new_test_ext().execute_with(|| {
            assert_eq!(Balances::free_balance(&1), 13);
        })
    }

    #[test]
    fn donations_work() {
        new_test_ext().execute_with(|| {
			// User 1 donates 10 of her 13 tokens
            assert_ok!(Charity::donate(Origin::signed(1), 10));

			// Charity should have 10 tokens
			assert_eq!(Charity::pot(), 10);

			// Donor should have 3 remaining
            assert_eq!(Balances::free_balance(&1), 3);

            // Check that the correct event is emitted
            let expected_event = TestEvent::charity(RawEvent::DonationReceived(1, 10, 10));
            assert!(System::events().iter().any(|a| a.event == expected_event));
        })
    }

	#[test]
    fn cant_donate_too_much() {
        new_test_ext().execute_with(|| {
			// User 1 donates 20 toekns but only has 13
            assert_err!(Charity::donate(Origin::signed(1), 20), "Can't make donation");
        })
    }

	#[test]
	fn imbalances_work() {
		new_test_ext().execute_with(|| {
			let imb = balances::NegativeImbalance::new(5);
			Charity::on_nonzero_unbalanced(imb);

			assert_eq!(Charity::pot(), 5);

			// Check that the correct event is emitted
			let expected_event = TestEvent::charity(RawEvent::ImbalanceAbsorbed(5, 5));
			assert!(System::events().iter().any(|a| a.event == expected_event));
		})
	}

	#[test]
	fn allocating_works() {
		new_test_ext().execute_with(|| {
			// Charity acquires 10 tokens from user 1
			assert_ok!(Charity::donate(Origin::signed(1), 10));

			// Charity allocates 5 tokens to user 2
			assert_ok!(Charity::allocate(RawOrigin::Root.into(), 2, 5));

			// Check that the correct event is emitted
			let expected_event = TestEvent::charity(RawEvent::FundsAllocated(2, 5, 5));
			assert!(System::events().iter().any(|a| a.event == expected_event));
		})
	}
	//TODO What if we try to allocate more funds than we have
	#[test]
	fn cant_allocate_too_much() {
		new_test_ext().execute_with(|| {
			// Charity acquires 10 tokens from user 1
			assert_ok!(Charity::donate(Origin::signed(1), 10));

			// Charity tries to allocates 20 tokens to user 2
			assert_err!(Charity::allocate(RawOrigin::Root.into(), 2, 20), "Can't make allocation");
		})
	}
}
