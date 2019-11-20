extern crate easy_hash;

use self::easy_hash::{Sha256, Hasher, HashResult};

static TAG:&[u8] = b"BIPSchnorr";

pub fn message_hash_with_tag(msg: &[u8]) -> [u8; 32]{
    let th = Sha256::hash(TAG);
    let mut d = th.to_vec();
    d.extend(&th);
    d.extend(msg);
    let res = Sha256::hash(&d);
    res
}