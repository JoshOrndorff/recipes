use sc_cli::{RunCmd, Subcommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: SuperRunCmd,
}

#[derive(Debug, StructOpt)]
pub struct SuperRunCmd {
	#[structopt(flatten)]
	pub run: RunCmd,

	#[structopt(long)]
	pub manual_seal: bool,
}
