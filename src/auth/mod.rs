use serde::{Deserialize, Serialize};

use crate::{error::InfisicalError, resources::helper::check_response};

/// Authentication methods for the Infisical API.
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// Universal authentication using a client ID and client secret.
    UniversalAuth {
        client_id: String,
        client_secret: String,
    },
}

impl AuthMethod {
    /// Creates a new `AuthMethod::UniversalAuth` variant.
    pub fn new_universal_auth(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        AuthMethod::UniversalAuth {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

/// Represents a successful access token response from the server.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenSuccessResponse {
    pub access_token: String,
    pub expires_in: u64,
    #[serde(rename = "accessTokenMaxTTL")]
    pub access_token_max_ttl: u64,
    pub token_type: String,
}

pub struct AuthHelper {
    base_url: String,
}

impl AuthHelper {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Converts an `AuthMethod` into a bearer token by calling the appropriate API endpoint.
    pub async fn get_access_token(
        &self,
        http_client: &reqwest::Client,
        auth_method: AuthMethod,
    ) -> Result<String, InfisicalError> {
        match auth_method {
            AuthMethod::UniversalAuth {
                client_id,
                client_secret,
            } => {
                self.exchange_universal_auth(http_client, &client_id, &client_secret)
                    .await
            }
        }
    }

    async fn exchange_universal_auth(
        &self,
        http_client: &reqwest::Client,
        client_id: &str,
        client_secret: &str,
    ) -> Result<String, InfisicalError> {
        let url = format!("{}/api/v1/auth/universal-auth/login", self.base_url);

        let response = http_client
            .post(&url)
            .json(&serde_json::json!({
                "clientId": client_id,
                "clientSecret": client_secret
            }))
            .send()
            .await?;
        let response = check_response(response).await?;

        let login_response = response.json::<AccessTokenSuccessResponse>().await?;

        Ok(login_response.access_token)
    }
}
