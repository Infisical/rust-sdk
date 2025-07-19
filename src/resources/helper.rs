use url::Url;

use crate::error::InfisicalError;

/// Builds a URL by appending query parameters from a JSON object.
pub fn build_url(base_url: &str, params: &serde_json::Value) -> Result<String, InfisicalError> {
    let mut url = Url::parse(base_url)?;

    if let Some(obj) = params.as_object() {
        let mut query_pairs = url.query_pairs_mut();
        for (key, value) in obj {
            if let Some(value_str) = value.as_str() {
                query_pairs.append_pair(key, value_str);
            }
        }
    }

    Ok(url.to_string())
}

/// Helper to check HTTP response status and return `InfisicalError` if unsuccessful.
pub async fn check_response(
    response: reqwest::Response,
) -> Result<reqwest::Response, InfisicalError> {
    if !response.status().is_success() {
        let status = response.status();
        let message = response.text().await.unwrap_or_default();
        return Err(InfisicalError::HttpError { status, message });
    }
    Ok(response)
}
