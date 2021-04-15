use std::sync::Arc;
use std::marker::PhantomData;
use sc_client_api::backend;
use sp_consensus::{SelectChain, Error as ConsensusError};
use sp_blockchain::{Backend, HeaderBackend};
use sp_runtime::{
	traits::{NumberFor, Block as BlockT},
	generic::BlockId,
};
//TODO what am I actually using?
use futures::prelude::*;


/// A select chain implementation that receives commands dictating when to change the best block over a channel.
pub struct ManualSelectChain<B, Block: BlockT, CS> {
	backend: Arc<B>,
	command_stream: CS, //TODO is it actually useful for CS to be generic? Will we ever use anything else?
	selected: Option<Block::Hash>,
	_phantom: PhantomData<Block>
}

// So it turns out that SelectChain has to impl Clone.
// That means I either need to use `watch` vs `mpsc`, or not have the channel belong
// to the struct. Which I guess would mean the start method would become a standalone function
// For now I'll continue assuming I'll use `watch`.
impl<B, Block: BlockT, CS: Clone> Clone for ManualSelectChain<B, Block, CS> {
	fn clone(&self) -> Self {
		Self {
			backend: self.backend.clone(),
			command_stream: self.command_stream.clone(),
			selected: self.selected.clone(),
			_phantom: Default::default(),
		}
	}
}

impl<B, Block, CS> ManualSelectChain<B, Block, CS>
	where
		B: backend::Backend<Block>,
		Block: BlockT,
		// TODO The manual seal code I copied had Unpin + 'static here. consider restoring if mysterious errors arise
		CS: Stream<Item=<Block as BlockT>::Hash>,
{
	/// Instantiate a new ManualSelectChain with the given backend and channel
	pub fn new(backend: Arc<B>, command_stream: CS) -> Self {
        // Or maybe make the channel here and return the sender.
		Self {
			backend,
			command_stream,
			selected: None,
			_phantom: Default::default()
		}
	}

	/// Run the worker task that listens to the RPC and updates the best block accordingly
	/// This is a little different than manual seal. There it is just a function. There is no
	/// struct for it to be a method on.
	//TODO Oh wow, shoudl the channel just be a parameter to this function rather than a field on the struct?
	// Probably.
	pub async fn start_worker(&self) {
		while let Some(newly_selected) = self.command_stream.next().await {
			// TODO Set the `selected` field to be whatever came in over the channel.
			// The tricky part is that `ManualSelectChain` has to be `Clone`. So how can
			// I ensure they all get updated?
		}
	}

	fn best_block_header(&self) -> sp_blockchain::Result<<Block as BlockT>::Header> {
		let info = self.backend.blockchain().info();
		let import_lock = self.backend.get_import_lock();
		let best_hash = self.backend
			.blockchain()
			.best_containing(info.best_hash, None, import_lock)?
			.unwrap_or(info.best_hash);

		Ok(self.backend.blockchain().header(BlockId::Hash(best_hash))?
			.expect("given block hash was fetched from block in db; qed"))
	}

	fn leaves(&self) -> Result<Vec<<Block as BlockT>::Hash>, sp_blockchain::Error> {
		self.backend.blockchain().leaves()
	}
}

impl<B, Block, CS> SelectChain<Block> for ManualSelectChain<B, Block, CS>
	where
		B: backend::Backend<Block>,
		Block: BlockT,
		CS: Send + Sync + Clone,
		CS: Stream<Item=<Block as BlockT>::Hash>,
{
 
	fn leaves(&self) -> Result<Vec<<Block as BlockT>::Hash>, ConsensusError> {
		Self::leaves(self)
			.map_err(|e| ConsensusError::ChainLookup(e.to_string()).into())
	}

	fn best_chain(&self) -> Result<<Block as BlockT>::Header, ConsensusError>
	{
		Self::best_block_header(&self)
			.map_err(|e| ConsensusError::ChainLookup(e.to_string()).into())
	}

	fn finality_target(
		&self,
		target_hash: Block::Hash,
		maybe_max_number: Option<NumberFor<Block>>
	) -> Result<Option<Block::Hash>, ConsensusError> {
		let import_lock = self.backend.get_import_lock();
		self.backend.blockchain().best_containing(target_hash, maybe_max_number, import_lock)
			.map_err(|e| ConsensusError::ChainLookup(e.to_string()).into())
	}
}

