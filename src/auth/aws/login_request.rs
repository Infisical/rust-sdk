use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AwsLoginRequest {
    pub identity_id: String,
    pub iam_http_request_method: String,
    pub iam_request_headers: String,
    pub iam_request_body: String,
}
