use reqwest::StatusCode;

use crate::error::api_error_handler;
use crate::helper::{build_base_request, build_url, ensure_unique_secrets_by_key, set_env_vars};
use crate::{Client, ImportResponse, ListSecretsOptions, Result, Secret};

pub async fn list_secrets(client: &mut Client, request: ListSecretsOptions) -> Result<Vec<Secret>> {
    let base_url = format!("{}/api/v3/secrets/raw", client.base_url.clone());

    let json = &serde_json::json!({
        "environment": request.environment,
        "workspaceId": request.project_id,

        "expandSecretReferences": request.expand_secret_references.unwrap_or(true).to_string(),
        "recursive": request.recursive.unwrap_or(false).to_string(),
        "secretPath": request.path.as_ref().unwrap_or(&"/".to_string()), // default is "/"
        "include_imports": "true"

    });

    let url = &build_url(base_url, json);
    let base_request = build_base_request(client, url, reqwest::Method::GET).await?;

    let response = base_request.json(json).send().await?;
    let status = response.status();

    if status == StatusCode::OK {
        let mut response = response.json::<ImportResponse>().await?;

        if request.recursive.unwrap_or(false) {
            ensure_unique_secrets_by_key(&mut response.secrets);
        }

        let mut secrets = response.secrets.clone();

        for import in response.imports {
            for import_secret in import.secrets.clone() {
                // CASE: We need to ensure that the imported values don't override the "base" secrets.
                // Priority order is:
                // Local/Preset variables -> Actual secrets -> Imported secrets (high->low)

                // Check if the secret already exists in the secrets list
                if !secrets
                    .iter()
                    .any(|secret| secret.secret_key == import_secret.secret_key)
                {
                    secrets.push(import_secret);
                }
            }
        }

        set_env_vars(request.attach_to_process_env.unwrap_or(false), &secrets);

        Ok(secrets)
    } else {
        let err = api_error_handler(status, response, None).await?;
        Err(err)
    }
}
