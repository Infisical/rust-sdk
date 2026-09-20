use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AwsLoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[allow(unused)]
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
    #[allow(unused)]
    #[serde(rename = "accessTokenMaxTTL")]
    pub access_token_max_ttl: Option<u64>,
    #[allow(unused)]
    #[serde(rename = "tokenType")]
    pub token_type: Option<String>,
}
