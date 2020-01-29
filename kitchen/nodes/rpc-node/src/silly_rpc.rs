use jsonrpc_derive::rpc;
use jsonrpc_core::Result;

#[rpc]
pub trait SillyRpc {
    #[rpc(name = "silly_seven")]
    fn silly_7(&self) -> Result<u64>;

    #[rpc(name = "silly_double")]
    fn silly_double(&self, val: u64) -> Result<u64>;
}

/// A struct that implements the `SillyRpc`
pub struct Silly;

impl SillyRpc for Silly {
    fn silly_7(&self) -> Result<u64> {
        Ok(7)
    }

    fn silly_double(&self, val: u64) -> Result<u64> {
        Ok(2 * val)
    }
}
