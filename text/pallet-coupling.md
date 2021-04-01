# Tightly- and Loosely-Coupled Pallets

`pallets/check-membership`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fcheck-membership%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/check-membership/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

The `check-membership` crate contains two pallets that solve the same problems in slightly different
ways. Both pallets implement a single dispatchable function that can only be successfully executed
by callers who are members of an
[access control list](https://en.wikipedia.org/wiki/Access-control_list). The job of maintaining the
access control list is abstracted away to another pallet. This pallet and the membership-managing
pallet can be coupled in two different ways which are demonstrated by the tight and loose variants
of the pallet.

## Twin Pallets

Before we dive into the pallet code, let's talk a bit more about the structure of the crate in the
`pallets/check-membership` directory. This directory is a single Rust crate that contains two
pallets. The two pallets live in the `pallets/check-membership/tight` and
`pallets/check-membership/loose` directories. In the crate's main `lib.rs` we simply export each of
these variants of the pallet.

```rust, ignore
pub mod loose;
pub mod tight;
```

This allows us to demonstrate both techniques while keeping the closely related work in a single
crate.

## Controlling Access

While the primary learning objective of these twin pallets is understanding the way in which they
are coupled to the membership-managing pallets, they also demonstrate the concept of an access
control list, which we will investigate first.

It is often useful to designate some functions as permissioned and, therefore, accessible only to a
defined group of users. In this pallet, we check that the caller of the `check_membership` function
corresponds to a member of the permissioned set.

The loosely coupled variant looks like this.

```rust, ignore
/// Checks whether the caller is a member of the set of Account Ids provided by the
/// MembershipSource type. Emits an event if they are, and errors if not.
fn check_membership(origin) -> DispatchResult {
	let caller = ensure_signed(origin)?;

	// Get the members from the vec-set pallet
	let members = T::MembershipSource::accounts();

	// Check whether the caller is a member
	ensure!(members.contains(&caller), Error::<T>::NotAMember);

	// If the previous call didn't error, then the caller is a member, so emit the event
	Self::deposit_event(RawEvent::IsAMember(caller));
	Ok(())
}
```

## Coupling Pallets

Each `check-membership` pallet actually contains very little logic. It has no storage of its own and
a single extrinsic that does the membership checking. All of the heavy lifting is abstracted away to
another pallet. There are two different ways that pallets can be coupled to one another and this
section investigates both.

### Tight Coupling

Tightly coupling pallets is more explicit than loosely coupling them. When you are writing a pallet
that you want to tightly couple with some other pallet as a dependency, you explicitly specify the
name of the pallet on which you depend as a trait bound on the configuration trait of the pallet you
are writing. This is demonstrated in the tightly coupled variant of `check-membership`.

```rust, ignore
pub trait Config: frame_system::Config + vec_set::Trait {
	// --snip--
}
```

> This pallet, and all pallets, are tightly coupled to `frame_system`.

Supplying this trait bound means that the tightly coupled variant of `check-membership` pallet can
only be installed in a runtime that also has the [`vec-set` pallet](./vec-set.md)
installed. We also see the tight coupling in the pallet's `Cargo.toml` file, where `vec-set` is
listed by name.

```toml
vec-set = { path = '../vec-set', default-features = false }
```

To actually get the set of members, we have access to the getter function declared in `vec-set`.

```rust, ignore
// Get the members from the vec-set pallet
let members = vec_set::Module::<T>::members();
```

While tightly coupling pallets is conceptually simple, it has the disadvantage that it depends on a
specific implementation rather than an abstract interface. This makes the code more difficult to
maintain over time and is generally frowned upon. The tightly coupled version of `check-membership`
depends on exactly the `vec-set` pallet rather than a behavior such as managing a set of accounts.

## Loose Coupling

Loose coupling solves the problem of coupling to a specific implementation. When loosely coupling to
another pallet, you add an associated type to the pallet's configuration trait and ensure the
supplied type implements the necessary behavior by specifying a trait bound.

```rust, ignore
pub trait Config: frame_system::Config {
	// --snip--

	/// A type that will supply a set of members to check access control against
	type MembershipSource: AccountSet<AccountId = Self::AccountId>;
}
```

> Many pallets throughout the ecosystem are coupled to a token through the
> [`Currency` trait](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html).

Having this associated type means that the loosely coupled variant of the `check-membership` pallet
can be installed in any runtime that can supply it with a set of accounts to use as an access
control list. The code for the `AccountSet` trait lives in `traits/account-set/src/lib.rs` directory
and is quite short.

```rust, ignore
pub trait AccountSet {
	type AccountId;

	fn accounts() -> BTreeSet<Self::AccountId>;
}
```

We also see the loose coupling in the pallet's `Cargo.toml` file, where `account-set` is listed.

```toml
account-set = { path = '../../traits/account-set', default-features = false }
```

To actually get the set of members, we use the `accounts` method supplied by the trait.

```rust, ignore
// Get the members from the vec-set pallet
let members = T::MembershipSource::accounts();
```
