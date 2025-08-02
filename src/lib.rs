// Infisical
// (c) 2025 Infisical, under MIT license

//! Official Rust SDK for Infisical

pub mod auth;
pub mod client;
pub mod error;
pub mod resources;

#[cfg(test)]
mod tests;

pub mod secrets {
    pub use crate::resources::secrets::*;
}

pub mod kms {
    pub use crate::resources::kms::*;
}

pub use auth::AuthMethod;
pub use client::Client;
pub use resources::kms::{decode_base64, encode_base64};
