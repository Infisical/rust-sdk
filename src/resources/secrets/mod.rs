mod create;
mod delete;
mod get;
mod helper;
mod list;
mod types;
mod update;

use std::collections::HashSet;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use types::*;
pub use update::*;

use crate::{
    client::Client,
    error::InfisicalError,
    resources::helper::{build_url, check_response},
};
use helper::{ensure_unique_secrets_by_key, set_env_vars};
use serde::{Deserialize, Serialize};

/// Provides access to secrets operations.
pub struct SecretsClient<'a> {
    client: &'a Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct SecretResponse<T> {
    secret: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListSecretsResponseImports {
    pub secret_path: String,
    pub folder_id: String,
    pub environment: String,
    pub secrets: Vec<Secret>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImportResponse {
    pub imports: Vec<ListSecretsResponseImports>,
    pub secrets: Vec<Secret>,
}

impl<'a> SecretsClient<'a> {
    /// Creates a new `SecretsClient`.
    ///
    /// This client is not meant to be constructed directly, but rather
    /// through the main `Infisical` client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // Internal helper for GET requests
    async fn get_helper<T: for<'de> serde::Deserialize<'de>>(
        &self,
        url: String,
    ) -> Result<T, InfisicalError> {
        if !self.client.logged_in {
            return Err(InfisicalError::NotAuthenticated);
        }

        let response = self
            .client
            .http_client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        let response = check_response(response).await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }

    // Internal helper for POST/PATCH/DELETE requests with a body
    async fn request_with_body<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        url: String,
        body: &serde_json::Value,
    ) -> Result<T, InfisicalError> {
        if !self.client.logged_in {
            return Err(InfisicalError::NotAuthenticated);
        }

        let response = self
            .client
            .http_client
            .request(method, url)
            .json(body)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        let response = check_response(response).await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }

    /// Gets a secret by name.
    pub async fn get(&self, request: GetSecretRequest) -> Result<Secret, InfisicalError> {
        let base_url = format!(
            "{}/api/v3/secrets/raw/{}",
            self.client.base_url, request.secret_name
        );

        let query_params = serde_json::json!({
            "workspaceId": request.project_id,
            "environment": request.environment,
            "secretPath": request.path.as_deref().unwrap_or("/"),
            "expandSecretReferences": request.expand_secret_references.unwrap_or(true).to_string(),
            "type": request.r#type.as_deref().unwrap_or("shared"),
            "include_imports": "true",
        });

        let url = build_url(&base_url, &query_params)?;

        let response: SecretResponse<Secret> = self.get_helper(url).await?;

        Ok(response.secret)
    }

    /// Lists secrets in a given project and environment.
    pub async fn list(&self, request: ListSecretsRequest) -> Result<Vec<Secret>, InfisicalError> {
        let base_url = format!("{}/api/v3/secrets/raw", self.client.base_url,);

        let query_params = serde_json::json!({
            "workspaceId": request.project_id,
            "environment": request.environment,
            "secretPath": request.path.as_deref().unwrap_or("/"),
            "expandSecretReferences": request.expand_secret_references.unwrap_or(true).to_string(),
            "recursive": request.recursive.unwrap_or(false).to_string(),
            "include_imports": "true",
        });

        let url = build_url(&base_url, &query_params)?;

        let mut response: ImportResponse = self.get_helper(url).await?;

        if request.recursive.unwrap_or(false) {
            ensure_unique_secrets_by_key(&mut response.secrets);
        }

        let mut secrets = response.secrets;
        let existing_keys: HashSet<String> = secrets.iter().map(|s| s.secret_key.clone()).collect();

        for import in response.imports {
            for import_secret in import.secrets {
                if !existing_keys.contains(&import_secret.secret_key) {
                    secrets.push(import_secret);
                }
            }
        }

        set_env_vars(request.attach_to_process_env.unwrap_or(false), &secrets);

        Ok(secrets)
    }

    /// Creates a new secret.
    pub async fn create(&self, request: CreateSecretRequest) -> Result<Secret, InfisicalError> {
        let base_url = format!(
            "{}/api/v3/secrets/raw/{}",
            self.client.base_url, request.secret_name
        );

        let body = serde_json::json!({
            "secretValue": request.secret_value,
            "workspaceId": request.project_id,
            "environment": request.environment,
            "secretPath": request.path.as_deref().unwrap_or("/"),
            "type": request.r#type.as_deref().unwrap_or("shared"),
            "secretComment": request.secret_comment.as_deref().unwrap_or(""),
            "skipMultilineEncoding": request.skip_multiline_encoding.unwrap_or(false),
        });

        let response: SecretResponse<Secret> = self
            .request_with_body(reqwest::Method::POST, base_url, &body)
            .await?;
        Ok(response.secret)
    }

    /// Updates an existing secret.
    pub async fn update(&self, request: UpdateSecretRequest) -> Result<Secret, InfisicalError> {
        let base_url = format!(
            "{}/api/v3/secrets/raw/{}",
            self.client.base_url, request.secret_name
        );

        let mut body = serde_json::Map::new();

        // Required fields
        body.insert(
            "workspaceId".to_string(),
            serde_json::Value::String(request.project_id),
        );
        body.insert(
            "environment".to_string(),
            serde_json::Value::String(request.environment),
        );

        // Optional fields
        if let Some(new_secret_name) = request.new_secret_name {
            body.insert(
                "newSecretName".to_string(),
                serde_json::Value::String(new_secret_name),
            );
        }
        if let Some(secret_value) = request.secret_value {
            body.insert(
                "secretValue".to_string(),
                serde_json::Value::String(secret_value),
            );
        }
        if let Some(path) = request.path {
            body.insert("secretPath".to_string(), serde_json::Value::String(path));
        }
        if let Some(r#type) = request.r#type {
            body.insert("type".to_string(), serde_json::Value::String(r#type));
        }
        if let Some(secret_comment) = request.secret_comment {
            body.insert(
                "secretComment".to_string(),
                serde_json::Value::String(secret_comment),
            );
        }
        if let Some(skip_multiline_encoding) = request.skip_multiline_encoding {
            body.insert(
                "skipMultilineEncoding".to_string(),
                serde_json::Value::Bool(skip_multiline_encoding),
            );
        }

        let response: SecretResponse<Secret> = self
            .request_with_body(
                reqwest::Method::PATCH,
                base_url,
                &serde_json::Value::Object(body),
            )
            .await?;
        Ok(response.secret)
    }

    /// Deletes a secret.
    pub async fn delete(&self, request: DeleteSecretRequest) -> Result<Secret, InfisicalError> {
        let base_url = format!(
            "{}/api/v3/secrets/raw/{}",
            self.client.base_url, request.secret_name
        );

        let body = serde_json::json!({
            "workspaceId": request.project_id,
            "environment": request.environment,
            "secretPath": request.path.as_deref().unwrap_or("/"),
            "type": request.r#type.as_deref().unwrap_or("shared"),
        });

        let response: SecretResponse<Secret> = self
            .request_with_body(reqwest::Method::DELETE, base_url, &body)
            .await?;
        Ok(response.secret)
    }
}
