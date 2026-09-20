use crate::{
    auth::aws::{login_request::AwsLoginRequest, login_response::AwsLoginResponse},
    InfisicalError,
};
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region, SdkConfig};
use aws_credential_types::{provider::ProvideCredentials, Credentials};
use aws_sigv4::http_request::{
    sign, PayloadChecksumKind, SignableBody, SignableRequest, SigningInstructions, SigningParams,
    SigningSettings,
};
use aws_smithy_runtime_api::client::identity::Identity;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use std::{collections::BTreeMap, str::FromStr, time::SystemTime};

const HEADER_CONTENT_LENGTH_KEY: &str = "content-length";
const HEADER_CONTENT_TYPE_VALUE: &str = "application/x-www-form-urlencoded";
const HEADER_SESSION_TOKEN_KEY: &str = "x-amz-security-token";
pub(crate) const INFISICAL_AWS_LOGIN_PATH: &str = "/api/v1/auth/aws-auth/login";

pub(crate) struct AwsAuthFlow;

impl AwsAuthFlow {
    pub async fn try_access_token(
        http_client: &reqwest::Client,
        base_url: &str,
        identity_id: &str,
    ) -> Result<String, InfisicalError> {
        let sdk_config = Self::sdk_config().await;
        Self::try_access_token_with_config(http_client, identity_id, base_url, &sdk_config).await
    }
    /// Allows e2e testing
    pub(crate) async fn try_access_token_with_config(
        http_client: &reqwest::Client,
        identity_id: &str,
        base_url: &str,
        sdk_config: &SdkConfig,
    ) -> Result<String, InfisicalError> {
        let region = Self::region(sdk_config);
        let credentials = Self::credentials(sdk_config).await?;
        let endpoint = Self::endpoint(&region);
        let headers = Self::headers(&credentials);
        let body = Self::body();
        let identity = credentials.into();
        let instructions =
            Self::signing_instructions(region.as_ref(), &identity, &endpoint, &headers, &body)?;
        let signed_headers = Self::signed_headers(instructions, &endpoint, &headers, body.len())?;
        let aws_response =
            Self::aws_login_response(base_url, http_client, identity_id, &signed_headers, &body)
                .await?;
        Ok(aws_response.access_token)
    }
}

// private helper functions
impl AwsAuthFlow {
    async fn sdk_config() -> SdkConfig {
        let chain = RegionProviderChain::default_provider();
        aws_config::defaults(BehaviorVersion::latest())
            .region(chain)
            .load()
            .await
    }

    fn region(config: &SdkConfig) -> Region {
        config
            .region()
            .cloned()
            .unwrap_or_else(|| Region::from_static("us-east-1"))
    }

    async fn credentials(config: &SdkConfig) -> Result<Credentials, InfisicalError> {
        config
            .credentials_provider()
            .ok_or(InfisicalError::InvalidAuthMethod)?
            .provide_credentials()
            .await
            .map_err(|_| InfisicalError::InvalidCredentials)
    }

    fn endpoint(region: &Region) -> String {
        format!("https://sts.{}.amazonaws.com", region)
    }

    fn headers(credentials: &Credentials) -> BTreeMap<String, String> {
        let mut headers = BTreeMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE.as_str().into(),
            HEADER_CONTENT_TYPE_VALUE.into(),
        );
        if let Some(token) = credentials.session_token() {
            headers.insert(HEADER_SESSION_TOKEN_KEY.into(), token.into());
        }
        headers
    }

    fn body() -> String {
        let mut params = BTreeMap::new();
        params.insert("Action".to_string(), vec!["GetCallerIdentity".to_string()]);
        params.insert("Version".to_string(), vec!["2011-06-15".to_string()]);
        params
            .iter()
            .flat_map(|(k, values)| values.iter().map(move |v| (k, v)))
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn signing_instructions<'a>(
        region: &'a str,
        identity: &'a Identity,
        endpoint: &'a str,
        headers: &'a BTreeMap<String, String>,
        body: &'a str,
    ) -> Result<SigningInstructions, InfisicalError> {
        let signable_request = SignableRequest::new(
            Method::POST.as_str(),
            endpoint,
            headers.iter().map(|(k, v)| (k.as_str(), v.as_str())),
            SignableBody::Bytes(body.as_bytes()),
        )
        .map_err(InfisicalError::SigningError)?;
        let mut signing_settings = SigningSettings::default();
        signing_settings.payload_checksum_kind = PayloadChecksumKind::XAmzSha256;
        let signing_params = SigningParams::V4(
            aws_sigv4::sign::v4::SigningParams::builder()
                .identity(identity)
                .region(region)
                .name("sts")
                .time(SystemTime::now())
                .settings(signing_settings)
                .build()
                .map_err(|_| InfisicalError::InvalidAuthMethod)?,
        );
        let (instructions, _signature) = sign(signable_request, &signing_params)
            .map_err(InfisicalError::SigningError)?
            .into_parts();
        Ok(instructions)
    }

    /// This uses aws-sigv4 to create signed headers
    /// aws-sigv4 requires http::Request, so we build a temporary one for it to sign
    /// then read them out again to apply to the real one sent via the reqwest client
    fn signed_headers(
        instructions: SigningInstructions,
        endpoint: &str,
        unsigned_headers: &BTreeMap<String, String>,
        body_len: usize,
    ) -> Result<BTreeMap<String, String>, InfisicalError> {
        let mut header_map = HeaderMap::new();
        for (key, value) in unsigned_headers {
            let name = HeaderName::from_str(key).map_err(InfisicalError::InvalidAuthHeaderName)?;
            let value =
                HeaderValue::from_str(value).map_err(InfisicalError::InvalidAuthHeaderValue)?;
            header_map.insert(name, value);
        }
        // Signing container. Never sent, built purely to allow aws-sigv4 to create signed headers
        let mut temp_signing_request = http::Request::builder()
            .method(http::Method::POST)
            .uri(endpoint)
            .body(())
            .map_err(InfisicalError::RequestBuildError)?;
        *temp_signing_request.headers_mut() = header_map;
        instructions.apply_to_request_http1x(&mut temp_signing_request);
        // read headers back out
        let mut signed_headers: BTreeMap<String, String> = temp_signing_request
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_string(),
                    v.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect();
        signed_headers.insert(HEADER_CONTENT_LENGTH_KEY.into(), body_len.to_string());
        Ok(signed_headers)
    }

    async fn aws_login_response(
        base_url: &str,
        http_client: &reqwest::Client,
        identity_id: &str,
        headers: &BTreeMap<String, String>,
        body: &str,
    ) -> Result<AwsLoginResponse, InfisicalError> {
        let iam_http_request_method = Method::POST.to_string();
        let iam_request_headers =
            BASE64.encode(serde_json::to_vec(&headers).map_err(InfisicalError::SerdeError)?);
        let iam_request_body = BASE64.encode(body.as_bytes());
        let params = AwsLoginRequest {
            identity_id: identity_id.into(),
            iam_http_request_method,
            iam_request_headers,
            iam_request_body,
        };
        let url = format!(
            "{}/{}",
            base_url.trim_end_matches("/"),
            INFISICAL_AWS_LOGIN_PATH.trim_start_matches("/")
        );
        http_client
            .post(url)
            .json(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<AwsLoginResponse>()
            .await
            .map_err(|_| InfisicalError::NotAuthenticated)
    }
}
