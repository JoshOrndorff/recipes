// Batching Balance Transfers through an Account with
// automated, scheduled spending
//

use runtime_primitives::traits::{AccountIdConversion, Zero};
use runtime_primitives::ModuleId;
use support::traits::{Currency, Get, ReservableCurrency};
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
use parity_scale_codec::{Encode, Decode};
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
pub struct SpendRequest<T::AccountId, BalanceOf> {
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
        fn request_transfer(origin, dest: T::AccountId, amount: BalanceOf<T>) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(amount >= T::MinimumSpend::get(), "spend must be at least MinimumSpend");

            // if `value` > RequestCollateral {reserve RequestCollateral}
            // else reserve `value`
            let bond = Self::calculate_bond(value);
			T::Currency::reserve(&sender, bond)
				.map_err(|_| "Sender's balance too low")?;

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