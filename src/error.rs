use aws_sigv4::http_request::SigningError;
use base64::DecodeError;
use reqwest::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    StatusCode,
};
use serde_json::Error as SerdeError;
use std::string::FromUtf8Error;
use thiserror::Error;
use url::ParseError;

/// Infisical Errors.
#[derive(Debug, Error)]
pub enum InfisicalError {
    /// Failed to build a http request.
    #[error("Failed to build a http request: {0}")]
    RequestBuildError(#[from] http::Error),

    /// An unexpected response was returned from API causing a deserialization error.
    #[error("Failed to process API response: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Failed to create a valid authorization header name.
    #[error("Failed to create authorization header: {0}")]
    InvalidAuthHeaderName(#[from] InvalidHeaderName),

    /// Failed to create a valid authorization header value.
    #[error("Failed to create authorization header: {0}")]
    InvalidAuthHeaderValue(#[from] InvalidHeaderValue),

    /// Generic HTTP error.
    #[error("Received an HTTP error from server: {status}")]
    HttpError { status: StatusCode, message: String },

    /// Invalid auth method configured.
    #[error("You do not have a valid auth method configured.")]
    InvalidAuthMethod,

    /// Credentials cannot be used, they were either not provided or invalid.
    #[error("Credentials were not provided or are invalid.")]
    InvalidCredentials,

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

    /// Failed to create an AWS SignableRequest.
    #[error("Failed to create an AWS SignableRequest: {0}")]
    SigningError(#[from] SigningError),

    /// Serialization/Deserialization error.
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] SerdeError),
}
