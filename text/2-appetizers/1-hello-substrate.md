# Hello Substrate
*[`pallets/hello-substrate`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/hello-substrate)*

The first pallet we'll explore is a simple "hello world" example. This pallet will have one dispatchable call that prints a message to the node's output. Because this is our first pallet, we'll also explore the structure that every pallet has. This code lives in `pallets/hello-substrate/src/lib.rs`.

## No Std

The very first line of code tells the rust compiler that this crate should not use rust's standard library except when explicitly told to. This is useful because Substrate runtimes compile to Web Assembly where the standard library is not available.

```rust, ignore
#![cfg_attr(not(feature = "std"), no_std)]
```

## Imports

Next, you'll find imports that come from various parts of the Substrate framework. All pallets will import from a few common crates including [`frame-support`](https://substrate.dev/rustdocs/master/frame_support/index.html), and [`frame-system`](https://substrate.dev/rustdocs/master/frame_system/index.html).  Complex pallets will have many imports as we'll see later. The `hello-substrate` pallet uses these imports.

```rust, ignore
use frame_support::{ decl_module, dispatch::DispatchResult };
use frame_system::{ self as system, ensure_signed };
use sp_runtime::print;
```

## Tests

Next we see a reference to the tests module. This pallet has tests written in a separate file called `tests.rs`. We will not discuss the tests further at this point, but they are covered in the [Testing section](../3-entrees/testing/README.md) of the book.

## Configuration Trait

Next, each pallet has a configuration trait which is called `Trait`. The configuration trait can be used to access features from other pallets, or [constants](../3-entrees/constants.md) that effect the pallet's behavior. This pallet is simple enough that our configuration trait can remain empty, although it must still exist.

```rust, ignore
pub trait Trait: system::Trait {}
```

## Dispatchable Calls

A Dispatchable call is a function that a blockchain user can call as part of an Extrinsic. "Extrinsic" is Substrate jargon meaning a call from outside of the chain. Most of the time they are transactions, and for now it is fine to think of them as transactions. Dispatchable calls are defined in the [`decl_module!` macro](https://substrate.dev/rustdocs/master/frame_support/macro.decl_module.html).

```rust, ignore
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		pub fn say_hello(origin) -> DispatchResult {
			// --snip--
		}

		// More dispatchable calls could go here
	}
}
```

As you can see, our `hello-substrate` pallet has one dipatchable call that takes a single argument, called `origin` which we'll investigate shortly. Both calls return a [`DispatchResult`](https://substrate.dev/rustdocs/master/frame_support/dispatch/type.DispatchResult.html) which can be either `Ok(())` indicating that the call succeeded, or and `Err` which we'll investigate in the [appetizer about errors](./3-errors.md).

## Inisde a Dispatchable Call

Let's take a closer look at our dispatchable call.

```rust, ignore
pub fn say_hello(origin) -> DispatchResult {
	// Ensure that the caller is a regular keypair account
	let _ = ensure_signed(origin)?;

	// Print a message
	print("Hello World");

	// Indicate that this call succeeded
	Ok(())
}
```

This function essentially does three things. First is uses the [`ensure_signed!` macro](https://substrate.dev/rustdocs/master/frame_system/fn.ensure_signed.html) to ensure that the caller of the function was a regular user who owns a private key. This macro also returns who that caller was, but in this case we don't care who the caller was. In future recipes we'll explore origins other than signed.

Second, it actually prints the message. Notice that we aren't using rust's normal `println!` macro, but rather a special `print` function. The reason for this is explain in the next section.

Finally, the call returns `Ok(())` to indicate that the call has succeeded. At a glance it seems that there is no way for this call to fail, but this is not quite true. The `ensure_signed!` macro, used at the beginning, can return an error if the call was not from a signed origin. This is the first time we're seeing the important paradigm "**Verify first, write last**". In Substrate development, it is important that you always ensure preconditions are met and return errors at the beginning. After these checks have completed, then you may begin the functions computation.

## Printing from the Runtime

Printing to the terminal from a rust program is typically very simple using the `println!` macro. However, Substrate runtimes are compiled to Web Assembly as well as a regular native binary, and do not have access to rust's standard library. That means we cannot use the regular `println!`. I encourage you to modify the code to try using `println!` and confirm that it will not compile. Nonetheless, printing a message from the runtime is useful both for logging information, and also for debugging.

![Substrate Architecture Diagram](../img/substrate-architecture.png)

At the top of our pallet, we imported `sp_runtime`'s [`print` function](https://substrate.dev/rustdocs/master/sp_runtime/fn.print.html). This special function allows the runtime to pass a message for printing to the outer part of the node which is not built to Wasm. This function is only able to print items that implement the [`Printable` trait](https://substrate.dev/rustdocs/master/sp_runtime/traits/trait.Printable.html). Luckily all the primitive types already implement this trait, and you can implement the trait for your own datatypes too.

## Installing the Pallet in a Runtime

In order to actually use a pallet, it must be installed in a Substrate runtime. This particular pallet is installed in the `super-runtime` which you built as part of the kitchen node. To install a pallet in a runtime, you must do three things.

### Depend on the Pallet

First we must include the pallet in our runtime's `Cargo.toml` file. In the case of the super-runtime, this file is at `runtimes/super-runtime/Cargo.toml`.

```toml
[dependencies]
# --snip--
hello-substrate = { path = "../../pallets/hello-substrate", default-features = false }
```

Because the runtime is built to both native and Wasm, we must ensure that our pallet is built to the correct target as well. At the bottom of the `Cargo.toml` file, we see this.

```toml
[features]
default = ["std"]
std = [
	# --snip--
	"hello-substrate/std",
]
```

### Implement its Configuration Trait

Next we must implement the pallet's configuration trait. This happens in the runtime's main `lib.rs` file. In the case of the super-runtime, this file is at `runtimes/super-runtime/src/lib.rs`. Because this pallet's configuration trait is trivial, so is implementing it.

```rust ignore
impl hello_substrate::Trait for Runtime {}
```
You can see the corresponding trait implementations in the surrounding lines. Most of them are more complex.

### Add it to `construct_runtime!`

Finally, we add our pallet to the [`construct_runtime!` macro](https://substrate.dev/rustdocs/master/frame_support/macro.construct_runtime.html).

```rust, ignore
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// --snip--
		HelloSubstrate: hello_substrate::{Module, Call},
	}
);
```

This macro does the heavy lifting of composing each individual pallet into a single usable runtime. Let's explain the syntax for each line. Each Pallet listed in the macro needs several pieces of information.

First is a convenient name to give to this pallet. We've chosen `HelloSubstrate`. It is common to choose the same name as the pallet itself except when there is [more than one instance](../3-entrees/instantiable.md). Next is the name of the crate that the pallet lives in. And finally there is a list of features the pallet provides. All pallet require `Module`. Our pallet also provides dispatchable calls, so it requires `Call`.

## Try it Out

If you haven't already, try interacting with the pallet using the Apps UI. You should see your message printed to the log of your node. You're now well on your way to becoming a blockchain chef. Let's continue to build our skills with another appetizer.
