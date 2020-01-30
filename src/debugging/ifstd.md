# Printing In Native Builds

Substrate runtimes are compiled to native binaries as well as WebAssembly blobs. Rust's standard library (`#![std]`) attribute indicates that the crate will link to the std-crate instead of the core-crate making some assumptions about the system the program will run on.

## If Std

Substrate's runtime standard library compiled and linked with Rust's standard library (`std`), enables an alternative way of printing values in the terminal.

``` rust
use sp_std::if_std; // Import the if-std macro.
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

The values are printed in the terminal or the standard output every time that the runtime function gets called.

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