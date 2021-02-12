//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::{BlakeTwo256, Block as BlockT}};
use std::sync::Arc;
use std::collections::BTreeMap;
use sum_storage_runtime_api::SumStorageApi as SumStorageRuntimeApi;
use sc_client_api::backend::{StorageProvider, Backend, StateBackend};
use sp_io::hashing::twox_128;
use sp_storage::StorageKey;
use codec::Decode;
use sum_storage::SumStorageSchema;

pub trait SumStorageApiHelper<Block: BlockT> {
	fn get_sum(&self, at: BlockId<Block>) -> Result<u32>;
}

#[rpc]
pub trait SumStorageApi<BlockHash> {
	#[rpc(name = "sumStorage_getSum")]
	fn get_sum(&self, at: Option<BlockHash>) -> Result<u32>;
}

// TODO Can I use a blanket imlementation like this? Seems like a mostly nice idea,
// But I'm having some weird side effects. It seems like it is implementing the SumStorageApi for
// stuff where I didn't explicitly implement the SumStorageApiHelper.
// impl <BlockHash, T> SumStorageApi<BlockHash> for T
// 	where T: SumStorageApiHelper<BlockHash> + Send + Sync + 'static
// {
// 	fn get_sum(&self, at: Option<BlockHash>) -> Result<u32> {
// 		self.get_sum(at)
// 	}
// }

/// A struct that implements the `SumStorageApi`.
pub struct SumStorage<C, Block> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, Block> SumStorage<C, Block> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

impl<C, Block> SumStorageApiHelper<Block> for SumStorage<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: SumStorageRuntimeApi<Block>,
{
	fn get_sum(&self, at: BlockId<Block>) -> Result<u32> {
		let api = self.client.runtime_api();

		let runtime_api_result = api.get_sum(&at);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}




// A helper function to construct the raw storage keys
fn storage_prefix_build(module: &[u8], storage: &[u8]) -> Vec<u8> {
	[twox_128(module), twox_128(storage)].concat().to_vec()
}

/// A struct that implements the `SumStorageApi` by using hardcoded storage keys and the
/// state backend.
pub struct SumStorageOptimizedV1<C, BE, Block> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<(BE, Block)>,
}

impl<C, BE, Block> SumStorageOptimizedV1<C, BE, Block> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

impl<C, BE, Block> SumStorageApiHelper<Block> for SumStorageOptimizedV1<C, BE, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: StorageProvider<Block, BE>,
	C: HeaderBackend<Block>,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	fn get_sum(&self, at: BlockId<Block>) -> Result<u32> {

		// Get Thing1 from storage
		let thing1_encoded = self.client.storage(&at, &StorageKey(storage_prefix_build(b"SumStorage", b"Thing1")))
			.map_err(|e| RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Querying state backend for thing1 failed".into(),
				data: Some(format!("{:?}", e).into()),
			})?
			.ok_or(RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "No value stored for thing1".into(),
				data: None,
			})?
			.0;
		let thing1 : u32 = Decode::decode(&mut &thing1_encoded[..])
		.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Value stored for Thing1 could not decode to u32".into(),
			data: Some(format!("{:?}", e).into()),
		})?;

		// Get Thing2 from storage
		let thing2_encoded = self.client.storage(&at, &StorageKey(storage_prefix_build(b"SumStorage", b"Thing2")))
			.map_err(|e| RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Querying state backend for thing2 failed".into(),
				data: Some(format!("{:?}", e).into()),
			})?
			.ok_or(RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "No value stored for thing2".into(),
				data: None,
			})?
			.0;
		let thing2 : u32 = Decode::decode(&mut &thing2_encoded[..])
		.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Value stored for Thing2 could not decode to u32".into(),
			data: Some(format!("{:?}", e).into()),
		})?;

		// Return sum
		Ok(thing1 + thing2)
	}
}







/// A struct that implements the `SumStorageApi` by using hardcoded storage keys and the
/// state backend.
pub struct SumStorageOptimizedWithFallback<C, BE, Block: BlockT> {
	client: Arc<C>,
	optimized: BTreeMap<SumStorageSchema, Box<dyn SumStorageApiHelper<Block> + Send + Sync>>,
	fallback: SumStorage<C, Block>,
	_marker: std::marker::PhantomData<BE>,
}

impl<C, BE, Block> SumStorageOptimizedWithFallback<C, BE, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static,
		C: StorageProvider<Block, BE>,
		C: HeaderBackend<Block>,
		C: ProvideRuntimeApi<Block>,
		C::Api: SumStorageRuntimeApi<Block>,
		BE: Backend<Block> + 'static,
		BE::State: StateBackend<BlakeTwo256>,
{
	pub fn new(client: Arc<C>) -> Self {
		// This long-ass type annotation is, aparently, necessary to make it compile
		let mut optimized: BTreeMap<_, Box<dyn SumStorageApiHelper<_> + Send + Sync>> = BTreeMap::new();
		optimized.insert(
			SumStorageSchema::V1,
			Box::new(SumStorageOptimizedV1::new(client.clone()))
		);
		Self {
			client: client.clone(),
			optimized,
			fallback: SumStorage::new(client),
			_marker: Default::default(),
		}
	}
}

impl<C, BE, Block> SumStorageApi<<Block as BlockT>::Hash> for SumStorageOptimizedWithFallback<C, BE, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: StorageProvider<Block, BE>,
	C: HeaderBackend<Block>,
	C: ProvideRuntimeApi<Block>,
	C::Api: SumStorageRuntimeApi<Block>,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	fn get_sum(&self, maybe_at: Option<<Block as BlockT>::Hash>) -> Result<u32> {
		// If the block hash is not supplied assume the best block.
		let at = BlockId::hash(maybe_at.unwrap_or_else(||
			self.client.info().best_hash));

		// Grab the on-chain schema version
		let schema: SumStorageSchema = match self.client.storage(
			&at,
			&StorageKey(storage_prefix_build(b"SumStorage", b"StorageSchema"))
		) {
			Ok(Some(bytes)) => Decode::decode(&mut &bytes.0[..]).ok().unwrap_or(SumStorageSchema::Undefined),
			_ => SumStorageSchema::Undefined,
		};

		// If there is an optimized handler associated with this schema, then use it.
		// Otherwise fallback to the runtime api.
		if let Some(handler) = self.optimized.get(&schema) {
			handler.get_sum(at)
		}
		else {
			SumStorageApiHelper::get_sum(&self.fallback, at)
		}
	}
}
