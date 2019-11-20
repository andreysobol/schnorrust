extern crate num_bigint;

use num_bigint::{BigInt, ToBigInt};
use secp256k1::{secp256k1_params};
use point::{mul_points, Point, is_infinity};

pub fn sign(secret: BigInt, message: [u8; 32]){
    let sparam = secp256k1_params();
    let n = sparam.n;
    let g = Point::ExistingPoint{
        x: sparam.Gx,
        y: sparam.Gy,
    };

    if (((0.to_bigint().unwrap()) < secret) && (secret < n)){ 
        panic!("The secret not in range 1..n-1.");
    }

    let p = mul_points(g, secret);
    
    //realsecret = 
}