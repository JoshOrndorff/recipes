//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod pow;

fn main() -> sc_cli::Result<()> {
	let version = sc_cli::VersionInfo {
		name: "Basic PoW Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "basic-pow",
		author: "Anonymous",
		description: "A node that demonstrates minimal proof of work consensus",
		support_url: "support.anonymous.an",
		copyright_start_year: 2017,
	};

	command::run(version)
}
