//# codec = { version = "*", package="parity-scale-codec", default-features = false, features = ['derive']  }
//# hex = "0.4.2"

use codec::{Encode, Decode};
use hex;

type Nonce = u8;

/// The Extrinsic type for this runtime. Currently extrinsics are unsigned.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub enum FramelessTransaction {
	Set(Nonce),
	Clear(Nonce),
	Toggle(Nonce),
}

fn main() {
    println!("{:}", hex::encode(FramelessTransaction::Set(20).encode()))
}