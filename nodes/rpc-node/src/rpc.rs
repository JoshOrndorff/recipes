#![warn(missing_docs)]

use std::sync::Arc;

use runtime::{opaque::Block, Hash};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use sp_block_builder::BlockBuilder;
pub use sc_rpc_api::DenyUnsafe;
use sp_transaction_pool::TransactionPool;
use sc_consensus_manual_seal::{
	manual_seal_rpc::{ManualSeal, ManualSealApi, CreateBlockCommand},
	manual_finality_rpc::{ManualFinality, ManualFinalityApi, FinalizeBlockCommand},
};
use futures::channel::mpsc::Sender;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// A command stream to send authoring commands to manual seal consensus engine
	pub authorship_sink: Sender<CreateBlockCommand<Hash>>,
	/// A command stream to send finalization commands to manual finality engine
	pub finality_sink: Sender<FinalizeBlockCommand<Hash>>
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: BlockBuilder<Block>,
	C::Api: sum_storage_runtime_api::SumStorageApi<Block>,
	P: TransactionPool + 'static,
	// C::Api: sum_storage_runtime_api::SumStorageApi<sp_runtime::generic::block::Block<sp_runtime::generic::header::Header<u32, sp_runtime::traits::BlakeTwo256>, sp_runtime::OpaqueExtrinsic>>,
{

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps { client, authorship_sink, finality_sink, .. } = deps;

	// Add a silly RPC that returns constant values
	io.extend_with(crate::silly_rpc::SillyRpc::to_delegate(
		crate::silly_rpc::Silly {},
	));

	// Add a second RPC extension
	// Because this one calls a Runtime API it needs a reference to the client.
	io.extend_with(sum_storage_rpc::SumStorageApi::to_delegate(
		sum_storage_rpc::SumStorage::new(client),
	));

	// This RPC extension receives commands for the manual seal consensus engine.
	io.extend_with(
		// We provide the rpc handler with the sending end of the channel to allow the rpc
		// send `CreateBlockCommand`s to the background block authorship task.
		ManualSealApi::to_delegate(ManualSeal::new(authorship_sink)),
	);

	// This RPC extension receives commands for the manual finality engine.
	io.extend_with(
		// We provide the rpc handler with the sending end of the channel to allow the rpc
		// send `FinalizeBlockCommands` to the background finality task.
		ManualFinalityApi::to_delegate(ManualFinality::new(finality_sink)),
	);

	io
}
