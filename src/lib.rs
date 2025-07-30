// Infisical
// (c) 2025 Infisical, under MIT license

//! Official Rust SDK for Infisical

pub mod auth;
pub mod client;
pub mod error;
pub mod resources;

#[cfg(test)]
mod tests;

pub use auth::AuthMethod;
pub use client::Client;
pub use resources::kms::{
    decode_b64, encode_b64, CreateKmsKeyRequest, DecryptRequest, DeleteKmsKeyRequest,
    EncryptRequest, GetKmsKeyByNameRequest, GetKmsKeyRequest, KmsKey, ListKmsKeysRequest,
    SignRequest, UpdateKmsKeyRequest, VerifyRequest,
};
