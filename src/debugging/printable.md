# Printable

Substrate provides some types (`u8`, `u32`, `u64`, `usize`, `&[u8]`, `&str`) that implement the 
**Printable** trait by default.

The print function can be used as a way to log the status of the runtime execution.
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