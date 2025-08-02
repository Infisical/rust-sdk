use base64::DecodeError;
use reqwest::{header::InvalidHeaderValue, StatusCode};
use std::string::FromUtf8Error;
use thiserror::Error;
use url::ParseError;

/// Infisical Errors.
#[derive(Debug, Error)]
pub enum InfisicalError {
    /// An unexpected response was returned from API causing a deserialization error.
    #[error("Failed to process API response: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Failed to create a valid authorization header value.
    #[error("Failed to create authorization header: {0}")]
    InvalidAuthHeaderValue(#[from] InvalidHeaderValue),

    /// Generic HTTP error.
    #[error("Received an HTTP error from server: {status}")]
    HttpError { status: StatusCode, message: String },

    /// Invalid auth method configured.
    #[error("You do not have a valid auth method configured.")]
    InvalidAuthMethod,

    /// Failed to parse a URL.
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] ParseError),

    /// Attempted to make an authenticated request without logging in first.
    #[error("Client is not authenticated. Please call .login() first.")]
    NotAuthenticated,

    /// Failed to decode base64 data.
    #[error("Failed to decode base64 data: {0}")]
    Base64DecodeError(#[from] DecodeError),

    /// Failed to convert bytes to UTF-8 string.
    #[error("Failed to convert bytes to UTF-8 string: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
}
