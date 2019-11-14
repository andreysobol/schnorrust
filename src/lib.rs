extern crate num_bigint;

mod secp256k1;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn biguint_check(){
        let a = num_bigint::BigUint::parse_bytes(b"FFFF", 16).unwrap();
        let b = num_bigint::BigUint::parse_bytes(b"01", 16).unwrap();
        let c = a + b;
        assert_eq!(num_bigint::BigUint::parse_bytes(b"10000", 16).unwrap(), c);
    }

    #[test]
    fn biguint_check_max(){
        let a = num_bigint::BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16).unwrap();
        let b = num_bigint::BigUint::parse_bytes(b"01", 16).unwrap();
        let c = a + b;
        assert_eq!(num_bigint::BigUint::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000", 16).unwrap(), c);
    }
}