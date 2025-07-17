use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde()]
pub struct UnauthorizedError {
    pub message: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Request Failed: [status-code={status}] [request-id={request_id}] [message={message}]"
    )]
    RequestFailed {
        status: StatusCode,
        request_id: String,
        message: String,
    },

    // Regular unauthorized error
    #[error("Failed to authenticate: [request-id={request_id}] [message={message}]")]
    Unauthorized { message: String, request_id: String },

    // Secret not found
    #[error("Secret with name '{}' not found. [request-id={request_id}] [message={message}]", .secret_name)]
    SecretNotFound {
        secret_name: String,
        request_id: String,
        message: String,
    },

    #[error("404 Not found")]
    NotFound,

    #[error("Bad Request: [request-id={request_id}] [message={message}]")]
    BadRequest { request_id: String, message: String },

    #[error("Authentication Failed: [status-code={status}] [request-id={request_id}] [message={message}]")]
    AuthenticationFailed {
        status: StatusCode,
        request_id: String,
        message: String,
    },

    #[error("Rate Limited: [status-code={status}] [request-id={request_id}] [message={message}]")]
    RateLimited {
        status: StatusCode,
        request_id: String,
        message: String,
    },

    // Generic "base" error. This is the last resort error really.
    #[error("Received error message from server: (status {}), {}", .status, .message)]
    ResponseContent { status: StatusCode, message: String },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub message: String,
    #[serde(rename = "reqId")]
    pub req_id: String,
}

async fn try_parse_api_error(text: &str) -> Result<ApiErrorResponse> {
    let json_response = serde_json::from_str::<ApiErrorResponse>(text).map_err(Error::Serde)?;

    Ok(json_response)
}

pub async fn api_error_handler(
    status: StatusCode,
    res: Response,
    secret_name: Option<String>,
) -> Result<Error> {
    // Read the response body as text once
    let parsed_response = res.text().await.map_err(Error::Reqwest);

    let response_text = match parsed_response {
        Ok(text) => text,
        Err(_) => "".to_string(),
    };

    let api_error = &try_parse_api_error(&response_text).await;

    let request_id = match api_error {
        Ok(api_error) => api_error.req_id.clone(),
        Err(_) => "".to_string(),
    };

    let message = match api_error {
        Ok(api_error) => api_error.message.clone(),
        Err(_) => "".to_string(),
    };

    if status == StatusCode::NOT_FOUND {
        if secret_name.is_some() {
            let s = match secret_name {
                Some(secret_name) => secret_name,
                None => "".to_string(),
            };

            return Err(Error::SecretNotFound {
                secret_name: s,
                message,
                request_id,
            });
        } else {
            return Err(Error::NotFound);
        }
    }

    if status == StatusCode::BAD_REQUEST {
        return Ok(Error::BadRequest {
            message,
            request_id,
        });
    }

    if status == StatusCode::UNAUTHORIZED {
        return Ok(Error::Unauthorized {
            message,
            request_id,
        });
    }

    // Now we can parse the same text as a generic JSON value for the fallback
    let json_string = serde_json::from_str::<serde_json::Value>(&response_text);

    let err_message = match json_string {
        Ok(json) => json.to_string(),
        Err(_) => "Failed to decode error message".to_string(),
    };

    Ok(Error::ResponseContent {
        status,
        message: err_message,
    })
}
