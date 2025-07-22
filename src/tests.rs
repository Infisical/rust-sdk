// Ignored tests will load credentials from a .env file in the root of the project.
// Create a .env file with the following contents:
//
// INFISICAL_CLIENT_ID="your_client_id"
// INFISICAL_CLIENT_SECRET="your_client_secret"
// INFISICAL_PROJECT_ID="your_project_id"
//
// cargo test -- --ignored --nocapture

use crate::{
    resources::secrets::{
        CreateSecretRequest, DeleteSecretRequest, GetSecretRequest, ListSecretsRequest,
        UpdateSecretRequest,
    },
    AuthMethod, Client,
};
use dotenvy::dotenv;

#[tokio::test]
#[ignore = "This test requires a running Infisical instance and valid credentials"]
async fn test_get_secret() {
    dotenv().ok();

    let client_id = std::env::var("INFISICAL_CLIENT_ID").expect("INFISICAL_CLIENT_ID must be set");
    let client_secret =
        std::env::var("INFISICAL_CLIENT_SECRET").expect("INFISICAL_CLIENT_SECRET must be set");
    let project_id =
        std::env::var("INFISICAL_PROJECT_ID").expect("INFISICAL_PROJECT_ID must be set");

    let mut client = Client::builder()
        .base_url("http://localhost:8080")
        .build()
        .await
        .expect("Failed to build client");

    let auth_method = AuthMethod::new_universal_auth(client_id, client_secret);
    client.login(auth_method).await.unwrap();

    let request = GetSecretRequest::builder("FOO", project_id, "dev")
        .path("/")
        .build();

    let secret = client
        .secrets()
        .get(request)
        .await
        .expect("Failed to get secret");

    println!("Fetched secret: {secret:?}");

    // Verify the contents of the secret.
    assert_eq!(secret.secret_key, "FOO");
    assert_eq!(secret.secret_value, "BAR");
}

#[tokio::test]
#[ignore = "This test requires a running Infisical instance and valid credentials"]
async fn test_list_secrets() {
    dotenv().ok();

    let client_id = std::env::var("INFISICAL_CLIENT_ID").expect("INFISICAL_CLIENT_ID must be set");
    let client_secret =
        std::env::var("INFISICAL_CLIENT_SECRET").expect("INFISICAL_CLIENT_SECRET must be set");
    let project_id =
        std::env::var("INFISICAL_PROJECT_ID").expect("INFISICAL_PROJECT_ID must be set");

    let mut client = Client::builder()
        .base_url("http://localhost:8080")
        .build()
        .await
        .expect("Failed to build client");

    let auth_method = AuthMethod::new_universal_auth(client_id, client_secret);
    client.login(auth_method).await.unwrap();

    let request = ListSecretsRequest::builder(project_id, "dev")
        .attach_to_process_env(true)
        .path("/")
        .recursive(true)
        .build();

    let secrets = client
        .secrets()
        .list(request)
        .await
        .expect("Failed to list secrets");

    println!("Fetched secrets: {secrets:?}");

    assert!(!secrets.is_empty());
}

#[tokio::test]
#[ignore = "This test requires a running Infisical instance and valid credentials"]
async fn test_create_and_delete_secret() {
    dotenv().ok();

    let client_id = std::env::var("INFISICAL_CLIENT_ID").expect("INFISICAL_CLIENT_ID must be set");
    let client_secret =
        std::env::var("INFISICAL_CLIENT_SECRET").expect("INFISICAL_CLIENT_SECRET must be set");
    let project_id =
        std::env::var("INFISICAL_PROJECT_ID").expect("INFISICAL_PROJECT_ID must be set");

    let mut client = Client::builder()
        .base_url("http://localhost:8080")
        .build()
        .await
        .expect("Failed to build client");

    let auth_method = AuthMethod::new_universal_auth(client_id, client_secret);
    client.login(auth_method).await.unwrap();

    println!("Creating secret...");

    let request =
        CreateSecretRequest::builder("RUST_SDK_KEY", "RUST_SDK_SECRET", &project_id, "dev")
            .path("/")
            .secret_comment("Hello from rust!")
            .build();

    let secret = client
        .secrets()
        .create(request)
        .await
        .expect("Failed to create secret");

    println!("Created secret: {secret:?}");

    // Verify the contents of the secret.
    assert_eq!(secret.secret_key, "RUST_SDK_KEY");
    assert_eq!(secret.secret_value, "RUST_SDK_SECRET");
    assert_eq!(secret.secret_comment, "Hello from rust!");

    println!("Deleting secret...");

    let del_request =
        DeleteSecretRequest::builder(&secret.secret_key, secret.project_id, secret.environment)
            .path("/")
            .build();

    let del_secret = client
        .secrets()
        .delete(del_request)
        .await
        .expect("Failed to delete secret");

    println!("Deleted secret: {del_secret:?}");

    // Verify the contents of the secret.
    assert_eq!(del_secret.secret_key, secret.secret_key);
}

#[tokio::test]
#[ignore = "This test requires a running Infisical instance and valid credentials"]
async fn test_update_secret() {
    dotenv().ok();

    let client_id = std::env::var("INFISICAL_CLIENT_ID").expect("INFISICAL_CLIENT_ID must be set");
    let client_secret =
        std::env::var("INFISICAL_CLIENT_SECRET").expect("INFISICAL_CLIENT_SECRET must be set");
    let project_id =
        std::env::var("INFISICAL_PROJECT_ID").expect("INFISICAL_PROJECT_ID must be set");

    let mut client = Client::builder()
        .base_url("http://localhost:8080")
        .build()
        .await
        .expect("Failed to build client");

    let auth_method = AuthMethod::new_universal_auth(client_id, client_secret);
    client.login(auth_method).await.unwrap();

    let request = UpdateSecretRequest::builder("UPDATE_ME", &project_id, "dev")
        .path("/")
        .new_secret_name("UPDATED_NAME")
        .secret_value("UPDATED_VALUE")
        .secret_comment("UPDATED_COMMENT")
        .build();

    let secret = client
        .secrets()
        .update(request)
        .await
        .expect("Failed to update secret");

    println!("Updated secret: {secret:?}");

    // Verify the contents of the secret.
    assert_eq!(secret.secret_key, "UPDATED_NAME");
    assert_eq!(secret.secret_value, "UPDATED_VALUE");
    assert_eq!(secret.secret_comment, "UPDATED_COMMENT");

    // Revert the secret back
    let rev_request = UpdateSecretRequest::builder("UPDATED_NAME", &project_id, "dev")
        .path("/")
        .new_secret_name("UPDATE_ME")
        .secret_value("UPDATE_ME_SECRET")
        .secret_comment("UPDATE_ME_COMMENT")
        .build();

    let rev_secret = client
        .secrets()
        .update(rev_request)
        .await
        .expect("Failed to update secret");

    // Verify the contents of the secret.
    assert_eq!(rev_secret.secret_key, "UPDATE_ME");
    assert_eq!(rev_secret.secret_value, "UPDATE_ME_SECRET");
    assert_eq!(rev_secret.secret_comment, "UPDATE_ME_COMMENT");
}
