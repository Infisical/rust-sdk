use std::time::Duration;

use crate::auth_manager::auth_client::AuthClient;
use crate::secret_client::SecretClient;
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) base_url: String,
    pub(crate) http_client: reqwest::Client,
    pub(crate) request_timeout: Option<Duration>,
    pub(crate) user_agent: String,
}

impl Client {
    pub fn new(
        base_url: impl Into<String>,
        http_client: reqwest::Client,
        request_timeout: Option<Duration>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            request_timeout,
            user_agent: "infisical-rs".to_string(),
        }
    }

    pub fn auth(&mut self) -> AuthClient {
        AuthClient::new(self)
    }

    pub fn secrets(&mut self) -> SecretClient {
        SecretClient::new(self)
    }

    pub(crate) fn update_http_client_auth_header(&mut self, token: &str) -> Result<bool> {
        let mut headers = reqwest::header::HeaderMap::new();

        let formatted_token = format!("Bearer {token}");
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&formatted_token)
                .map_err(Error::InvalidHeaderValue)?,
        );

        let new_http_client = reqwest::Client::builder()
            .timeout(self.request_timeout.unwrap_or(Duration::from_secs(10)))
            .user_agent("infisical-rs")
            .use_rustls_tls()
            .default_headers(headers)
            .build()
            .map_err(Error::Reqwest)?;

        self.http_client = new_http_client;

        Ok(true)
    }
}

pub struct ClientBuilder {
    host_url: String,
    request_timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            host_url: "https://app.infisical.com".to_string(),
            request_timeout: None,
        }
    }

    pub fn with_host_url(mut self, url: impl Into<String>) -> Self {
        self.host_url = url.into();
        self
    }

    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<Client> {
        let http_client = reqwest::Client::builder()
            .timeout(self.request_timeout.unwrap_or(Duration::from_secs(10)))
            .user_agent("infisical-rs")
            .use_rustls_tls()
            .build()
            .map_err(Error::Reqwest)?; // return early with the error if any

        let client = Client::new(self.host_url, http_client, self.request_timeout);

        Ok(client)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
