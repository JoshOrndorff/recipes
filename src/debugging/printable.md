# Debugging

Debugging is a necessary in all walks of software development, and blockchain is no exception. Most of the same tools used for general purpose Rust debugging also apply to Substrate. However, there are some restrictions when operating inside of a no_std environment like the Substrate runtime.


## Substrate's Own `print` Function

To facilitate the debugging of the runtime, Substrate provides extra tools for `Print` debugging (or tracing). The [`print` function](https://substrate.dev/rustdocs/master/sp_runtime/fn.print.html) can be used as a way to log the status of the runtime execution.

``` rust
use sp_runtime::print;
```
``` rust
// --snip--
pub fn do_something(origin) -> DispatchResult {
	print("Execute do_something");

	let who = ensure_signed(origin)?;
	let my_val: u32 = 777;

	Something::put(my_val);

	print("After storing my_val");

	Self::deposit_event(RawEvent::SomethingStored(my_val, who));
	Ok(())
}
// --snip--
```

Start the chain using the `RUST_LOG` environment variable to see the print logs.
``` sh
RUST_LOG=runtime=debug ./target/release/node-template --dev
```

The values are printed in the terminal or the standard output every time that the
runtime function gets called.
``` sh
2020-01-01 00:00:00 tokio-blocking-driver DEBUG runtime  Execute do_something
2020-01-01 00:00:00 tokio-blocking-driver DEBUG runtime  After storing my_val
```

## Printable Trait

The [`print` function](https://substrate.dev/rustdocs/master/sp_runtime/fn.print.html) works with any type that implements the [`Printable` trait](https://substrate.dev/rustdocs/master/sp_runtime/traits/trait.Printable.html). Substrate implements this trait for some types (`u8`, `u32`, `u64`, `usize`, `&[u8]`, `&str`) by default. You can also implement it for your own custom types. Here is an example of implementing it for a pallet's `Error` type.

``` rust
// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}
}

impl<T: Trait> Printable for Error<T> {
	fn print(&self) {
		match self {
			Error::NoneValue => "Unexpected value".print(),
			Error::StorageOverflow => "Value exceeded".print(),
			_ => "Invalid case".print(),
		}
	}
}

```

``` rust
impl traits::Printable for DispatchError {
	fn print(&self) {
		"DispatchError".print();
		match self {
			Self::Other(err) => err.print(),
			Self::CannotLookup => "Can not lookup".print(),
			Self::BadOrigin => "Bad origin".print(),
			Self::Module { index, error, message } => {
				index.print();
				error.print();
				if let Some(msg) = message {
					msg.print();
				}
			}
		}
	}
}
```

## If Std

The `print` function works well when you just want to print and you have the `Printable` trait implemented. In some cases you may want to do more than print, or not bother with Substrate-specific traits just for debugging purposes. The [`if_std!` macro](https://substrate.dev/rustdocs/master/sp_std/macro.if_std.html) is for exactly this situation.

One caveat of using this macro is that the code inside will only execute when you are actually running the native version of the runtime.

``` rust
use sp_std::if_std; // Import into scope the if_std! macro.
```

The `println!` statement should be inside of the `if_std` macro.
``` rust
decl_module! {

		// --snip--
		pub fn do_something(origin) -> DispatchResult {

			let who = ensure_signed(origin)?;
			let my_val: u32 = 777;

			Something::put(my_val);

			if_std! {
				// This code is only being compiled and executed when the `std` feature is enabled.
				println!("Hello native world!");
				println!("My value is: {:#?}", my_val);
				println!("The caller account is: {:#?}", who);
			}

			Self::deposit_event(RawEvent::SomethingStored(my_val, who));
			Ok(())
		}
		// --snip--
}
```

The values are printed in the terminal or the standard output every time that the
runtime function gets called.

```sh
$		2020-01-01 00:00:00 Substrate Node
		2020-01-01 00:00:00   version x.y.z-x86_64-linux-gnu
		2020-01-01 00:00:00   by Anonymous, 2017, 2020
		2020-01-01 00:00:00 Chain specification: Development
		2020-01-01 00:00:00 Node name: my-node-007
		2020-01-01 00:00:00 Roles: AUTHORITY
		2020-01-01 00:00:00 Imported 999 (0x3d7a…ab6e)
		# --snip--
->		Hello native world!
->		My value is: 777
->		The caller account is: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
		# --snip--
		2020-01-01 00:00:00 Imported 1000 (0x3d7a…ab6e)

```
