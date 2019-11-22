use num_bigint::{BigInt, ToBigInt, Sign};

pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

impl Signature {
    fn r_to_bytes(&self) -> Vec<u8>{
        self.r.to_bytes_be().1
    }

    fn s_to_bytes(&self) -> Vec<u8>{
        self.r.to_bytes_be().1
    }
}