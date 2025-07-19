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
