use jsonrpc_derive::rpc;
use jsonrpc_core::Result;

#[rpc]
pub trait SillyRpc {
    #[rpc(name = "hello_five")]
    fn silly_5(&self) -> Result<u64>;

    #[rpc(name = "hello_seven")]
    fn silly_7(&self) -> Result<u64>;
}

/// A struct that implements the `SillyRpc`
pub struct Silly;

impl SillyRpc for Silly {
    fn silly_5(&self) -> Result<u64> {
        Ok(5)
    }

    fn silly_7(&self) -> Result<u64> {
        Ok(7)
    }
}
