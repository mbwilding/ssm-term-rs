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

use anyhow::{Context, Result};
use aws_sdk_kms::primitives::Blob;
use aws_sdk_kms::Client as KmsClient;

const KMS_KEY_SIZE_IN_BYTES: i32 = 64;

async fn new_kms_service() -> Result<KmsClient> {
    let config = aws_config::load_from_env().await;
    Ok(aws_sdk_kms::Client::new(&config))
}

async fn kms_decrypt(
    kms_client: &KmsClient,
    ciphertext_blob: Blob,
    context: (&str, &str),
) -> Result<Blob> {
    let decrypt = kms_client
        .decrypt()
        .ciphertext_blob(ciphertext_blob)
        .encryption_context(context.0, context.1)
        .send()
        .await?;

    let plain_text = decrypt.plaintext.context("KMS Plain text is empty.")?;

    Ok(plain_text)
}

pub async fn kms_generate_data_key(
    kms_client: &KmsClient,
    kms_key_id: &str,
    context: (&str, &str),
) -> Result<Blobs> {
    let generate_data_key = kms_client
        .generate_data_key()
        .key_id(kms_key_id)
        .number_of_bytes(KMS_KEY_SIZE_IN_BYTES)
        .encryption_context(context.0, context.1)
        .send()
        .await?;

    let cipher_text = generate_data_key
        .ciphertext_blob
        .context("KMS cipher text is empty.")?;

    let plain_text = generate_data_key
        .plaintext
        .context("KMS plain text is empty.")?;

    Ok(Blobs {
        cipher_text,
        plain_text,
    })
}

pub struct Blobs {
    pub cipher_text: Blob,
    pub plain_text: Blob,
}
