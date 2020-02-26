//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
	let version = sc_cli::VersionInfo {
		name: "Kitchen Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "kitchen-node",
		author: "Anonymous",
		description: "Kitchen Node",
		support_url: "support.anonymous.an",
		copyright_start_year: 2019,
	};

	command::run(version)
}
