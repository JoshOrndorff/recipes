# Handling Errors

_[`pallets/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/adding-machine)_

As we've mentioned before, in Substrate development it is important to **Verify first, write
last**. In this recipe, we'll create an adding machine that checks for unlucky numbers (a silly example)
as well as integer overflow (a serious and realistic example), and throws the appropriate errors.

## Declaring Errors

Errors are declared with the
[`decl_error!` macro](https://substrate.dev/rustdocs/v2.0.0-rc2/frame_support/macro.decl_error.html). Although it is
optional, it is good practice to write doc comments for each error variant as demonstrated here.

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

## Throwing Errors in `match` Statement

Errors can be thrown in two different ways, both of which are demonstrated in the the `add`
dispatchable call. The first is with the
[`ensure!` macro](https://substrate.dev/rustdocs/v2.0.0-rc2/frame_support/macro.ensure.html) where the error to throw
is the second parameter. The second is to throw the error by explicitly returning it.

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

Notice that the `Error` type always takes the generic parameter `T`. Notice also that we have
verified all preconditions, and thrown all possible errors before ever writing to storage.

## Throwing Errors with `.ok_or` and `.map_err`

In fact, the pattern of:

-   calling functions that returned a `Result` or `Option`, and
-   checking if the result is `Some` or `Ok` and if not, return from the function early with an error

are so common that there are two standard Rust methods help performing the task.

```rust, ignore
fn add_alternate(origin, val_to_add: u32) -> DispatchResult {
	let _ = ensure_signed(origin)?;

	ensure!(val_to_add != 13, <Error<T>>::UnluckyThirteen);

	// Using `ok_or()` to check if the returned value is `Ok` and unwrap the value.
	//   If not, returns error from the function.
	let result = Self::sum().checked_add(val_to_add).ok_or(<Error<T>>::SumTooLarge)?;

	Sum::put(result);
	Ok(())
}
```

Notice the pattern of `.ok_or(<Error<T>>::MyError)?;`. This is really handy when you have a function
call that returns an `Option` and you expect there should be a value inside. If not, return early
with an error message, all the while unwrapping the value for your further processing.

If your function returns a `Result<T, E>`, you could apply `.map_err(|_e| <Error<T>>::MyError)?;` in
the same spirit.

## Constructing the Runtime

Unlike before, adding errors to our pallet does _not_ require a change to the line in
`construct_runtime!`. This is just an idiosyncrasy of developing in Substrate.
