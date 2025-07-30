use serde::{Deserialize, Serialize};

/// Represents a KMS key from the Infisical API.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KmsKey {
    pub id: String,
    pub description: String,
    pub is_disabled: bool,
    pub org_id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub project_id: String,
    pub key_usage: String,
    pub version: i32,
    pub encryption_algorithm: String,
}

/// Represents the response for KMS key operations.
#[derive(Serialize, Deserialize, Debug)]
pub struct KmsKeyResponse {
    pub key: KmsKey,
}

/// Represents the response for listing KMS keys.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListKmsKeysResponse {
    pub keys: Vec<KmsKey>,
    pub total_count: i32,
}

/// Represents the response for encryption operations.
#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptResponse {
    pub ciphertext: String,
}

/// Represents the response for decryption operations.
#[derive(Serialize, Deserialize, Debug)]
pub struct DecryptResponse {
    pub plaintext: String,
}

/// Represents the response for signing operations.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignResponse {
    pub signature: String,
    pub key_id: String,
    pub signing_algorithm: String,
}

/// Represents the response for verification operations.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyResponse {
    pub signature_valid: bool,
    pub key_id: String,
    pub signing_algorithm: String,
}

/// Represents the response for public key operations.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyResponse {
    pub public_key: String,
}

/// Represents the response for signing algorithms operations.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SigningAlgorithmsResponse {
    pub signing_algorithms: Vec<String>,
} 