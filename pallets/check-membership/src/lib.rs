#![cfg_attr(not(feature = "std"), no_std)]

//! This module contains two nearly identical Substrate pallets. Both demonstrate access control
//! and coupling multiple pallets together in a FRAME runtime.
//!
//! The _tight_ variant demonstrates tightly coupling pallets and is itself tightly coupled to the
//! `vec-set` pallet.
//!
//! The _loose_ variant demonstrates loosely coupling pallets and is itself loosely coupled through
//! the `AccountSet` trait.

pub mod loose;
pub mod tight;
