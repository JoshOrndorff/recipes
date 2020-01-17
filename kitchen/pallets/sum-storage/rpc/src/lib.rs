// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

//! RPC interface for the transaction payment module.

use std::sync::Arc;
use sp_blockchain::HeaderBackend;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, ProvideRuntimeApi, UniqueSaturatedInto},
};
use sp_core::Bytes;
use sum_storage_rpc_runtime_api::SumStorageApi as SumStorageRuntimeApi;
//pub use pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi as TransactionPaymentRuntimeApi;
// pub use self::gen_client::Client as TransactionPaymentClient;

#[rpc]
pub trait SumStorageApi<BlockHash> {
	#[rpc(name = "sumStorage_getSum")]
	fn get_sum(
		&self,
		at: Option<BlockHash>
	) -> Result<u32>;
}

/// A struct that implements the `SumStorageApi`.
pub struct SumStorage<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> SumStorage<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block> SumStorageApi<<Block as BlockT>::Hash>
	for SumStorage<C, Block> // TODO: if you have more generics, no need to add <M, N, P, ..>, just do SumStorage<C, (tuple_of_all_useless_phantom_generics)>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi,
	C: HeaderBackend<Block>,
	// NOTE: this is always generic over block, even if you don't define it. Macro crap.
	// if you add more generics, then it becomes <Block, Foo, Bar, Baz>
	C::Api: SumStorageRuntimeApi<Block>,
{
	fn get_sum(
		&self,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<u32> {
		// TODO: use the api call... btw, do you see that `query_info` has two args but we
		// have given it also a BlockNumber? related to the same hardcoded `Block` generic. All
		// runtime calls are querying the runtime code and storage at a particular block number. I
		// am not sure wtf happens if your node has pruned some old state and you query it.
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

		// If I return a value straight from here, then this is just another
		// Silly RPC like we used previously
		// Ok(1337)

		// Instead we'll call into a runtime API
		// Example for transaction payment.
		// api.query_info(&at, uxt, encoded_len).map_err(|e| RpcError {
		// 	code: ErrorCode::ServerError(Error::RuntimeError.into()),
		// 	message: "Unable to query dispatch info.".into(),
		// 	data: Some(format!("{:?}", e).into()),
		// }).map(CappedDispatchInfo::new)

		// Our actual call to sum-storage-runtime-api
		let runtime_api_result = api.get_sum(&at);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
