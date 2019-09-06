# smpl-treasury

This recipe demonstrates how [srml/treasury](https://github.com/paritytech/substrate/blob/master/srml/treasury/src/lib.rs) instantiates a pot of funds and schedules funding. *See [kitchen/treasury](https://github.com/substrate-developer-hub/recipes/tree/master/kitchen/treasury) for the full code*

## instantiate a pot

```rust
use runtime_primitives::{ModuleId, traits::AccountIdConversion};

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

## proxy transfers

```rust
decl_storage! {
	trait Store for Module<T: Trait> as STreasury {
		/// the amount, the address to which it is sent
		SpendQ get(spend_q): Vec<(T::AccountId, BalanceOf<T>)>;
	}
}

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

## schedule spending

```rust
pub trait Trait: system::Trait {
    /// Period between successive spends.
	type SpendPeriod: Get<Self::BlockNumber>;
}

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
