use num_bigint::{BigInt, ToBigInt};

pub fn unsigned_modulo(i: &BigInt, m: &BigInt) -> BigInt{
    let mut r = i % m;
    let zero = 0.to_bigint().unwrap();

    if(&r < &zero){
        r = m + r;
    }

    r
}