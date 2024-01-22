# Basic Token

`pallets/basic-token`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fbasic-token%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/basic-token/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

This recipe demonstrates a simple but functional token in a pallet.

## Mapping Accounts to Balances

Mappings are a very powerful primitive. A _stateful_ cryptocurrency might store a mapping between
accounts and balances. Likewise, mappings prove useful when representing _owned_ data. By tracking
ownership with maps, it is easy manage permissions for modifying values specific to individual users
or groups.

## Storage Items

The primary storage item is the mapping between AccountIds and Balances described above. Every
account that holds tokens appears as a key in that map and its value is the number of tokens it
holds.

The next two storage items set the total supply of the token and keep track of whether the token has
been initialized yet.

```rust, ignore
#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type Balances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

#[pallet::type_value]
	pub(super) fn TotalSupplyDefaultValue<T: Config>() -> u64 {
		21000000
}

#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;
```

Because users can influence the keys in our storage map, we've chosen the `blake2_128_concat` hasher
as described in the recipe on [storage maps](./storage-maps.md)s.

## Events and Errors

The pallet defines events and errors for common lifecycle events such as successful and failed
transfers, and successful and failed initialization.

```rust, ignore
#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Token was initialized by user
		Initialized(T::AccountId),
		/// Tokens successfully transferred between users
		Transfer(T::AccountId, T::AccountId, u64), // (from, to, value)
}

#[pallet::error]
	pub enum Error<T> {
		/// Attempted to initialize the token after it had already been initialized.
		AlreadyInitialized,
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
}
```

## Initializing the Token

In order for the token to be useful, some accounts need to own it. There are many possible ways to
initialize a token including genesis config, claims process, lockdrop, and many more. This pallet
will use a simple process where the first user to call the `init` function receives all of the
funds. The total supply is hard-coded in the pallet in a fairly naive way: It is specified as the
default value in the `decl_storage!` block.

```rust ignore
#[pallet::weight(10_000)]
		pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(!Self::is_init(), <Error<T>>::AlreadyInitialized);

			<Balances<T>>::insert(sender, Self::total_supply());

			Init::<T>::put(true);
			Ok(().into())
}
```

As usual, we first check for preconditions. In this case that means making sure that the token is
not already initialized. Then we do any mutation necessary.

## Transferring Tokens

To transfer tokens, a user who owns some tokens calls the `transfer` method specifying the recipient
and the amount of tokens to transfer as parameters.

We again check for error conditions before mutating storage. In this case it is _not_ necessary to
check whether the token has been initialized. If it has not, nobody has any funds and the transfer
will simply fail with `InsufficientFunds`.

```rust, ignore
#[pallet::weight(10_000)]
		pub fn transfer(
			_origin: OriginFor<T>,
			to: T::AccountId,
			value: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances
			let updated_from_balance = sender_balance
				.checked_sub(value)
				.ok_or(<Error<T>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance
				.checked_add(value)
				.expect("Entire supply fits in u64; qed");

			// Write new balances to storage
			<Balances<T>>::insert(&sender, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(Event::Transfer(sender, to, value));
			Ok(().into())
}
```

## Don't Panic!

When adding the incoming balance, notice the peculiar `.expect` method. In Substrate, **your runtime must never panic**. To encourage careful thinking about your code, you use the `.expect`
method and provide a proof of why the potential panic will never happen.
