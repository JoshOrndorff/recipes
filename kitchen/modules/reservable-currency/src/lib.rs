//! borrows collateral locking logic from treasury/lib.rs
// demonstrates https://crates.parity.io/srml_support/traits/trait.ReservableCurrency.html
use support::traits::{
	Currency, ReservableCurrency, ExistenceRequirement::AllowDeath
};
use support::{decl_event, decl_module, dispatch::Result};
use system::ensure_signed;

// balance type using reservable currency type
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait + Sized {
    // overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Currency type for this module.
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        LockFunds(AccountId, Balance, BlockNumber),
        UnlockFunds(AccountId, Balance, BlockNumber),
        // sender, dest, amount, block number
        TransferFunds(AccountId, AccountId, Balance, BlockNumber),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn lock_funds(origin, amount: BalanceOf<T>) -> Result {
            let locker = ensure_signed(origin)?;

            T::Currency::reserve(&locker, amount)
                    .map_err(|_| "locker can't afford to lock the amount requested")?;

            let now = <system::Module<T>>::block_number();

            Self::deposit_event(RawEvent::LockFunds(locker, amount, now));
            Ok(())
        }

        pub fn unlock_funds(origin, amount: BalanceOf<T>) -> Result {
            let unlocker = ensure_signed(origin)?;

            T::Currency::unreserve(&unlocker, amount);
            // this function does not fail (it will lock up as much as amount)
            // https://crates.parity.io/srml_support/traits/trait.ReservableCurrency.html

            let now = <system::Module<T>>::block_number();

            Self::deposit_event(RawEvent::LockFunds(unlocker, amount, now));
            Ok(())
        }

        pub fn transfer_funds(origin, dest: T::AccountId, amount: BalanceOf<T>) -> Result {
            let sender = ensure_signed(origin)?;

            T::Currency::transfer(&sender, &dest, amount, AllowDeath)?;

            let now = <system::Module<T>>::block_number();

            Self::deposit_event(RawEvent::TransferFunds(sender, dest, amount, now));
            Ok(())
        }

        // might be useful in closed economic systems
        pub fn unreserve_and_transfer(
            origin,
            to_punish: T::AccountId,
            dest: T::AccountId,
            collateral: BalanceOf<T>
        ) -> Result {
            let _ = ensure_signed(origin)?; // dangerous because can be called with any signature (so dont do this in practice ever!)

            let unreserved = T::Currency::unreserve(&to_punish, collateral);
            T::Currency::transfer(&to_punish, &dest, unreserved, AllowDeath)?;

            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::TransferFunds(to_punish, dest, unreserved, now));

            Ok(())
        }
    }
}
