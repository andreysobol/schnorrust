use num_bigint::{BigInt, ToBigInt, Sign};

use signature::Signature;

pub fn sign(public: BigInt, message: [u8; 32], signature: Signature) -> bool {
    true
}