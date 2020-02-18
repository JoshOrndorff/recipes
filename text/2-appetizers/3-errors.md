# Handling Errors
*[`pallets/adding-machine`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/adding-machine)*

As we've mentioned before, in Substrate development, it is important to **Verify first, write last**. In this recipe, we'll create an adding machine checks for unlucky numbers (a silly example) as well as integer overflow (a serious and realistic example), and throws the appropriate errors.

## Declaring Custom Errors

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

## Throwing Custom Errors

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

## Throwing Generic Errors

`DispatchResult` returned from the above dispatchable call is defined as [`Result<(), DispatchError>`](https://substrate.dev/rustdocs/master/frame_support/dispatch/type.DispatchResult.html). Looking up on [`DispatchError`](https://substrate.dev/rustdocs/master/frame_support/dispatch/enum.DispatchError.html), you will see it is an enum containing a variant of `Other(&'static err)`.

This means you can also throw generic error created from mere string (your error message) as follows:

```rust, ignore
fn add(origin, val_to_add: u32) -> DispatchResult {
	// -- snip --
	let result = match Self::sum().checked_add(val_to_add).ok_or("addition overflow")?;
	Sum::put(result);
	Ok(())
}
```

Notice the pattern of `.ok_or("my error message")?;`. This is really handy when you have a function call that returns an `Option` and you expect there should be a value inside. If not, returns early with an error message, all the while unwrapping the value for your further processing.

If your function returns a `Result<T, E>`, you could apply `.map_err(|_e| "my error message")?;` in the same spirit.

## Constructing the Runtime

Unlike before, adding errors to our pallet does _not_ require a change to the line in `construct_runtime!`. This is just an idiosyncrasy of developing in Substrate.
