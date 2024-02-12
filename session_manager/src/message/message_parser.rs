// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"). You may not
// use this file except in compliance with the License. A copy of the
// License is located at
//
// http://aws.amazon.com/apache2.0/
//
// or in the "license" file accompanying this file. This file is distributed
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing
// permissions and limitations under the License.

use crate::message::client_message::message::{ClientMessage, ClientMessageError, MessageType};
use byteorder::{BigEndian, ByteOrder};
use std::mem::size_of;
use uuid::Uuid;

impl ClientMessage {
    pub fn deserialize_client_message(input: &[u8]) -> Result<Self, ClientMessageError> {
        let message_type = get_string(input, Self::MESSAGE_TYPE_OFFSET, Self::MESSAGE_TYPE_LENGTH)
            .and_then(|s| {
                s.parse::<MessageType>().map_err(|e| {
                    ClientMessageError::DeserializationError(format!("Parse error: {}", e))
                })
            })
            .map_err(|e| {
                log::error!("Error in deserializing and parsing message_type: {}", e);
                e
            })?;

        let schema_version = get_u32(input, Self::SCHEMA_VERSION_OFFSET).map_err(|e| {
            log::error!(
                "Could not deserialize field schema_version with error: {}",
                e
            );
            e
        })?;

        let created_date = get_u64(input, Self::CREATED_DATE_OFFSET).map_err(|e| {
            log::error!("Could not deserialize field created_date with error: {}", e);
            e
        })?;

        let sequence_number = get_i64(input, Self::SEQUENCE_NUMBER_OFFSET).map_err(|e| {
            log::error!(
                "Could not deserialize field sequence_number with error: {}",
                e
            );
            e
        })?;

        let flags = get_u64(input, Self::FLAGS_OFFSET).map_err(|e| {
            log::error!("Could not deserialize field flags with error: {}", e);
            e
        })?;

        let message_id = get_uuid(input, Self::MESSAGE_ID_OFFSET).map_err(|e| {
            log::error!("Could not deserialize field message_id with error: {}", e);
            e
        })?;

        let payload_digest = get_bytes(
            input,
            Self::PAYLOAD_DIGEST_OFFSET,
            Self::PAYLOAD_DIGEST_LENGTH,
        )
        .map_err(|e| {
            log::error!(
                "Could not deserialize field payload_digest with error: {}",
                e
            );
            e
        })?;

        let payload_type = get_u32(input, Self::PAYLOAD_TYPE_OFFSET)
            .map_err(|e| {
                log::error!("Could not deserialize field payload_type with error: {}", e);
                e
            })?
            .into();

        let payload_length = get_u32(input, Self::PAYLOAD_LENGTH_OFFSET).map_err(|e| {
            log::error!(
                "Could not deserialize field payload_length with error: {}",
                e
            );
            e
        })?;

        let header_length = get_u32(input, Self::HL_OFFSET).map_err(|e| {
            log::error!(
                "Could not deserialize field header_length with error: {}",
                e
            );
            e
        })?;

        let payload =
            String::from_utf8_lossy(&input[header_length as usize + Self::PAYLOAD_LENGTH_LENGTH..])
                .to_string();

        Ok(Self {
            header_length,
            message_type,
            schema_version,
            created_date,
            sequence_number,
            flags,
            message_id,
            payload_digest,
            payload_type,
            payload_length,
            payload,
        })
    }

    pub fn serialize_client_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header_length.to_be_bytes());
        bytes.extend_from_slice(&pad_trim_string(
            self.message_type.to_string().as_bytes(),
            32,
        ));
        bytes.extend_from_slice(&self.schema_version.to_be_bytes());
        bytes.extend_from_slice(&self.created_date.to_be_bytes());
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&put_uuid(&self.message_id));
        bytes.extend_from_slice(&self.payload_digest);
        let payload_type: u32 = self.payload_type.into();
        bytes.extend_from_slice(&payload_type.to_be_bytes());
        bytes.extend_from_slice(&self.payload_length.to_be_bytes());
        bytes.extend_from_slice(&self.payload.as_bytes());

        bytes
    }
}

// Check if the byte slice and offset are valid for type T.
fn check_valid<T: Sized>(byte_array: &[u8], offset: usize) -> Result<(), ClientMessageError> {
    if offset + size_of::<T>() > byte_array.len() {
        log::error!("check_valid failed: Offset is invalid.");
        return Err(ClientMessageError::DeserializationError(
            "Offset is outside the byte array.".to_string(),
        ));
    }

    Ok(())
}

/// Get a string value from the byte array starting from the specified offset to the defined length.
fn get_string(
    byte_array: &[u8],
    offset: usize,
    string_length: usize,
) -> Result<String, ClientMessageError> {
    let byte_array_length = byte_array.len();
    if offset >= byte_array_length || offset + string_length > byte_array_length {
        log::error!("get_string failed: Offset is invalid.");
        return Err(ClientMessageError::DeserializationError(
            "Offset is outside the byte array.".to_string(),
        ));
    }

    // Remove nulls from the bytes array
    let b = &byte_array[offset..offset + string_length];

    let string = match std::str::from_utf8(b) {
        Ok(s) => s,
        Err(e) => {
            log::error!("UTF-8 conversion error: {}", e);
            return Err(ClientMessageError::DeserializationError(
                "UTF-8 conversion failed.".to_string(),
            ));
        }
    }
    .trim();

    Ok(string.to_string())
}

/// Gets
fn get_bytes(
    byte_array: &[u8],
    offset: usize,
    byte_length: usize,
) -> Result<Vec<u8>, ClientMessageError> {
    let byte_array_length = byte_array.len();

    if offset >= byte_array_length || offset + byte_length > byte_array_length {
        log::error!("get_bytes failed: Offset is invalid.");
        return Err(ClientMessageError::DeserializationError(
            "Offset is outside the byte array.".to_string(),
        ));
    }

    Ok(byte_array[offset..offset + byte_length].to_vec())
}

/// Converts the big-endian byte slice to little-endian Uuid.
fn get_uuid(byte_array: &[u8], offset: usize) -> Result<Uuid, ClientMessageError> {
    let byte_array_length = byte_array.len();
    if offset >= byte_array_length || offset + 16 > byte_array_length {
        log::error!("get_uuid failed: Offset is invalid.");
        return Err(ClientMessageError::DeserializationError(
            "Offset is outside the byte array.".to_string(),
        ));
    }

    let mut uuid_bytes = [0u8; 16];

    uuid_bytes[0..4].copy_from_slice(&byte_array[offset + 8..offset + 12]);
    uuid_bytes[4..6].copy_from_slice(&byte_array[offset + 12..offset + 14]);
    uuid_bytes[6..8].copy_from_slice(&byte_array[offset + 14..offset + 16]);
    uuid_bytes[8..10].copy_from_slice(&byte_array[offset..offset + 2]);
    uuid_bytes[10..16].copy_from_slice(&byte_array[offset + 2..offset + 8]);

    Ok(Uuid::from_bytes(uuid_bytes))
}

/// Converts the little-endian Uuid to a big-endian Uuid.
fn put_uuid(uuid: &Uuid) -> [u8; 16] {
    let mut uuid_bytes = uuid.as_bytes().clone();

    uuid_bytes.swap(0, 3);
    uuid_bytes.swap(1, 2);
    uuid_bytes.swap(4, 5);
    uuid_bytes.swap(6, 7);

    uuid_bytes.into()
}

fn get_u32(byte_array: &[u8], offset: usize) -> Result<u32, ClientMessageError> {
    check_valid::<u32>(byte_array, offset)?;

    Ok(BigEndian::read_u32(&byte_array[offset..offset + 4]))
}

fn get_u64(byte_array: &[u8], offset: usize) -> Result<u64, ClientMessageError> {
    check_valid::<u64>(byte_array, offset)?;

    Ok(BigEndian::read_u64(&byte_array[offset..offset + 8]))
}

fn get_i64(byte_array: &[u8], offset: usize) -> Result<i64, ClientMessageError> {
    check_valid::<i64>(byte_array, offset)?;

    Ok(BigEndian::read_i64(&byte_array[offset..offset + 8]))
}

fn pad_trim_string(bytes: &[u8], desired: usize) -> Vec<u8> {
    if bytes.len() >= desired {
        bytes[..desired].to_vec()
    } else {
        let mut padded = Vec::from(bytes);
        padded.resize(desired, 0);
        padded
    }
}
