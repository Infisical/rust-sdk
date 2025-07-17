use serde::{Deserialize, Serialize};

// This is a hack, because Serde can't parse boolean values by default...
fn default_as_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    pub version: i32,
    pub workspace: String,
    pub r#type: String,
    pub environment: String,
    pub secret_key: String,
    pub secret_value: String,
    pub secret_comment: String,

    pub secret_path: Option<String>,

    #[serde(default = "default_as_false")]
    pub is_fallback: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenSuccessResponse {
    pub access_token: String,
    pub expires_in: i64,
    #[serde(rename = "accessTokenMaxTTL")]
    pub access_token_max_ttl: i64,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalAuthLoginOptions {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecretOptions {
    pub environment: String,
    pub secret_comment: Option<String>,
    pub path: Option<String>,
    pub secret_value: String,
    pub skip_multiline_encoding: Option<bool>,
    pub r#type: Option<String>,
    pub project_id: String,
    pub secret_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecretResponse {
    pub secret: Secret,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSecretOptions {
    pub secret_name: String,
    pub environment: String,
    pub project_id: String,
    pub path: Option<String>,
    pub expand_secret_references: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSecretResponse {
    pub secret: Secret,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListSecretsOptions {
    pub environment: String,
    pub project_id: String,
    pub path: Option<String>,

    pub expand_secret_references: Option<bool>,
    pub recursive: Option<bool>,
    pub attach_to_process_env: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListSecretsResponse {
    pub secrets: Vec<Secret>,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSecretOptions {
    pub environment: String,    // environment
    pub path: Option<String>,   // secretPath
    pub r#type: Option<String>, // shared / personal
    pub secret_name: String,    // secretName (PASSED AS PARAMETER IN REQUEST)
    pub project_id: String,     // workspaceId
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSecretResponse {
    pub secret: Secret,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSecretOptions {
    pub secret_name: String,  // secretName (PASSED AS PARAMETER IN REQUEST)
    pub environment: String,  // environment
    pub path: Option<String>, // secretPath
    pub new_secret_name: Option<String>,
    pub secret_value: String,                  // secretValue
    pub skip_multiline_encoding: Option<bool>, // skipMultilineEncoding
    pub r#type: Option<String>,                // shared / personal
    pub project_id: String,                    // workspaceId
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSecretResponse {
    pub secret: Secret,
}
