# Instantiable Pallets

`pallets/last-caller`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Flast-caller%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/last-caller/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

`pallets/default-instance`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fdefault-instance%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/default-instance/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

Instantiable pallets enable multiple instances of the same pallet logic within a single runtime.
Each instance of the pallet has its own independent storage, and extrinsics must specify which
instance of the pallet they are intended for. These patterns are illustrated in the kitchen in the
last-caller and default-instance pallets.

Some use cases:

-   Token chain hosts two independent cryptocurrencies.
-   Marketplace track users' reputations as buyers separately from their reputations as sellers.
-   Governance has two (or more) houses which act similarly internally.

Substrate's own Balances and Collective pallets are good examples of real-world code using this
technique. The default Substrate node has two instances of the Collectives pallet that make up its
Council and Technical Committee. Each collective has its own storage, events, and configuration.

```rust, ignore
Council: collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
TechnicalCommittee: collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>}
```

## Writing an Instantiable Pallet

Writing an instantiable pallet is almost entirely the same process as writing a plain
non-instantiable pallet. There are just a few places where the syntax differs.

> You must call `decl_storage!`
>
> Instantiable pallets _must_ call the `decl_storage!` macro so that the `Instance` type is created.

### Configuration Trait

```rust, ignore
pub trait Config<I: Instance>: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self, I>> + Into<<Self as frame_system::Config>::Event>;
}
```

### Storage Declaration

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config<I>, I: Instance> as TemplatePallet {
		...
	}
}
```

### Declaring the `Module` Struct

```rust, ignore
decl_module! {
	/// The module declaration.
	pub struct Module<T: Config<I>, I: Instance> for enum Call where origin: T::Origin {
		...
	}
}
```

### Accessing Storage

```rust, ignore
<Something<T, I>>::put(something);
```

If the storage item does not use any types specified in the configuration trait, the T is omitted,
as always.

```rust, ignore
<Something<I>>::put(something);
```

### Event initialization

```rust, ignore
fn deposit_event() = default;
```

### Event Declaration

```rust, ignore
decl_event!(
	pub enum Event<T, I> where AccountId = <T as frame_system::Config>::AccountId {
		...
	}
}
```

## Installing a Pallet Instance in a Runtime

The syntax for including an instance of an instantiable pallet in a runtime is slightly different
than for a regular pallet. The only exception is for pallets that use the
[Default Instance](#default-instance) feature described below.

### Implementing Configuration Traits

Each instance needs to be configured separately. Configuration consists of implementing the specific
instance's trait. The following snippet shows a configuration for `Instance1`.

```rust, ignore
impl template::Config<template::Instance1> for Runtime {
	type Event = Event;
}
```

### Using the `construct_runtime!` Macro

The final step of installing the pallet instance in your runtime is updating the
`construct_runtime!` macro. You may give each instance a meaningful name. Here I've called
`Instance1` `FirstTemplate`.

```rust, ignore
FirstTemplate: template::<Instance1>::{Module, Call, Storage, Event<T>, Config},
```

## Default Instance <a name="default-instance"></a>

One drawback of instantiable pallets, as we've presented them so far, is that they require the
runtime designer to use the more elaborate syntax even if they only desire a single instance of the
pallet. To alleviate this inconvenience, Substrate provides a feature known as DefaultInstance. This
allows runtime developers to deploy an instantiable pallet exactly as they would if it were not
instantiable provided they **only use a single instance**.

To make your instantiable pallet support DefaultInstance, you must specify it in four places.

```rust, ignore
pub trait Config<I=DefaultInstance>: frame_system::Config {
```

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config<I>, I: Instance=DefaultInstance> as TemplateModule {
		...
	}
}
```

```rust, ignore
decl_module! {
	pub struct Module<T: Config<I>, I: Instance = DefaultInstance> for enum Call where origin: T::Origin {
		...
	}
}
```

```rust, ignore
decl_event!(
	pub enum Event<T, I=DefaultInstance> where ... {
		...
	}
}
```

Having made these changes, a developer who uses your pallet doesn't need to know or care that your
pallet is instantable. They can deploy it just as they would any other pallet.

## Genesis Configuration

Some pallets require a genesis configuration to be specified. Let's look to the default Substrate
node's use of the Collective pallet as an example.

In its `chain_spec.rs` file we see

```rust, ignore
GenesisConfig {
	...
	collective_Instance1: Some(CouncilConfig {
		members: vec![],
		phantom: Default::default(),
	}),
	collective_Instance2: Some(TechnicalCommitteeConfig {
		members: vec![],
		phantom: Default::default(),
	}),
	...
}
```
