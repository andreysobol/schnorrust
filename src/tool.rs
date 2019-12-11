use num_bigint::{BigInt, ToBigInt};

pub fn unsigned_modulo(i: &BigInt, m: &BigInt) -> BigInt{
    let mut r = i % m;
    let zero = 0.to_bigint().unwrap();

    if(&r < &zero){
        r = m + r;
    }

    r
}


#[test]
fn positive_unsigned_modulo(){
    let i = 4.to_bigint().unwrap();
    let m = 3.to_bigint().unwrap();

    let r = unsigned_modulo(&i, &m);
    assert_eq!(r, 1.to_bigint().unwrap());
}

#[test]
fn negative_unsigned_modulo(){
    let i = (-7).to_bigint().unwrap();
    let m = 3.to_bigint().unwrap();

    let r = unsigned_modulo(&i, &m);
    assert_eq!(r, 2.to_bigint().unwrap());
}