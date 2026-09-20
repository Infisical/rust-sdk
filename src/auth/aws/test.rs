use crate::auth::aws::auth_flow::{AwsAuthFlow, INFISICAL_AWS_LOGIN_PATH};
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::collections::BTreeMap;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const REGION: &str = "my-awesome-region";
const IDENTITY_ID: &str = "my-awesome-identity-id";
const TOKEN: &str = "my-awesome-token";

#[tokio::test]
async fn aws_auth_flow_signs_and_posts_to_infisical() {
    // 1. Stand up a fake Infisical
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(INFISICAL_AWS_LOGIN_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "accessToken": TOKEN,
            "expiresIn": 3600
        })))
        .expect(1)
        .mount(&mock)
        .await;

    // 2. Static AWS credentials + region
    let credentials = Credentials::for_tests();
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .credentials_provider(credentials)
        .load()
        .await;

    // 3. Run the real flow against the mock
    let http_client = reqwest::Client::new();
    let token = AwsAuthFlow::try_access_token_with_config(
        &http_client,
        IDENTITY_ID,
        &mock.uri(),
        &sdk_config,
    )
    .await
    .expect("flow should succeed");

    assert_eq!(token, TOKEN);

    // 4. Inspect what Infisical actually received.
    let received = mock.received_requests().await.unwrap();
    assert_eq!(received.len(), 1);
    let req = &received[0];

    let payload: serde_json::Value =
        serde_json::from_slice(&req.body).expect("body should be JSON");
    assert_eq!(payload["identityId"], IDENTITY_ID);
    assert_eq!(payload["iamHttpRequestMethod"], "POST");

    // 5. Decode iamRequestHeaders and verify SigV4 structure.
    let header_bytes = BASE64
        .decode(payload["iamRequestHeaders"].as_str().unwrap())
        .expect("headers should be valid Base64");
    let headers: BTreeMap<String, String> =
        serde_json::from_slice(&header_bytes).expect("headers should be JSON");
    let auth = headers
        .get("authorization")
        .expect("authorization header must be present");
    assert!(
        auth.starts_with("AWS4-HMAC-SHA256 "),
        "unexpected Authorization prefix: {auth}"
    );
    assert!(auth.contains("Credential=ANOTREAL/")); // This access key id comes from Credentials::for_tests()
    assert!(auth.contains(format!("/{REGION}/sts/aws4_request").as_str()));
    assert_eq!(headers.len(), 5);
    assert!(headers.contains_key("authorization"));
    assert!(headers.contains_key("content-length"));
    assert!(headers.contains_key("content-type"));
    assert!(headers.contains_key("x-amz-date"));
    assert!(headers.contains_key("x-amz-content-sha256"));
    let signed_headers = auth
        .split("SignedHeaders=")
        .nth(1)
        .and_then(|s| s.split(',').next())
        .expect("SignedHeaders must be present");
    let signed: Vec<&str> = signed_headers.split(';').collect();
    assert_eq!(
        signed,
        vec!["content-type", "host", "x-amz-content-sha256", "x-amz-date",],
        "SignedHeaders must be exactly the expected set in SigV4 sorted order"
    );

    // 6. Decode iamRequestBody and verify the STS parameters.
    let body_bytes = BASE64
        .decode(payload["iamRequestBody"].as_str().unwrap())
        .expect("body should be valid Base64");
    let body_str = std::str::from_utf8(&body_bytes).unwrap();
    assert!(body_str.contains("Action=GetCallerIdentity"));
    assert!(body_str.contains("Version=2011-06-15"));
}
