# Handling Errors
*[`pallets/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/adding-machine)*

As we've mentioned before, in Substrate development, it is important to **Verify first, write last**. In this recipe, we'll create an adding machine checks for unlucky numbers (a silly example) as well as integer overflow (a serious and realistic example), and throws the appropriate errors.

## Declaring Errors

Errors are declared with the [`decl_error!` macro](https://substrate.dev/rustdocs/master/frame_support/macro.decl_error.html). Although it is optional, it is good practice to write doc comments for each error variant as demonstrated here.

```rust, ignore
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Thirteen is unlucky and prohibitted
		UnluckyThirteen,
		/// Sum would have overflowed if we had added
		SumTooLarge,
	}
}
```

## Throwing Errors

Errors can be thrown in two different ways, both of which are demonstrated in the the `add` dispatchable call. The first is with the [`ensure!` macro](https://substrate.dev/rustdocs/master/frame_support/macro.ensure.html) where the error to throw is the second parameter. The second is to throw the error by explicitly returning it.

```rust, ignore
fn add(origin, val_to_add: u32) -> DispatchResult {
	let _ = ensure_signed(origin)?;

	// First check for unlucky number 13
	ensure!(val_to_add != 13, <Error<T>>::UnluckyThirteen);

	// Now check for overflow while adding
	let result = match Self::sum().checked_add(val_to_add) {
		Some(r) => r,
		None => return Err(<Error<T>>::SumTooLarge.into()),
	};

	// Write the new sum to storage
	Sum::put(result);

	Ok(())
}
```

Notice that the `Error` type always takes the generic parameter `T`. Notice also that we have verified all preconditions, and thrown all possible errors before ever writing to storage.

## Constructing the Runtime

Unlike before, adding errors to our pallet does _not_ require a change to the line in `construct_runtime!`. This is just an idiosyncrasy of developing in Substrate.
