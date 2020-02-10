use crate::service;
use futures::{
    future::{select, Map},
    FutureExt,
    TryFutureExt,
    channel::oneshot,
    compat::Future01CompatExt,
};
use futures01::Future;
use std::cell::RefCell;
use tokio::runtime::Runtime;
pub use sc_cli::{VersionInfo, IntoExit, error};
use sc_cli::{informant, parse_and_prepare, ParseAndPrepare, NoCustom};
use sc_service::{AbstractService, Roles as ServiceRoles, Configuration};
use crate::chain_spec;
use log::info;

/// Parse command line arguments into service configuration.
pub fn run<I, T, E>(args: I, exit: E, version: VersionInfo) -> error::Result<()> where
	I: IntoIterator<Item = T>,
	T: Into<std::ffi::OsString> + Clone,
	E: IntoExit,
{
	type Config<T> = Configuration<(), T>;
	match parse_and_prepare::<NoCustom, NoCustom, _>(&version, "substrate-node", args) {
		ParseAndPrepare::Run(cmd) => cmd.run(load_spec, exit,
		|exit, _cli_args, _custom_args, config: Config<_>| {
			info!("{}", version.name);
			info!("  version {}", config.full_version());
			info!("  by {}, 2017, 2018", version.author);
			info!("Chain specification: {}", config.chain_spec.name());
			info!("Node name: {}", config.name);
			info!("Roles: {:?}", config.roles);
			let runtime = Runtime::new().map_err(|e| format!("{:?}", e))?;
			match config.roles {
				ServiceRoles::LIGHT => run_until_exit(
					runtime,
				 	service::new_light(config).map_err(|e| format!("{:?}", e))?,
					exit
				),
				_ => run_until_exit(
					runtime,
					service::new_full(config).map_err(|e| format!("{:?}", e))?,
					exit
				),
			}.map_err(|e| format!("{:?}", e))
		}),
		ParseAndPrepare::BuildSpec(cmd) => cmd.run::<NoCustom, _, _, _>(load_spec),
		ParseAndPrepare::ExportBlocks(cmd) => cmd.run_with_builder(|config: Config<_>|
			Ok(new_full_start!(config).0), load_spec, exit),
        ParseAndPrepare::CheckBlock(cmd) => cmd.run_with_builder(|config: Config<_>|
            Ok(new_full_start!(config).0), load_spec, exit),
        ParseAndPrepare::ImportBlocks(cmd) => cmd.run_with_builder(|config: Config<_>|
			Ok(new_full_start!(config).0), load_spec, exit),
		ParseAndPrepare::PurgeChain(cmd) => cmd.run(load_spec),
		ParseAndPrepare::RevertChain(cmd) => cmd.run_with_builder(|config: Config<_>|
			Ok(new_full_start!(config).0), load_spec),
		ParseAndPrepare::CustomCommand(_) => Ok(())
	}?;

	Ok(())
}

fn load_spec(id: &str) -> Result<Option<chain_spec::ChainSpec>, String> {
	Ok(match chain_spec::Alternative::from(id) {
		Some(spec) => Some(spec.load()?),
		None => None,
	})
}

fn run_until_exit<T, E>(
	mut runtime: Runtime,
	service: T,
	e: E,
) -> error::Result<()>
where
	T: AbstractService,
	E: IntoExit,
{
	let (exit_send, exit) = exit_future::signal();

	let informant = informant::build(&service);
	let future = select(exit, informant)
		.map(|_| Ok(()))
		.compat();
	runtime.executor().spawn(future);

	// we eagerly drop the service so that the internal exit future is fired,
	// but we need to keep holding a reference to the global telemetry guard
	let _telemetry = service.telemetry();
    //
    // let fut01 = Futures01<
    //     Item = (),
    //     Error = ()
    // >;
    //
    // fut01.map(|x| ()).map_err(|x| x);
    //
    // use futures::TryFutureExt;
    // let fut03 = Future<
    //     Output = Result<(), ()>
    // >;
    // .map()// then()
    // .map_ok() // map()
    // .map_err();

	let service_res = {
		let exit = e.into_exit();
		let service = service
			.map_err(|err| error::Error::Service(err))
			.compat();
		let select = select(service, exit)
			.map(|_| Ok(()))
			.compat();
		runtime.block_on(select)
        //
		// let exit = e.into_exit().compat();
        //
        // //.map_err(|_| error::Error::Other("Exit future failed.".into()));
		// let service = service.map_err(|err| error::Error::Service(err));
		// let select = service.select(exit)
        //     .map(|_| ())
        //     .map_err(|(err, _)| err);
		// runtime.block_on(select)
	};

	if let Err(e) = exit_send.fire() {
        log::error!("Couldn't send the exit signal: {:?}", e);
    }

	// TODO [andre]: timeout this future #1318
	let _ = runtime.shutdown_on_idle().wait();

	service_res
}

// handles ctrl-c
pub struct Exit;
impl IntoExit for Exit {
	type Exit = Map<
        oneshot::Receiver<()>,
        fn(Result<(), oneshot::Canceled>) -> ()
    >;
	fn into_exit(self) -> Self::Exit {
		// can't use signal directly here because CtrlC takes only `Fn`.
		let (exit_send, exit) = oneshot::channel();

		let exit_send_cell = RefCell::new(Some(exit_send));
		ctrlc::set_handler(move || {
			let exit_send = exit_send_cell.try_borrow_mut().expect("signal handler not reentrant; qed").take();
			if let Some(exit_send) = exit_send {
				exit_send.send(()).expect("Error sending exit notification");
			}
		}).expect("Error setting Ctrl-C handler");

		exit.map(drop)
	}
}
