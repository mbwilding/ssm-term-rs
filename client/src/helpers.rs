use sha2::{Digest, Sha256};

pub fn get_sha256_hash(input: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}
