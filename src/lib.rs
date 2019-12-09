extern crate num_bigint;

mod secp256k1;
mod point;
mod message_hash;
mod sign;
mod signature;
mod verify;
mod tool;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bigint_check(){
        let a = num_bigint::BigInt::parse_bytes(b"FFFF", 16).unwrap();
        let b = num_bigint::BigInt::parse_bytes(b"01", 16).unwrap();
        let c = a + b;
        assert_eq!(num_bigint::BigInt::parse_bytes(b"10000", 16).unwrap(), c);
    }

    #[test]
    fn bigint_check_max(){
        let a = num_bigint::BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16).unwrap();
        let b = num_bigint::BigInt::parse_bytes(b"01", 16).unwrap();
        let c = a + b;
        assert_eq!(num_bigint::BigInt::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000", 16).unwrap(), c);
    }

    #[test]
    fn bigint_check_non_neg(){
        use num_bigint::{ToBigInt};
        let a = num_bigint::BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16).unwrap();
        assert!(a > (0.to_bigint().unwrap()));
    }
}