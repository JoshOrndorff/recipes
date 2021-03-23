//! RPC Node Template CLI library.
#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;
mod silly_rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
