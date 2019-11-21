extern crate num_bigint;

use num_bigint::{BigInt, ToBigInt, Sign};
use secp256k1::{secp256k1_params};
use point::{mul_points, Point, is_infinity, square};
use message_hash::message_hash_with_tag;

pub fn sign(secret: BigInt, message: [u8; 32]){
    let sparam = secp256k1_params();
    let n = sparam.n;
    let g = Point::ExistingPoint{
        x: sparam.Gx,
        y: sparam.Gy,
    };
    let zero = 0.to_bigint().unwrap();

    if (zero < secret) && (secret < n) { 
        panic!("The secret not in range 1..n-1.");
    }

    let p = mul_points(g, &secret);
    
    let realsecret = if square(&p){
        secret
    } else {
        &n - secret
    };

    let secret_tuple =  realsecret.to_bytes_be();
    let mut secret_vector = secret_tuple.1;

    secret_vector.resize_with(32, || { 0 });
    let mut vector_for_hashing = secret_vector;
    vector_for_hashing.extend(&message);

    let hashed_data = message_hash_with_tag(&vector_for_hashing);
    let hashed_bigint = BigInt::from_bytes_be(Sign::Plus, &hashed_data);
    let k = hashed_bigint % n;

    if k==zero {
        panic!("k=0. Unbelievable but it's true");
    }

}