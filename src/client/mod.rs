use std::time::Duration;

use crate::{
    auth::{AuthHelper, AuthMethod},
    error::InfisicalError,
    resources::secrets::SecretsClient,
};

/// Infisical Client. Used to interact with the Infisical API.
///
/// Use the [Client::builder()] to construct an instance.
#[derive(Debug, Clone)]
pub struct Client {
    /// Base URL of your Infisical instance.
    pub base_url: String,
    /// HTTP Client used to make requests.
    pub http_client: reqwest::Client,
}

/// A builder for creating an [Infisical Client](Client).
#[derive(Debug)]
pub struct ClientBuilder {
    auth_method: AuthMethod,
    base_url: Option<String>,
    user_agent: Option<String>,
    request_timeout: Option<Duration>,
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder`.
    fn new(auth_method: AuthMethod) -> Self {
        Self {
            auth_method,
            base_url: None,
            user_agent: None,
            request_timeout: None,
        }
    }

    /// Sets the base URL of the Infisical instance.
    ///
    /// Defaults to `https://app.infisical.com`.
    #[must_use]
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the user agent for the HTTP client.
    ///
    /// Defaults to `infisical-rs`.
    #[must_use]
    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets the request timeout for the HTTP client.
    ///
    /// Defaults to 10 seconds.
    #[must_use]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    /// Builds the `Client`.
    pub async fn build(self) -> Result<Client, InfisicalError> {
        let params = ClientParams {
            auth_method: Some(self.auth_method),
            base_url: self.base_url,
            user_agent: self.user_agent,
            request_timeout: self.request_timeout,
        };

        Client::new(params).await
    }
}

#[derive(Debug)]
pub(crate) struct ClientParams<S: Into<String>> {
    pub auth_method: Option<AuthMethod>,
    pub base_url: Option<S>,
    pub request_timeout: Option<Duration>,
    pub user_agent: Option<S>,
}

impl Client {
    /// Creates a new builder for the Infisical Client.
    ///
    /// # Arguments
    ///
    /// * `auth_method` - The authentication method to use.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use infisical::{Client, AuthMethod};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let auth_method = AuthMethod::UniversalAuth {
    ///         client_id: "YOUR_CLIENT_ID".to_string(),
    ///         client_secret: "YOUR_CLIENT_SECRET".to_string(),
    ///     };
    ///
    ///     let client = Client::builder(auth_method)
    ///         .build()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn builder(auth_method: AuthMethod) -> ClientBuilder {
        ClientBuilder::new(auth_method)
    }

    /// Internal method to create a new Infisical Client.
    /// The public interface is [Client::builder()].
    async fn new<S>(params: ClientParams<S>) -> Result<Self, InfisicalError>
    where
        S: Into<String> + Clone,
    {
        let base_url = params
            .base_url
            .map(|s| s.into())
            .unwrap_or_else(|| "https://app.infisical.com".to_string());

        let user_agent = params
            .user_agent
            .map(|s| s.into())
            .unwrap_or_else(|| "infisical-rs".to_string());

        let timeout = params
            .request_timeout
            .unwrap_or_else(|| Duration::from_secs(10));

        let temp_http_client = reqwest::Client::builder()
            .timeout(timeout)
            .user_agent(user_agent.clone())
            .use_rustls_tls()
            .build()?;

        let auth_method = params
            .auth_method
            .ok_or(InfisicalError::InvalidAuthMethod)?;

        let token = AuthHelper::new(&base_url)
            .get_access_token(&temp_http_client, auth_method)
            .await?;

        let mut headers = reqwest::header::HeaderMap::new();
        let formatted_token = format!("Bearer {}", token);

        let mut auth_value = reqwest::header::HeaderValue::from_str(&formatted_token)?;
        auth_value.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);

        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .user_agent(user_agent)
            .use_rustls_tls()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            base_url,
            http_client,
        })
    }

    /// Access secrets operations.
    pub fn secrets(&self) -> SecretsClient {
        SecretsClient::new(self)
    }
}
