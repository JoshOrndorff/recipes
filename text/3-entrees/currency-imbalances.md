
# Currency Imbalances

`pallets/currency-imbalances`


[`Imbalance`](https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/traits/trait.Imbalance.html) is used when tokens are burned or minted. In order to execute `imbalance` implement the [`OnUnbalanced`] (https://substrate.dev/rustdocs/v2.0.0-rc4/frame_support/traits/trait.OnUnbalanced.html)trait.
In this pallet a specific amount of funds will be slashed from an account and 
award a specific amount of funds to said specific account.

Slash funds:
pub fn slash_funds(origin, to_punish: T::AccountId, collateral: BalanceOf<T>) {
            let _ = ensure_signed(origin)?;

            let imbalance = T::Currency::slash_reserved(&to_punish, collateral).0;
            T::Slash::on_unbalanced(imbalance);

            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::SlashFunds(to_punish, collateral, now));
        }


Reward funds:
   pub fn reward_funds(origin, to_reward: T::AccountId, reward: BalanceOf<T>) {
            let _ = ensure_signed(origin)?;

            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

            let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
            total_imbalance.maybe_subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            let now = <system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::RewardFunds(to_reward, reward, now));
        }

