use serde::{Deserialize, Serialize};

/// Represents the supported key usage types for KMS keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyUsage {
    /// For encryption and decryption operations.
    EncryptDecrypt,
    /// For signing and verification operations.
    SignVerify,
}

impl ::std::fmt::Display for KeyUsage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EncryptDecrypt => "encrypt-decrypt",
                Self::SignVerify => "sign-verify",
            }
        )
    }
}

/// Represents the supported encryption algorithms for KMS keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM, for use with KeyUsage::EncryptDecrypt.
    Aes256Gcm,
    /// AES-128-GCM, for use with KeyUsage::EncryptDecrypt.
    Aes128Gcm,
    /// RSA with a 4096-bit key, for use with KeyUsage::SignVerify.
    Rsa4096,
    /// Elliptic Curve Cryptography using the NIST P-256 curve, for use with KeyUsage::SignVerify.
    EccNistP256,
}

impl ::std::fmt::Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Aes256Gcm => "aes-256-gcm",
                Self::Aes128Gcm => "aes-128-gcm",
                Self::Rsa4096 => "RSA_4096",
                Self::EccNistP256 => "ECC_NIST_P256",
            }
        )
    }
}

/// Represents the supported signing algorithms for KMS keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigningAlgorithm {
    /// RSASSA-PSS with SHA-512.
    RsassaPssSha512,
    /// RSASSA-PSS with SHA-384.
    RsassaPssSha384,
    /// RSASSA-PSS with SHA-256.
    RsassaPssSha256,
    /// RSASSA-PKCS1-v1.5 with SHA-512.
    RsassaPkcs1V15Sha512,
    /// RSASSA-PKCS1-v1.5 with SHA-384.
    RsassaPkcs1V15Sha384,
    /// RSASSA-PKCS1-v1.5 with SHA-256.
    RsassaPkcs1V15Sha256,
    /// ECDSA with SHA-512.
    EcdsaSha512,
    /// ECDSA with SHA-384.
    EcdsaSha384,
    /// ECDSA with SHA-256.
    EcdsaSha256,
}

impl ::std::fmt::Display for SigningAlgorithm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::RsassaPssSha512 => "RSASSA_PSS_SHA_512",
                Self::RsassaPssSha384 => "RSASSA_PSS_SHA_384",
                Self::RsassaPssSha256 => "RSASSA_PSS_SHA_256",
                Self::RsassaPkcs1V15Sha512 => "RSASSA_PKCS1_V1_5_SHA_512",
                Self::RsassaPkcs1V15Sha384 => "RSASSA_PKCS1_V1_5_SHA_384",
                Self::RsassaPkcs1V15Sha256 => "RSASSA_PKCS1_V1_5_SHA_256",
                Self::EcdsaSha512 => "ECDSA_SHA_512",
                Self::EcdsaSha384 => "ECDSA_SHA_384",
                Self::EcdsaSha256 => "ECDSA_SHA_256",
            }
        )
    }
}

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
