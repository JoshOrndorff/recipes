use sc_cli::{RunCmd};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<MinimalNodeSubcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}

// All the subcommands for our minimal node
#[derive(Debug, StructOpt)]
pub enum MinimalNodeSubcommand {
	/// All the default Substrate commands
	#[structopt(flatten)]
	Base(sc_cli::Subcommand),

	/// Our silly subcomand for greeting the user
	#[structopt(
		name = "say-hi",
		about = "Greet the user and exit. Do no blockchain stuff."
	)]
	SayHi(SayHi),
}

/// Simply greet the user by name and exit
#[derive(Debug, StructOpt)]
pub struct SayHi {
	#[structopt(short, long)]
	pub name: Option<String>,
}
