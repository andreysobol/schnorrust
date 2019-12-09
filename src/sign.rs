extern crate num_bigint;

use num_bigint::{BigInt, ToBigInt, Sign};
use secp256k1::{secp256k1_params};
use point::{mul_points, Point, is_infinity, square, get_x};
use message_hash::message_hash_with_tag;
use signature::Signature;

pub fn sign(secret: BigInt, message: [u8; 32]) -> Signature {
    let sparam = secp256k1_params();
    let n = sparam.n;
    let g = Point::ExistingPoint{
        x: sparam.Gx,
        y: sparam.Gy,
    };
    let zero = 0.to_bigint().unwrap();

    if !((zero < secret) && (secret < n)) {
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
    let k = hashed_bigint % &n;

    if k==zero {
        panic!("k=0. Unbelievable but it's true");
    }

    let anothersparam = secp256k1_params();
    let anotherg = Point::ExistingPoint{
        x: anothersparam.Gx,
        y: anothersparam.Gy,
    };
    let r = mul_points(anotherg, &k);

    let realk = if square(&r){
        k
    } else {
        &n - k
    };

    let rx = get_x(&r);
    let rx_tuple = rx.to_bytes_be();
    let mut rx_vector = rx_tuple.1;
    rx_vector.resize_with(32, || { 0 });

    let px = get_x(&p);
    let px_tuple = px.to_bytes_be();
    let mut px_vector = px_tuple.1;
    px_vector.resize_with(32, || { 0 });

    let mut second_hash_data = Vec::new();
    second_hash_data.extend(&rx_vector);
    second_hash_data.extend(&px_vector);
    second_hash_data.extend(&message);

    let second_hashed_data = message_hash_with_tag(&second_hash_data);
    let second_hashed_bigint = BigInt::from_bytes_be(Sign::Plus, &second_hashed_data);
    let e = second_hashed_bigint % &n;

    let s = (realk + e * realsecret) % &n;

    Signature{
        r: rx.clone(),
        s: s,
    }
}

#[test]
fn test_sign_vector1() {
    assert_eq!(1, 1);

    let secret_key = BigInt::parse_bytes(b"B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF", 16).unwrap();
    let message_bigint = BigInt::parse_bytes(b"DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659", 16).unwrap();
    let mut message_vec = message_bigint.to_bytes_be().1;
    message_vec.resize_with(32, || { 0 });
    let mut message = [0; 32];
    message.copy_from_slice(&message_vec); 

    let signature = sign(secret_key, message);
    let s = signature.to_bytes();
    //println!("{:x?}", s);
    //assert_eq!(1, 1);
}
//index,secret key,public key,message,signature,verification result,comment
//1,B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF,DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659,243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89,667C2F778E0616E611BD0C14B8A600C5884551701A949EF0EBFD72D452D64E844160BCFC3F466ECB8FACD19ADE57D8699D74E7207D78C6AEDC3799B52A8E0598,TRUE,
