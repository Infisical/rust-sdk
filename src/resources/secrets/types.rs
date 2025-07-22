use serde::{Deserialize, Serialize};

/// Represents a secret from the Infisical API.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "workspace")]
    pub project_id: String,
    pub version: i32,
    /// The type of secret (`shared` or `personal`).
    #[serde(rename = "type")]
    pub secret_type: String,
    pub environment: String,
    pub secret_key: String,
    pub secret_value: String,
    pub secret_comment: String,
}
