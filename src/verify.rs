use num_bigint::{BigInt, ToBigInt, Sign};

use signature::Signature;
use point::Point;

pub fn sign(public: BigInt, message: [u8; 32], signature: Signature) -> bool {
    let p = Point::from_x(public);
    true
}