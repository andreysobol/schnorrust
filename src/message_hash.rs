extern crate easy_hash;

use self::easy_hash::{Sha256, Hasher, HashResult};

pub fn message_hash_with_tag(msg: &[u8]){
    let th = Sha256::hash("BIPSchnorr".as_bytes());
    let mut d = th.to_vec();
    d.extend(&th);
    d.extend(msg);
    let res = Sha256::hash(&d);
}