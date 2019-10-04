// simple version of treasury
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
    /// Period between successive spends.
    type SpendPeriod: Get<Self::BlockNumber>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // uses the example treasury as a proxy for transferring funds
        fn proxy_transfer(origin, dest: T::AccountId, amount: BalanceOf<T>) -> Result {
            let sender = ensure_signed(origin)?;

            let _ = T::Currency::transfer(&sender, &Self::account_id(), amount)?;
            <SpendQ<T>>::mutate(|requests| requests.push((dest.clone(), amount)));
            Self::deposit_event(RawEvent::ProxyTransfer(dest, amount));
            Ok(())
        }

        fn on_finalize(n: T::BlockNumber) {
            if (n % T::SpendPeriod::get()).is_zero() {
                Self::spend_funds();
            }
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as STreasury {
        /// the amount, the address to which it is sent
        SpendQ get(spend_q): Vec<(T::AccountId, BalanceOf<T>)>;
    }
}

decl_event!(
	pub enum Event<T>
	where
		Balance = BalanceOf<T>,
		<T as system::Trait>::AccountId
	{
		/// New spend requets.
		ProxyTransfer(AccountId, Balance),
        /// New spend execution
        SpendExecute(AccountId, Balance),
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

    fn spend_funds() {
        let mut budget_remaining = Self::pot();
        <SpendQ<T>>::get().into_iter().for_each(|request| {
            if request.1 <= budget_remaining {
                budget_remaining -= request.1;
                let _ = T::Currency::transfer(&Self::account_id(), &request.0, request.1);
            }
        });
    }
}
