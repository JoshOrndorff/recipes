# Currency Types and Locking Techniques

To use the `Balance` type in our runtime, it is sufficient to import the [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) trait.

```rust
use support::traits::Currency;
```

The [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) trait provides an abstraction over a fungible assets system. To use the behavior defined in [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html), it is sufficient to include `Currency` in the trait bounds of a module type.

```rust
// included system::Trait inheritance because it's in my code
pub trait Trait: system::Trait {
    type Currency: Currency<Self::AccountId>;
}
```

By adding a type to our module that satisfies [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) in the trait bound, it is possible to define other types via type aliasing so that runtime types [inherit fungibility and other useful behavior](https://crates.parity.io/srml_support/traits/trait.Currency.html).

```rust
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
```

Specifically, the new `BalanceOf` type can check the total issuance in the system 

```rust
T::Currency::total_issuance();
```

Indeed, the [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) trait comes with many useful methods. For example, [`deposit_into_existing`](https://crates.parity.io/srml_support/traits/trait.Currency.html#tymethod.deposit_into_existing) mints `reward: BalanceOf<T>` to the free balance of `winner: &AccountId`.

```rust
T::Currency::deposit_into_existing(winner, reward)?;
```

The [`LockableCurrency`](https://crates.parity.io/srml_support/traits/trait.LockableCurrency.html) trait similarly provides nuanced handling of capital locking. This will prove useful in the context of economic systems that often align incentives and enforce accountability by collateralizing fungible resources. Import this trait and a few helper items from `srml/support`

```rust
use support::traits::{LockIdentifier, LockableCurrency}
```

Following the convention in [`srml/staking`](https://github.com/paritytech/substrate/blob/master/srml/staking/src/lib.rs), instantiate a lock identifier and assign the `Currency` 

```rust
const EXAMPLE_ID: LockIdentifier = *b"example ";

pub trait Trait: system::Trait {
    /// The lockable currency type
    type Currency: LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;

    // The length of a generic lock period
    type LockPeriod: Get<Self::BlockNumber>;
    ...
}
```

The [`LockableCurrency`](https://crates.parity.io/srml_support/traits/trait.LockableCurrency.html) trait comes with runtime methods for locking, unlocking, and extending existing locks.

```rust
fn lock_capital(origin, amount: BalanceOf<T>) -> Result {
    let user = ensure_signed(origin)?;

    T::Currency::set_lock(
        EXAMPLE_ID,
        user.clone(),
        amount,
        T::LockPeriod::get(),
        WithdrawReasons::except(WithdrawReason::TransactionPayment),
    );

    Self::deposit_event(RawEvent::Locked(user, amount));
    Ok(())
}
```

The [`ReservableCurrency`](https://crates.parity.io/srml_support/traits/trait.ReservableCurrency.html) trait offers another API for managing an account's liquidity. In [`srml/treasury`](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs), this trait is used instead of `LockableCurrency`; it does not require a lock identifier.

```rust
use support::traits::{Currency, ReservableCurrency};

pub trait Trait: system::Trait {
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}
```

To lock or unlock some quantity of funds, it is sufficient to invoke `reserve` and `unreserve` respectively

```rust
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
    // https://crates.parity.io/srml_support/traits/trait.ReservableCurrency.html

    let now = <system::Module<T>>::block_number();

    Self::deposit_event(RawEvent::LockFunds(unlocker, amount, now));
    Ok(())
}
```

The way by which we represent value in the runtime dictates both the security and flexibility of the underlying transactional system. Likewise, it is nice to be able to take advantage of Rust's [flexible trait system](https://blog.rust-lang.org/2015/05/11/traits.html) when building systems intended to rethink how we exchange information and value 🚀 

*worth checking out the [`Imbalance`](https://crates.parity.io/srml_support/traits/trait.Imbalance.html) and [`OnDilution`](https://crates.parity.io/srml_support/traits/trait.OnDilution.html#tymethod.on_dilution) traits*