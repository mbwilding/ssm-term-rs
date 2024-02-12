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

use crate::encryption::kms_service::kms_generate_data_key;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};
use anyhow::{bail, Result};
use aws_sdk_kms::Client as KmsClient;
use rand::{rngs::OsRng, RngCore};

const NONCE_SIZE: usize = 12;

pub struct Encrypter {
    kms_client: KmsClient,
    kms_key_id: String,
    cipher_text_key: Vec<u8>,
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
}

impl Encrypter {
    pub async fn new(
        kms_client: KmsClient,
        kms_key_id: String,
        context: (&str, &str),
    ) -> Result<Self> {
        let keys = Self::generate_encryption_key(&kms_client, &kms_key_id, context).await?;

        Ok(Self {
            kms_client,
            kms_key_id,
            cipher_text_key: keys.cypher_text_key,
            encryption_key: keys.encryption_key,
            decryption_key: keys.decryption_key,
        })
    }

    /// Calls KMS to generate a new encryption key.
    async fn generate_encryption_key(
        kms_client: &KmsClient,
        kms_key_id: &str,
        context: (&str, &str),
    ) -> Result<Keys> {
        let blobs = kms_generate_data_key(kms_client, kms_key_id, (context.0, context.1)).await?;

        let key_size = blobs.plain_text.as_ref().len() / 2;

        Ok(Keys {
            encryption_key: blobs.plain_text.as_ref()[..key_size].to_vec(),
            decryption_key: blobs.plain_text.as_ref()[key_size..].to_vec(),
            cypher_text_key: blobs.cipher_text.as_ref().to_vec(),
        })
    }

    /// Gets AEAD which is a GCM cipher mode providing authenticated encryption with associated data.
    pub fn get_aead(plain_text_key: &[u8]) -> Aes256Gcm {
        let key = GenericArray::from_slice(plain_text_key);
        Aes256Gcm::new(key)
    }

    /// GetEncryptedDataKey returns the cipher_text that was pulled from KMS.
    pub fn get_encrypted_data_key(&self) -> &[u8] {
        &self.cipher_text_key
    }

    /// Gets the KMS key id that is used to generate the encryption key.
    pub fn get_kms_key_id(&self) -> &str {
        &self.kms_key_id
    }

    /// Encrypts a byte slice and returns the encrypted slice.
    fn encrypt(&self, plain_text: &[u8]) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);

        let mut nonce = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce);
        let nonce = GenericArray::from_slice(&nonce);

        // Encrypt plain_text using given key and newly generated nonce
        match cipher.encrypt(nonce, plain_text) {
            Ok(mut cipher_text) => {
                // Append nonce to the beginning of the cipher_text to be used while decrypting
                let mut result = nonce.to_vec();
                result.append(&mut cipher_text);
                Ok(result)
            }
            Err(e) => bail!("Unable to encrypt: {}", e),
        }
    }

    /// Decrypts a byte slice and returns the decrypted slice.
    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(&self.decryption_key);
        let cipher = Aes256Gcm::new(key);

        // Pull the nonce out of the cipher_text
        let nonce = &cipher_text[..NONCE_SIZE];
        let cipher_text_without_nonce = &cipher_text[NONCE_SIZE..];
        let nonce = GenericArray::from_slice(nonce);

        // Decrypt just the actual cipher_text using nonce extracted above
        match cipher.decrypt(nonce, cipher_text_without_nonce) {
            Ok(decrypted_data) => Ok(decrypted_data),
            Err(e) => bail!("Unable to decrypt: {}", e),
        }
    }
}

/// Key container.
struct Keys {
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
    cypher_text_key: Vec<u8>,
}
