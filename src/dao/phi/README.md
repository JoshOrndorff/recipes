# Fees Module Design

* open with Cantillon's effect
* lead with Dilution Safety Mechanism
* derive fees model based on actors and actions within this closed system...

The basic invariant followed in a closed incentive loop is that any collateral posted by applications should cover the rewards issued for member participation in the applicant's worst-case scenario. We vary the fee according to the amount of collateral posted. More collateral pays less over any arbitrary time period. *Future functionality might allow members to kick old applications by vote or automatically according to some rule.*

The basic idea is that we associate the `fed` type with a fees calculation that calculates acceptable rates to applicants. The `fed`, fee managenent system, is designed to be generic over any required fee calculations. 

* `node/runtime`
* `sr-primitives/weight.rs`
* `OnFeeChargedTrait`
* move `Imbalance`-dependent types from `Balances`
* abstract out fees as auctions...

## Fees as Auctions

Before getting all worked up about defining transaction fees in the context of blockchains, let's establish that this abstraction lives at the application layer. The fee management system presented here is not directly related to rent at the protocol layer. Even so, it can help the DAO manage economic security at the application layer in which funds can be raised and lost within a few transactions. 

* dilution safety mechanism
* look at parachain auction code and see how to target a specific application flow
* think in terms of flow rates `=>` `block_number` * 
* I want to make it so that people's votes are probabilistic `=>` the longer they stay in, the longer they promise to stay in (with fee penalty) (for proposals at least and then implement some quadratic discount)

1. parachain auction code https://github.com/paritytech/polkadot/blob/master/runtime/src/slots.rs

* (1.5) https://hackernoon.com/blockchain-fees-are-broken-here-are-3-proposals-to-fix-them-1f772e1530dd

**BALANCE** PAPER

3. how do I get this to fee-setting to poll conditions to adjust the fee accordingly; it needs to be able to exit at any time with a price while also allowing for an adjustment of per-block price

4. think of it as rent, but, because the collateral provides the dao with increased liquidity, locking in the rent results in a lower average per-block fee

* I do want to do something on circular buffer to make it useful for logging reputation `=>` maybe by using the average of the last 50 blocks in per-block fees...
* I want to be able to lower prices with the sale of call options? or maybe it is the purchase of puts?

