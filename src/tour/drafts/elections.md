# Elections Notes

**Using support::traits**
* `election` demonstrates how/when to use certain `support::trait`s
* I think that the voting from `democracy` should be moved to `election`

* `ok_or` error handling is very common

## WOW

**Complexity Management** (comments at top)
* all unbonded public operations need to be constant time
* by bonding, we can allow for linear time operations in terms of the prior public operations

* **taking a voting bond once when the voter starts voting and then giving it back when they become inactive is the way to do things**

* guess you can set the starting storage value with syntax like

```rust
pub VoterCount get(voter_count): SetIndex = 0;
```

* reaping inactive voters uses this 
```rust
who: <T::Lookup as StaticLookup>::Source

let who = T::Lookup::lookup(who)?
```

* killing `reporter` or `who` `=>` the management of the conditional path is much cleaner than using a messy match statement...

## Questions

* what does **presentation** mean? invalid presentation?
* (answer this question for others once you understand)

* why is the voters vector a `Vec<Option<T::AccountId>>` (why the `Option`)?
* why store the `VoterInfo` in a struct?

* should the proxy be a map to an `Option<T::AccountId>`? I think one value-holding account might have multiple vote-transaction-sending accounts

```rust
pub Proxy get(proxy): map T::AccountId => Option<T::AccountId>;
```

* adding `voter` and removing `voter` for valid bonding pattern `=>` use `reap_inactive_voter` for managing inactivity

* `mutate` seems excessive and unnecessary for iterating a storage value...I want to implement a better way...

## Ideas for Improvement

* creating a custom type for `DesiredSeats` so that can be changed as well `=>` if we want more or less desired seats or we want the `DesiredSeats` to follow some distribution based on capital staked in support?