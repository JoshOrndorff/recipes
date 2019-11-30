# smpl-treasury
*[`kitchen/modules/smpl-treasury`](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/smpl-treasury)*

> the links don't work and the code is outdated but I'd like to keep some of the wording -- it is concise and still accurate

Otherwise, see *[the WIP](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/modules/smpl-treasury/README.md)*

## Instantiate a Pot

To instantiate a pool of funds, import [`ModuleId`](https://crates.parity.io/sr_primitives/struct.ModuleId.html) and [`AccountIdConversion`](https://crates.parity.io/sr_primitives/traits/trait.AccountIdConversion.html) from [`sr-primitives`](https://crates.parity.io/sr_primitives/index.html).

```rust
use runtime_primitives::{ModuleId, traits::AccountIdConversion};
```

With these imports, a `MODULE_ID` constant can be generated as an identifier for the pool of funds. This identifier can be converted into an `AccountId` with the `into_account()` method provided by the [`AccountIdConversion`](https://crates.parity.io/sr_primitives/traits/trait.AccountIdConversion.html) trait.

```rust
const MODULE_ID: ModuleId = ModuleId(*b"example ");

impl<T: Trait> Module<T> {
    pub fn account_id() -> T::AccountId {
		MODULE_ID.into_account()
	}

    fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}
}
```

Accessing the pot's balance is as simple as using the [`Currency`](https://crates.parity.io/srml_support/traits/trait.Currency.html) trait to access the balance of the associated `AccountId`.

## Proxy Transfers

In [srml/treasury](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs), approved spending proposals are queued in runtime storage before they are scheduled for execution. For the example dispatch queue, each entry represents a request to transfer `BalanceOf<T>` to `T::AccountId` from the pot.

```rust
decl_storage! {
	trait Store for Module<T: Trait> as STreasury {
		/// the amount, the address to which it is sent
		SpendQ get(fn spend_q): Vec<(T::AccountId, BalanceOf<T>)>;
	}
}
```

In other words, the dispatch queue holds the `AccountId` of the recipient (destination) in the first field of the tuple and the `BalanceOf<T>` in the second field. The runtime method for adding a spend request to the queue looks like this

```rust
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // uses the example treasury as a proxy for transferring funds
        fn proxy_transfer(origin, dest: T::AccountId, amount: BalanceOf<T>) -> Result {
            let sender = ensure_signed(origin)?;

            let _ = T::Currency::transfer(&sender, &Self::account_id(), amount)?;
            <SpendQ<T>>::mutate(|requests| requests.push((dest.clone(), amount)));
            Self::deposit_event(RawEvent::ProxyTransfer(dest, amount));
            Ok(())
        }
    }
}
```

This method transfers some funds to the pot along with the request to transfer the same funds from the pot to a recipient (the input field `dest: T::AccountId`).

NOTE: *Instead of relying on direct requests, [srml/treasury](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs) coordinates spending decisions through a proposal process.*

## Scheduling Spending

To schedule spending like [`srml/treasury`](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs), first add a configurable module constant in the `Trait`. This constant determines how often the spending queue is executed.

```rust
pub trait Trait: system::Trait {
    /// Period between successive spends.
	type SpendPeriod: Get<Self::BlockNumber>;
}
```

This constant is invoked in the runtime method [`on_finalize`](https://crates.parity.io/sr_primitives/traits/trait.OnFinalize.html) to schedule spending every `T::SpendPeriod::get()` blocks.

```rust
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // other runtime methods
        fn on_finalize(n: T::BlockNumber) {
            if (n % T::SpendPeriod::get()).is_zero() {
                Self::spend_funds();
            }
        }
    }
}
```

*To see the logic within `spend_funds`, see the [kitchen/treasury](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/treasury).* This recipe could be extended to give priority to certain spend requests or set a cap on the spends for a given `spend_funds()` call.
