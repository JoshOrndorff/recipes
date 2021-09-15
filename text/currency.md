# Currency Types

`pallets/lockable-currency`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Flockable-currency%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/lockable-currency/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

`pallets/reservable-currency`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Freservable-currency%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/reservable-currency/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

`pallets/currency-imbalances`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fcurrency-imbalances%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/currency-imbalances/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>


## Just Plain Currency

To use a balance type in the runtime, import the
[`Currency`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html) trait from
`frame_support`.

```rust, ignore
use frame_support::traits::Currency;
```

The `Currency` trait provides an abstraction over a
[fungible assets](https://en.wikipedia.org/wiki/Fungibility) system. To use such a fungible asset
from your pallet, include an associated type with the `Currency` trait bound in your pallet's
configuration trait.

```rust, ignore
pub trait Config: frame_system::Config {
	type Currency: Currency<Self::AccountId>;
}
```

Defining an associated type with this trait bound allows this pallet to access the provided methods
of [`Currency`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html). For example, it
is straightforward to check the total issuance of the system:

```rust, ignore
// in decl_module block
T::Currency::total_issuance();
```

As promised, it is also possible to type alias a balances type for use in the runtime:

```rust, ignore
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
```

This new `BalanceOf<T>` type satisfies the type constraints of `Self::Balance` for the provided
methods of `Currency`. This means that this type can be used for
[transfer](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html#tymethod.transfer),
[minting](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html#tymethod.deposit_into_existing),
and much more.

## Reservable Currency

Substrate's [Treasury pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_treasury/index.html) uses the
`Currency` type for bonding spending proposals. To reserve and unreserve funds for bonding,
`treasury` uses the
[`ReservableCurrency`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.ReservableCurrency.html)
trait. The import and associated type declaration follow convention

```rust, ignore
use frame_support::traits::{Currency, ReservableCurrency};

pub trait Config: frame_system::Config {
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}
```

To lock or unlock some quantity of funds, it is sufficient to invoke `reserve` and `unreserve`
respectively

```rust, ignore
pub fn reserve_funds(origin, amount: BalanceOf<T>) -> DispatchResult {
	let locker = ensure_signed(origin)?;

	T::Currency::reserve(&locker, amount)
			.map_err(|_| "locker can't afford to lock the amount requested")?;

	let now = <frame_system::Module<T>>::block_number();

	Self::deposit_event(RawEvent::LockFunds(locker, amount, now));
	Ok(())
}
```

```rust, ignore
pub fn unreserve_funds(origin, amount: BalanceOf<T>) -> DispatchResult {
	let unlocker = ensure_signed(origin)?;

	T::Currency::unreserve(&unlocker, amount);
	// ReservableCurrency::unreserve does not fail (it will lock up as much as amount)

	let now = <frame_system::Module<T>>::block_number();

	Self::deposit_event(RawEvent::UnlockFunds(unlocker, amount, now));
	Ok(())
}
```

## Lockable Currency

Substrate's [Staking pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_staking/index.html) similarly uses
[`LockableCurrency`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.LockableCurrency.html)
trait for more nuanced handling of capital locking based on time increments. This type can be very
useful in the context of economic systems that enforce accountability by collateralizing fungible
resources. Import this trait in the usual way

```rust, ignore
use frame_support::traits::{LockIdentifier, LockableCurrency}
```

To use `LockableCurrency`, it is necessary to define a
[`LockIdentifier`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/type.LockIdentifier.html).

```rust, ignore
const EXAMPLE_ID: LockIdentifier = *b"example ";
```

By using this `EXAMPLE_ID`, it is straightforward to define logic within the runtime to schedule
locking, unlocking, and extending existing locks.

```rust, ignore
fn lock_capital(origin, amount: BalanceOf<T>) -> DispatchResult {
	let user = ensure_signed(origin)?;

	T::Currency::set_lock(
		EXAMPLE_ID,
		&user,
		amount,
		WithdrawReasons::except(WithdrawReason::TransactionPayment),
	);

	Self::deposit_event(RawEvent::Locked(user, amount));
	Ok(())
}
```

## Imbalances

Functions that alter balances return an object of the
[`Imbalance`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Imbalance.html) type to express
how much account balances have been altered in aggregate. This is useful in the context of state
transitions that adjust the total supply of the `Currency` type in question.

To manage this supply adjustment, the
[`OnUnbalanced`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.OnUnbalanced.html) handler is
often used. An example might look something like

```rust, ignore
pub fn reward_funds(origin, to_reward: T::AccountId, reward: BalanceOf<T>) {
	let _ = ensure_signed(origin)?;

	let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

	let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
	total_imbalance.maybe_subsume(r);
	T::Reward::on_unbalanced(total_imbalance);

	let now = <frame_system::Module<T>>::block_number();
	Self::deposit_event(RawEvent::RewardFunds(to_reward, reward, now));
}
```

## takeaway

The way we represent value in the runtime dictates both the security and flexibility of the
underlying transactional system. Likewise, it is convenient to be able to take advantage of Rust's
[flexible trait system](https://blog.rust-lang.org/2015/05/11/traits.html) when building systems
intended to rethink how we exchange information and value 🚀
