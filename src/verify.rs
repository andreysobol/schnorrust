use num_bigint::{BigInt, ToBigInt, Sign};

use signature::Signature;
use point::{Point, equal_points, mul_points, sum_points, square, is_infinity, get_x};
use secp256k1::{secp256k1_params};
use message_hash::message_hash_with_tag;

pub fn verify(public: BigInt, message: [u8; 32], signature: Signature) -> bool {

    let sparam = secp256k1_params();
    let p = sparam.p;
    let n = sparam.n;
    let g = Point::ExistingPoint{
        x: sparam.Gx,
        y: sparam.Gy,
    };

    let point = Point::from_x(public.clone());

    if equal_points(&point, &Point::Infinity) {
        return false;
    }

    if (signature.r >= p || signature.s >= n){
        return false;
    }

    let public_tuple = public.to_bytes_be();
    let mut public_vector = public_tuple.1;
    public_vector.resize_with(32, || { 0 });

    let r_tuple = signature.r.to_bytes_be();
    let mut r_vector = r_tuple.1;
    r_vector.resize_with(32, || { 0 });

    let mut hash_data = Vec::new();
    hash_data.extend(&r_vector);
    hash_data.extend(&public_vector);
    hash_data.extend(&message);

    let hashed = message_hash_with_tag(&hash_data);
    let hashed_bigint = BigInt::from_bytes_be(Sign::Plus, &hashed);
    let e = hashed_bigint % &n;

    let gs = mul_points(g, &signature.s);
    let pne = mul_points(point, &(n - e));
    let r = sum_points(&gs, &pne);

    if is_infinity(&r){
        return false;
    }

    if square(&r){
        return false;
    }

    get_x(&r) == &(signature.r)
}

#[test]
fn test_verify() {
    assert_eq!(1, 1);
}