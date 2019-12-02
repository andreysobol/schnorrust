use num_bigint::{BigInt, BigUint, Sign};

fn parse_bigint_from_u_bytes(bytes: &[u8]) -> BigInt {
    let bui = BigUint::parse_bytes(bytes, 16).unwrap();
    let i = BigInt::from_biguint(Sign::Plus, bui);
    i
}