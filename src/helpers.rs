use aws_sdk_ssm::client::customize;
use sha2::{Digest, Sha256};
use std::str::FromStr;
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

pub fn big_endian_uuid(buffer: &[u8]) -> Uuid {
    let uuid = format!(
        "{0}-{1}-{2}-{3}-{4}",
        bytes_to_hex(&buffer[8..12]),
        bytes_to_hex(&buffer[12..14]),
        bytes_to_hex(&buffer[14..16]),
        bytes_to_hex(&buffer[0..2]),
        bytes_to_hex(&buffer[2..8])
    );

    let custom_uuid = Uuid::from_str(&uuid).unwrap();
    let be_uuid = Uuid::from_slice(buffer).unwrap();
    let le_uuid = Uuid::from_slice_le(buffer).unwrap();

    println!("Custom UUID: {:?}", custom_uuid);
    println!("Big Endian UUID: {:?}", be_uuid);
    println!("Little Endian UUID: {:?}", le_uuid);

    custom_uuid
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::new(), |acc, &byte| format!("{}{:02x}", acc, byte))
}

pub fn pattern_at(source: &[u8], pattern: &[u8]) -> Vec<usize> {
    source
        .windows(pattern.len())
        .enumerate()
        .filter_map(|(i, window)| if window == pattern { Some(i) } else { None })
        .collect()
}
