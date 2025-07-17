use reqwest::StatusCode;

use crate::error::api_error_handler;
use crate::helper::{build_base_request, build_url, get_fallback_env_secret};
use crate::{Client, GetSecretOptions, GetSecretResponse, Result, Secret};

pub async fn get_secret(client: &mut Client, request: GetSecretOptions) -> Result<Secret> {
    let base_url = format!(
        "{}/api/v3/secrets/raw/{}",
        client.base_url.clone(),
        &request.secret_name
    );

    let json: &serde_json::Value = &serde_json::json!({
        "workspaceId": request.project_id,
        "environment": request.environment,
        "secretPath": request.path.clone().unwrap_or("/".to_string()), // default is "/"
        "expandSecretReferences": request.expand_secret_references.unwrap_or(true).to_string(),
        "type": request.r#type.clone().unwrap_or("shared".to_string()), // default is shared
        "include_imports": request.include_imports.unwrap_or(false).to_string(),
    });

    let url = build_url(base_url, json);

    let base_request = build_base_request(client, &url, reqwest::Method::GET).await?;

    let response = base_request.send().await?;

    let status = response.status();

    if status == StatusCode::OK {
        let response = response.json::<GetSecretResponse>().await?;

        Ok(response.secret)
    } else {
        let fallback_secret = get_fallback_env_secret(&request.secret_name);

        if let Some(fallback_secret) = fallback_secret {
            return Ok(fallback_secret);
        }

        let err = api_error_handler(status, response, Some(request.secret_name.clone())).await?;
        Err(err)
    }
}
