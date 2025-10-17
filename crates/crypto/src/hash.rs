use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

pub fn hash160(input: &[u8]) -> [u8; 20] {
    let sha = sha256(input);
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(sha);
    ripemd160.finalize().into()
}
