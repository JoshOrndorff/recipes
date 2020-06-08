//! Manaul / Instant Seal CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
// mod combined_service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
	command::run()
}
