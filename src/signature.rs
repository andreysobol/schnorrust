use num_bigint::{BigInt, ToBigInt, Sign};

pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}