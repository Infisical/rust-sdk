use reqwest::StatusCode;

use crate::error::api_error_handler;
use crate::helper::build_base_request;
use crate::{Client, DeleteSecretOptions, DeleteSecretResponse, Result, Secret};

pub async fn delete_secret(client: &mut Client, request: DeleteSecretOptions) -> Result<Secret> {
    let base_url = format!(
        "{}/api/v3/secrets/raw/{}",
        client.base_url.clone(),
        request.secret_name
    );

    let json = &serde_json::json!({
        "workspaceId": request.project_id,
        "environment": request.environment,
        "type": request.r#type.as_ref().unwrap_or(&"shared".to_string()),
        "secretPath": request.path.as_ref().unwrap_or(&"/".to_string()),
    });

    let base_request = build_base_request(client, &base_url, reqwest::Method::DELETE).await?;

    let response = base_request.json(json).send().await?;
    let status = response.status();

    if status == StatusCode::OK {
        let response = response.json::<DeleteSecretResponse>().await?;

        Ok(response.secret)
    } else {
        let err = api_error_handler(status, response, Some(request.secret_name.clone())).await?;
        Err(err)
    }
}
