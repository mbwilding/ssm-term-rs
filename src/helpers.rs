use sha2::{Digest, Sha256};
use std::convert::TryInto;
use uuid::Uuid;

pub fn pad_trim(bytes: &[u8], desired: usize) -> Vec<u8> {
    if bytes.len() >= desired {
        bytes[..desired].to_vec()
    } else {
        let mut padded = Vec::from(bytes);
        padded.resize(desired, 0);
        padded
    }
}

pub fn get_sha256_hash(input: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

pub fn pattern_at(source: &[u8], pattern: &[u8]) -> Vec<usize> {
    source
        .windows(pattern.len())
        .enumerate()
        .filter_map(|(i, window)| if window == pattern { Some(i) } else { None })
        .collect()
}
