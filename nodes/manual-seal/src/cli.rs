use sc_cli::{RunCmd, Subcommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,

	/// Custom flag to enable instant-seal mode
	#[structopt(long)]
	pub instant_seal: bool,
}
