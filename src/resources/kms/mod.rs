mod create;
mod decrypt;
mod delete;
mod encrypt;
mod get;
pub mod helper;
mod list;
mod sign;
mod types;
mod update;
mod verify;

pub use create::*;
pub use decrypt::*;
pub use delete::*;
pub use encrypt::*;
pub use get::*;
pub use helper::{decode_base64, encode_base64};
pub use list::*;
pub use sign::*;
pub use types::*;
pub use update::*;
pub use verify::*;

use crate::{
    client::Client,
    error::InfisicalError,
    resources::helper::{build_url, check_response},
};

/// Provides access to KMS operations.
pub struct KmsClient<'a> {
    client: &'a Client,
}

impl<'a> KmsClient<'a> {
    /// Creates a new `KmsClient`.
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

    /// Lists KMS keys in a given project.
    pub async fn list(&self, request: ListKmsKeysRequest) -> Result<Vec<KmsKey>, InfisicalError> {
        let base_url = format!("{}/api/v1/kms/keys", self.client.base_url);

        let query_params = serde_json::json!({
            "projectId": request.project_id,
        });

        let url = build_url(&base_url, &query_params)?;

        let response: ListKmsKeysResponse = self.get_helper(url).await?;

        Ok(response.keys)
    }

    /// Gets a KMS key by ID.
    pub async fn get(&self, request: GetKmsKeyRequest) -> Result<KmsKey, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}",
            self.client.base_url, request.key_id
        );

        let response: KmsKeyResponse = self.get_helper(url).await?;

        Ok(response.key)
    }

    /// Gets a KMS key by name.
    pub async fn get_by_name(
        &self,
        request: GetKmsKeyByNameRequest,
    ) -> Result<KmsKey, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/key-name/{}",
            self.client.base_url, request.key_name
        );

        let response: KmsKeyResponse = self.get_helper(url).await?;

        Ok(response.key)
    }

    /// Creates a new KMS key.
    pub async fn create(&self, request: CreateKmsKeyRequest) -> Result<KmsKey, InfisicalError> {
        let url = format!("{}/api/v1/kms/keys", self.client.base_url);

        let body = serde_json::json!({
            "projectId": request.project_id,
            "name": request.name,
            "description": request.description.as_deref().unwrap_or(""),
            "keyUsage": request.key_usage.as_deref().unwrap_or("encrypt-decrypt"),
            "encryptionAlgorithm": request.encryption_algorithm.as_deref().unwrap_or("aes-256-gcm"),
        });

        let response: KmsKeyResponse = self
            .request_with_body(reqwest::Method::POST, url, &body)
            .await?;
        Ok(response.key)
    }

    /// Updates an existing KMS key.
    pub async fn update(&self, request: UpdateKmsKeyRequest) -> Result<KmsKey, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}",
            self.client.base_url, request.key_id
        );

        let mut body = serde_json::Map::new();
        if let Some(name) = request.name {
            body.insert("name".to_string(), serde_json::Value::String(name));
        }
        if let Some(is_disabled) = request.is_disabled {
            body.insert(
                "isDisabled".to_string(),
                serde_json::Value::Bool(is_disabled),
            );
        }
        if let Some(description) = request.description {
            body.insert(
                "description".to_string(),
                serde_json::Value::String(description),
            );
        }

        let response: KmsKeyResponse = self
            .request_with_body(
                reqwest::Method::PATCH,
                url,
                &serde_json::Value::Object(body),
            )
            .await?;
        Ok(response.key)
    }

    /// Deletes a KMS key.
    pub async fn delete(&self, request: DeleteKmsKeyRequest) -> Result<KmsKey, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}",
            self.client.base_url, request.key_id
        );

        let response: KmsKeyResponse = self
            .request_with_body(reqwest::Method::DELETE, url, &serde_json::Value::Null)
            .await?;
        Ok(response.key)
    }

    /// Encrypts data using a KMS key.
    pub async fn encrypt(&self, request: EncryptRequest) -> Result<String, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}/encrypt",
            self.client.base_url, request.key_id
        );

        let body = serde_json::json!({
            "plaintext": request.plaintext,
        });

        let response: EncryptResponse = self
            .request_with_body(reqwest::Method::POST, url, &body)
            .await?;
        Ok(response.ciphertext)
    }

    /// Decrypts data using a KMS key.
    pub async fn decrypt(&self, request: DecryptRequest) -> Result<String, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}/decrypt",
            self.client.base_url, request.key_id
        );

        let body = serde_json::json!({
            "ciphertext": request.ciphertext,
        });

        let response: DecryptResponse = self
            .request_with_body(reqwest::Method::POST, url, &body)
            .await?;
        Ok(response.plaintext)
    }

    /// Signs data using a KMS key.
    pub async fn sign(&self, request: SignRequest) -> Result<SignResponse, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}/sign",
            self.client.base_url, request.key_id
        );

        let body = serde_json::json!({
            "signingAlgorithm": request.signing_algorithm.as_deref().unwrap_or("RSASSA_PKCS1_V1_5_SHA_256"),
            "isDigest": request.is_digest.unwrap_or(false),
            "data": request.data,
        });

        let response: SignResponse = self
            .request_with_body(reqwest::Method::POST, url, &body)
            .await?;
        Ok(response)
    }

    /// Verifies a signature using a KMS key.
    pub async fn verify(&self, request: VerifyRequest) -> Result<VerifyResponse, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}/verify",
            self.client.base_url, request.key_id
        );

        let body = serde_json::json!({
            "isDigest": request.is_digest.unwrap_or(false),
            "data": request.data,
            "signature": request.signature,
            "signingAlgorithm": request.signing_algorithm.as_deref().unwrap_or("RSASSA_PKCS1_V1_5_SHA_256"),
        });

        let response: VerifyResponse = self
            .request_with_body(reqwest::Method::POST, url, &body)
            .await?;
        Ok(response)
    }

    /// Gets the public key for a KMS key.
    pub async fn get_public_key(&self, key_id: &str) -> Result<String, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}/public-key",
            self.client.base_url, key_id
        );

        let response: PublicKeyResponse = self.get_helper(url).await?;
        Ok(response.public_key)
    }

    /// Gets the signing algorithms for a KMS key.
    pub async fn get_signing_algorithms(
        &self,
        key_id: &str,
    ) -> Result<Vec<String>, InfisicalError> {
        let url = format!(
            "{}/api/v1/kms/keys/{}/signing-algorithms",
            self.client.base_url, key_id
        );

        let response: SigningAlgorithmsResponse = self.get_helper(url).await?;
        Ok(response.signing_algorithms)
    }
}
