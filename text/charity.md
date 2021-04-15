# Charity

`pallets/charity`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fcharity%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/charity/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

The Charity pallet represents a simple charitable organization that collects funds into a pot that
it controls, and allocates those funds to the appropriate causes. It demonstrates two useful
concepts in Substrate development:

-   A pallet-controlled shared pot of funds
-   Absorbing imbalances from the runtime

## Instantiate a Pot

Our charity needs an account to hold its funds. Unlike other accounts, it will not be controlled by
a user's cryptographic key pair, but directly by the pallet. To instantiate such a pool of funds,
import [`ModuleId`](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/struct.ModuleId.html) and
[`AccountIdConversion`](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/traits/trait.AccountIdConversion.html)
from [`sp-runtime`](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/index.html).

```rust, ignore
use sp-runtime::{ModuleId, traits::AccountIdConversion};
```

With these imports, a `PALLET_ID` constant can be generated as an identifier for the pool of funds.
The `PALLET_ID` must be exactly eight characters long which is why we've included the exclamation
point. (Well, that and Charity work is just so exciting!) This identifier can be converted into an
`AccountId` with the `into_account()` method provided by the `AccountIdConversion` trait.

```rust, ignore
const PALLET_ID: ModuleId = ModuleId(*b"Charity!");

impl<T: Config> Module<T> {
	/// The account ID that holds the Charity's funds
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	/// The Charity's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}
}
```

# Receiving Funds

Our charity can receive funds in two different ways.

## Donations

The first and perhaps more familiar way is through charitable donations. Donations can be made
through a standard `donate` extrinsic which accepts the amount to be donated as a parameter.

```rust, ignore
fn donate(
		origin,
		amount: BalanceOf<T>
) -> DispatchResult {
		let donor = ensure_signed(origin)?;

		let _ = T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath);

		Self::deposit_event(RawEvent::DonationReceived(donor, amount, Self::pot()));
		Ok(())
}
```

## Imbalances

The second way the charity can receive funds is by absorbing imbalances created elsewhere in the
runtime. An [`Imbalance`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Imbalance.html) is
created whenever tokens are burned, or minted. Because our charity wants to _collect_ funds, we are
specifically interested in
[`NegativeImbalance`](https://substrate.dev/rustdocs/v3.0.0/pallet_balances/struct.NegativeImbalance.html)s.
Negative imbalances are created, for example, when a validator is slashed for violating consensus
rules, transaction fees are collected, or another pallet burns funds as part of an
incentive-alignment mechanism. To allow our pallet to absorb these imbalances, we implement the
[`OnUnbalanced` trait](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.OnUnbalanced.html).

```rust, ignore
use frame_support::traits::{OnUnbalanced, Imbalance};
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
		let numeric_amount = amount.peek();

		// Must resolve into existing but better to be safe.
		let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

		Self::deposit_event(RawEvent::ImbalanceAbsorbed(numeric_amount, Self::pot()));
	}
}
```

# Allocating Funds

In order for the charity to affect change with the funds it has collected it must be able to
allocate those funds. Our charity pallet abstracts the governance of where funds will be allocated
to the rest of the runtime. Funds can be allocated by a root call to the `allocate` extrinsic. One
good example of a governance mechanism for such decisions is Substrate's own
[Democracy pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_democracy/index.html).
