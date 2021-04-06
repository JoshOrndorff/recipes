# Simple Crowdfund

`pallets/simple-crowdfund`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fsimple-crowdfund%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/simple-crowdfund/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

This pallet demonstrates a simple on-chain crowdfunding app where participants can pool funds toward
a common goal. It demonstrates a pallet that controls multiple token accounts, and storing data in
[child storage](https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/child/index.html).

## Basic Usage

Any user can start a crowdfund by specifying a goal amount for the crowdfund, an end time, and a
beneficiary who will receive the pooled funds if the goal is reached by the end time. If the fund is
not successful, it enters into a retirement period when contributors can reclaim their pledged
funds. Finally, an unsuccessful fund can be dissolved, sending any remaining tokens to the user who
dissolves it.

## Configuration Trait

We begin by declaring our configuration trait. In addition to the ubiquitous `Event` type, our
crowdfund pallet will depend on a notion of
[`Currency`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html), and three
[configuration constants](./constants.md).

```rust, ignore
/// The pallet's configuration trait
pub trait Config: frame_system::Config {
	/// The ubiquious Event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// The currency in which the crowdfunds will be denominated
	type Currency: ReservableCurrency<Self::AccountId>;

	/// The amount to be held on deposit by the owner of a crowdfund
	type SubmissionDeposit: Get<BalanceOf<Self>>;

	/// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
	/// least ExistentialDeposit.
	type MinContribution: Get<BalanceOf<Self>>;

	/// The period of time (in blocks) after an unsuccessful crowdfund ending during which
	/// contributors are able to withdraw their funds. After this period, their funds are lost.
	type RetirementPeriod: Get<Self::BlockNumber>;
}
```

## Custom Types

Our pallet introduces a custom struct that is used to store the metadata about each fund.

```rust, ignore
#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FundInfo<AccountId, Balance, BlockNumber> {
	/// The account that will recieve the funds if the campaign is successful
	beneficiary: AccountId,
	/// The amount of deposit placed
	deposit: Balance,
	/// The total amount raised
	raised: Balance,
	/// Block number after which funding must have succeeded
	end: BlockNumber,
	/// Upper bound on `raised`
	goal: Balance,
}
```

In addition to this `FundInfo` struct, we also introduce an index type to track the number of funds
that have ever been created and three convenience aliases.

```rust, ignore
pub type FundIndex = u32;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type FundInfoOf<T> = FundInfo<AccountIdOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;
```

## Storage

The pallet has two storage items declared the usual way using `decl_storage!`. The first is the
index that tracks the number of funds, and the second is a mapping from index to `FundInfo`.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config> as ChildTrie {
		/// Info on all of the funds.
		Funds get(fn funds):
			map hasher(blake2_128_concat) FundIndex => Option<FundInfoOf<T>>;

		/// The total number of funds that have so far been allocated.
		FundCount get(fn fund_count): FundIndex;

		// Additional information is stored in a child trie. See the helper
		// functions in the impl<T: Config> Module<T> block below
	}
}
```

This pallet also stores the data about which users have contributed and how many funds they
contributed in a [child trie](https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/child/index.html). This
child trie is not explicitly declared anywhere.

The use of the child trie provides two advantages over using standard storage. First, it allows for
removing the entirety of the trie is a single storage write when the fund is dispensed or dissolved.
Second, it allows any contributor to prove that they contributed using a
[Merkle Proof](https://medium.com/crypto-0-nite/merkle-proofs-explained-6dd429623dc5).

### Using the Child Trie API

The child API is abstracted into a few helper functions in the `impl<T: Config> Module<T>` block.

```rust, ignore
/// Record a contribution in the associated child trie.
pub fn contribution_put(index: FundIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
	let id = Self::id_from_index(index);
	who.using_encoded(|b| child::put(&id, b, &balance));
}

/// Lookup a contribution in the associated child trie.
pub fn contribution_get(index: FundIndex, who: &T::AccountId) -> BalanceOf<T> {
	let id = Self::id_from_index(index);
	who.using_encoded(|b| child::get_or_default::<BalanceOf<T>>(&id, b))
}

/// Remove a contribution from an associated child trie.
pub fn contribution_kill(index: FundIndex, who: &T::AccountId) {
	let id = Self::id_from_index(index);
	who.using_encoded(|b| child::kill(&id, b));
}

/// Remove the entire record of contributions in the associated child trie in a single
/// storage write.
pub fn crowdfund_kill(index: FundIndex) {
	let id = Self::id_from_index(index);
	child::kill_storage(&id);
}
```

Because this pallet uses one trie for each active crowdfund, we need to generate a unique
[`ChildInfo`](https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/child/enum.ChildInfo.html) for each of
them. To ensure that the ids are really unique, we incluce the `FundIndex` in the generation.

```rust, ignore
pub fn id_from_index(index: FundIndex) -> child::ChildInfo {
	let mut buf = Vec::new();
	buf.extend_from_slice(b"crowdfnd");
	buf.extend_from_slice(&index.to_le_bytes()[..]);

	child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
}
```

## Pallet Dispatchables

The dispatchable functions in this pallet follow a standard flow of verifying preconditions, raising
appropriate errors, mutating storage, and finally emitting events. We will not present them all in
this writeup, but as always, you're encouraged to experiment with the recipe.

We will look closely only at the `dispense` dispatchable which pays the funds to the beneficiary
after a successful crowdfund. This dispatchable, as well as `dissolve`, use an incentivization
scheme to encourage users of the chain to eliminate extra data as soon as possible.

Data from finished funds takes up space on chain, so it is best to settle the fund and cleanup the
data as soon as possible. To incentivize this behavior, the pallet awards the initial deposit to
whoever calls the `dispense` function. Users, in hopes of receiving this reward, will race to call
these cleanup methods before each other.

```rust, ignore
/// Dispense a payment to the beneficiary of a successful crowdfund.
/// The beneficiary receives the contributed funds and the caller receives
/// the deposit as a reward to incentivize clearing settled crowdfunds out of storage.
#[weight = 10_000]
fn dispense(origin, index: FundIndex) {
	let caller = ensure_signed(origin)?;

	let fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

	// Check that enough time has passed to remove from storage
	let now = <frame_system::Module<T>>::block_number();

	ensure!(now >= fund.end, Error::<T>::FundStillActive);

	// Check that the fund was actually successful
	ensure!(fund.raised >= fund.goal, Error::<T>::UnsuccessfulFund);

	let account = Self::fund_account_id(index);

	// Beneficiary collects the contributed funds
	let _ = T::Currency::resolve_creating(&fund.beneficiary, T::Currency::withdraw(
		&account,
		fund.raised,
		WithdrawReasons::from(WithdrawReason::Transfer),
		ExistenceRequirement::AllowDeath,
	)?);

	// Caller collects the deposit
	let _ = T::Currency::resolve_creating(&caller, T::Currency::withdraw(
		&account,
		fund.deposit,
		WithdrawReasons::from(WithdrawReason::Transfer),
		ExistenceRequirement::AllowDeath,
	)?);
```

This pallet also uses the Currency
[`Imbalance`](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Imbalance.html) trait as discussed in
the [Charity recipe](./charity.md), to make transfers without incurring transfer fees to the
crowdfund pallet itself.
