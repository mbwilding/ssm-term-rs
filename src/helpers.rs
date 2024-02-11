use sha2::{Digest, Sha256};
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

pub fn parse_uuid(buffer: &[u8]) -> Uuid {
    let mut uuid_bytes = [0u8; 16];

    uuid_bytes[0..4].copy_from_slice(&buffer[8..12]);
    uuid_bytes[4..6].copy_from_slice(&buffer[12..14]);
    uuid_bytes[6..8].copy_from_slice(&buffer[14..16]);
    uuid_bytes[8..10].copy_from_slice(&buffer[0..2]);
    uuid_bytes[10..16].copy_from_slice(&buffer[2..8]);

    Uuid::from_bytes(uuid_bytes)
}

#[allow(dead_code)]
pub fn pattern_at(source: &[u8], pattern: &[u8]) -> Vec<usize> {
    source
        .windows(pattern.len())
        .enumerate()
        .filter_map(|(i, window)| if window == pattern { Some(i) } else { None })
        .collect()
}
