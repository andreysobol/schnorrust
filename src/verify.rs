use num_bigint::{BigInt, ToBigInt, Sign};

use signature::Signature;
use point::{Point, equal_points};
use secp256k1::{secp256k1_params};

pub fn sign(public: BigInt, message: [u8; 32], signature: Signature) -> bool {

    let sparam = secp256k1_params();
    let p = sparam.p;
    let n = sparam.n;

    let point = Point::from_x(public);

    if equal_points(&point, &Point::Infinity) {
        return false;
    }

    if (signature.r >= p || signature.s >= n){
        return false;
    }

    true
}