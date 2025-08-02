use std::time::Duration;

use crate::{
    auth::{AuthHelper, AuthMethod},
    error::InfisicalError,
    resources::{kms::KmsClient, secrets::SecretsClient},
};

/// Infisical Client. Used to interact with the Infisical API.
///
/// Use the [Client::builder()] to construct an instance.
#[derive(Debug)]
pub struct Client {
    /// Base URL of your Infisical instance.
    pub base_url: String,
    /// HTTP Client used to make requests.
    pub http_client: reqwest::Client,
    request_timeout: Duration,
    user_agent: String,
    /// Indicates whether the client has successfully logged in.
    pub logged_in: bool,
}

/// A builder for creating an [Infisical Client](Client).
#[derive(Debug)]
pub struct ClientBuilder {
    base_url: Option<String>,
    user_agent: Option<String>,
    request_timeout: Option<Duration>,
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder`.
    fn new() -> Self {
        Self {
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
        Client::new(self.base_url, self.user_agent, self.request_timeout).await
    }
}

impl Client {
    /// Creates a new builder for the Infisical Client.
    /// The client will be initialized without authentication.
    /// Use [Client::login()] to add authentication later.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use infisical::{Client, AuthMethod};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::builder()
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///
    ///     let auth_method = AuthMethod::new_universal_auth("<your-client-id>", "<your-client-secret>");
    ///
    ///     client.login(auth_method)
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Internal method to create a new Infisical Client without authentication.
    /// The public interface is [Client::builder()].
    async fn new(
        base_url_opt: Option<String>,
        user_agent_opt: Option<String>,
        request_timeout_opt: Option<Duration>,
    ) -> Result<Self, InfisicalError> {
        let base_url = base_url_opt.unwrap_or_else(|| "https://app.infisical.com".to_string());

        let user_agent = user_agent_opt.unwrap_or_else(|| "infisical-rs".to_string());

        let timeout = request_timeout_opt.unwrap_or_else(|| Duration::from_secs(10));

        // Initialize http_client without any authorization headers
        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .user_agent(user_agent.clone())
            .use_rustls_tls()
            .build()?;

        Ok(Self {
            base_url,
            http_client,
            request_timeout: timeout,
            user_agent,
            logged_in: false,
        })
    }

    /// Logs in the client using the provided authentication method.
    /// This will obtain an access token and update the internal HTTP client
    /// to include the authorization headers for subsequent requests.
    pub async fn login(&mut self, auth_method: AuthMethod) -> Result<(), InfisicalError> {
        let token = AuthHelper::new(&self.base_url)
            .get_access_token(&self.http_client, auth_method)
            .await?;

        let mut headers = reqwest::header::HeaderMap::new();
        let formatted_token = format!("Bearer {token}");

        let mut auth_value = reqwest::header::HeaderValue::from_str(&formatted_token)?;
        auth_value.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);

        let new_http_client = reqwest::Client::builder()
            .timeout(self.request_timeout)
            .user_agent(self.user_agent.clone())
            .use_rustls_tls()
            .default_headers(headers)
            .build()?;

        self.http_client = new_http_client;
        self.logged_in = true;

        Ok(())
    }

    /// Access secrets operations.
    pub fn secrets(&self) -> SecretsClient {
        SecretsClient::new(self)
    }

    /// Access KMS operations.
    pub fn kms(&self) -> KmsClient {
        KmsClient::new(self)
    }
}
