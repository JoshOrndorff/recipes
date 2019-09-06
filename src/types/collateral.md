# Currency Types and Locking Techniques

To use a balances type in the runtime, import the [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) trait from `srml/support`

```rust
use support::traits::Currency;
```

The [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) trait provides an abstraction over a fungible assets system. To use the behavior defined in [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html), include it in the trait bounds of a module type.

```rust
// included system::Trait inheritance because it's in my code
pub trait Trait: system::Trait {
    type Currency: Currency<Self::AccountId>;
}
```

Defining a module type with this trait bound allows the runtime to access the provided methods of [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html). For example, it is straightforward to check the total issuance of the system:

```rust
// in decl_module block
T::Currency::total_issuance();
```

As promised, it is also possible to type alias a balances type for use in the runtime:

```rust
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
```

This new `BalanceOf<T>` type satisfies the type constraints of `Self::Balance` for the provided methods of [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html). This means that this type can be used for [transfer](https://crates.parity.io/srml_support/traits/trait.Currency.html#tymethod.transfer), [minting](https://crates.parity.io/srml_support/traits/trait.Currency.html#tymethod.deposit_into_existing), and [much more](https://crates.parity.io/srml_support/traits/trait.Currency.html).

## Reservable Currency

[`srml/treasury`](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs) uses the `Currency` type for bonding spending proposals. To reserve and unreserve balances for bonding, `treasury` uses the [`ReservableCurrency`](https://crates.parity.io/srml_support/traits/trait.ReservableCurrency.html) trait. The import and module type declaration follow convention

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

    let now = <system::Module<T>>::block_number();

    Self::deposit_event(RawEvent::LockFunds(unlocker, amount, now));
    Ok(())
}
```

*The full code can be found in [`collateral/reservable`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/reservable) in the [kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/README.md)*

## Lockable Currency

[`srml/staking`](https://github.com/paritytech/substrate/blob/master/srml/staking/src/lib.rs) similarly uses [`LockableCurrency`](https://crates.parity.io/srml_support/traits/trait.LockableCurrency.html) trait for more nuanced handling of capital locking based on time increments. This type can be very useful in the context of economic systems that enforce accountability by collateralizing fungible resources. Import this trait in the usual way

```rust
use support::traits::{LockIdentifier, LockableCurrency}

pub trait Trait: system::Trait {
    /// The lockable currency type
    type Currency: LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;

    // Example length of a generic lock period
    type LockPeriod: Get<Self::BlockNumber>;
    ...
}
```

To use [`LockableCurrency`](https://crates.parity.io/srml_support/traits/trait.LockableCurrency.html), it is necessary to define a [`LockIdentifier`](https://crates.parity.io/srml_support/traits/type.LockIdentifier.html).

```rust
const EXAMPLE_ID: LockIdentifier = *b"example ";
```

By using this `EXAMPLE_ID`, it is straightforward to define logic within the runtime to schedule locking, unlocking, and extending existing locks.

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

*The full code can be found in [`collateral/lockable`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/lockable) in the [kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/README.md).*

## Imbalances

Functions that alter balances return an object of the [`Imbalance`](https://crates.parity.io/srml_support/traits/trait.Imbalance.html) type to express how much account balances have been altered in aggregate. This is useful in the context of state transitions that adjust the total supply of the `Currency` type in question.

To manage this supply adjustment, the [`OnUnbalanced`](https://crates.parity.io/srml_support/traits/trait.OnUnbalanced.html) handler is often used. An example might look something like 

```rust
// runtime method (ie decl_module block)
pub fn reward_funds(origin, to_reward: T::AccountId, reward: BalanceOf<T>) {
    let _ = ensure_signed(origin)?;

    let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

    let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
    total_imbalance.maybe_subsume(r);
    T::Reward::on_unbalanced(total_imbalance);

    let now = <system::Module<T>>::block_number();
    Self::deposit_event(RawEvent::RewardFunds(to_reward, reward, now));
}
```

*The full code can be found in [`collateral/imbalances`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/imbalances) in the [kitchen](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/README.md).*

## takeaways

The way by which we represent value in the runtime dictates both the security and flexibility of the underlying transactional system. Likewise, it is nice to be able to take advantage of Rust's [flexible trait system](https://blog.rust-lang.org/2015/05/11/traits.html) when building systems intended to rethink how we exchange information and value 🚀 

BONUS: *see [`OnDilution`](https://crates.parity.io/srml_support/traits/trait.OnDilution.html#tymethod.on_dilution)*