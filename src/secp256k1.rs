extern crate num_bigint;
use num_bigint::{BigInt};

pub struct Secp256k1Params{
    pub p: BigInt,
    pub n: BigInt,
    pub Gx: BigInt,
    pub Gy: BigInt,
}

pub fn secp256k1_params() -> Secp256k1Params{
    let p: BigInt = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
    let n: BigInt = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
    let gx: BigInt = BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let gy: BigInt = BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();

    Secp256k1Params {
        p: p,
        n: n,
        Gx: gx,
        Gy: gy,
    }
}