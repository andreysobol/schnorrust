use num_bigint::{BigInt, ToBigInt, Sign};

pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

impl Signature {
    pub fn r_to_bytes(&self) -> Vec<u8>{
        let mut rv = self.r.to_bytes_be().1;
        rv.resize_with(32, || { 0 });
        rv
    }

    pub fn s_to_bytes(&self) -> Vec<u8>{
        let mut sv = self.s.to_bytes_be().1;
        sv.resize_with(32, || { 0 });
        sv
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        let mut res = Vec::new();
        res.extend(self.r_to_bytes());
        res.extend(self.s_to_bytes());
        res
    }
}