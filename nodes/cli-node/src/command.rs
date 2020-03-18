// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use sc_cli::VersionInfo;
use crate::service;
use crate::chain_spec;
use crate::cli::{Cli, MinimalNodeSubcommand};

/// Parse and run command line arguments
pub fn run(version: VersionInfo) -> sc_cli::Result<()> {
	let opt = sc_cli::from_args::<Cli>(&version);

	let mut config = sc_service::Configuration::from_version(&version);

	match opt.subcommand {
		// User has invoked our custom subcommand
		Some(MinimalNodeSubcommand::SayHi(options)) => {
			let name = match options.name {
				Some(name) => name,
				None => "Substrate hacker".to_string(),
			};
			println!("Hi {}!", name);
			Ok(())
		}

		// User has invoked a standard Substrate subcommand
		Some(MinimalNodeSubcommand::Base(subcommand)) => {
			subcommand.init(&version)?;
			subcommand.update_config(&mut config, chain_spec::load_spec, &version)?;
			subcommand.run(
				config,
				|config: _| Ok(new_full_start!(config).0),
			)
		},

		// User has not specified a subcommand, we will fall back to `run` as default
		None => {
			opt.run.init(&version)?;
			opt.run.update_config(&mut config, chain_spec::load_spec, &version)?;
			opt.run.run(
				config,
				service::new_light,
				service::new_full,
				&version,
			)
		},
	}
}
