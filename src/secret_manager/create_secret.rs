use reqwest::StatusCode;

use crate::error::api_error_handler;
use crate::helper::build_base_request;
use crate::{Client, CreateSecretOptions, CreateSecretResponse, Result, Secret};

pub async fn create_secret(client: &mut Client, request: CreateSecretOptions) -> Result<Secret> {
    let base_url = format!(
        "{}/api/v3/secrets/raw/{}",
        client.base_url.clone(),
        request.secret_name
    );

    let json = &serde_json::json!({
        "environment": request.environment,
        "workspaceId": request.project_id,
        "secretValue": request.secret_value,

        // conditionally add path, and if its not there add type shared
        "type": request.r#type.as_ref().unwrap_or(&"shared".to_string()), // default is shared
        "secretPath": request.path.as_ref().unwrap_or(&"/".to_string()), // default is "/"
        "secretComment": request.secret_comment.as_ref().unwrap_or(&"".to_string()), // default is "/"
        "skipMultilineEncoding": request.skip_multiline_encoding.as_ref().unwrap_or(&false), // default is "/"

    });

    let base_request = build_base_request(client, &base_url, reqwest::Method::POST).await?;

    let response = base_request.json(json).send().await?;
    let status = response.status();

    if status == StatusCode::OK {
        let res = response.json::<CreateSecretResponse>().await?;
        Ok(res.secret)
    } else {
        let err = api_error_handler(status, response, Some(request.secret_name.clone())).await?;
        Err(err)
    }
}
