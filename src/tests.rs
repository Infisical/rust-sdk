// Ignored tests will load credentials from a .env file in the root of the project.
// Create a .env file with the following contents:
//
// INFISICAL_CLIENT_ID="your_client_id"
// INFISICAL_CLIENT_SECRET="your_client_secret"
// INFISICAL_BASE_URL="your_hosted_url" - defaults to http://localhost:8080
//
// INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID="your_project_id"
// INFISICAL_KMS_PROJECT_ID="your_project_id"
//
// cargo test -- --ignored --nocapture

use crate::{
    secrets::{
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
    let project_id = std::env::var("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID")
        .expect("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID must be set");
    let base_url =
        std::env::var("INFISICAL_BASE_URL").unwrap_or("http://localhost:8080".to_string());

    let mut client = Client::builder()
        .base_url(&base_url)
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
    let project_id = std::env::var("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID")
        .expect("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID must be set");
    let base_url =
        std::env::var("INFISICAL_BASE_URL").unwrap_or("http://localhost:8080".to_string());

    let mut client = Client::builder()
        .base_url(&base_url)
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
    let project_id = std::env::var("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID")
        .expect("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID must be set");
    let base_url =
        std::env::var("INFISICAL_BASE_URL").unwrap_or("http://localhost:8080".to_string());

    let mut client = Client::builder()
        .base_url(&base_url)
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
    let project_id = std::env::var("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID")
        .expect("INFISICAL_SECRETS_MANAGEMENT_PROJECT_ID must be set");
    let base_url =
        std::env::var("INFISICAL_BASE_URL").unwrap_or("http://localhost:8080".to_string());

    let mut client = Client::builder()
        .base_url(&base_url)
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

#[cfg(test)]
mod tests {
    use crate::{
        decode_base64, encode_base64,
        kms::{
            CreateKmsKeyRequest, DecryptRequest, EncryptRequest, GetKmsKeyRequest,
            ListKmsKeysRequest, SignRequest, UpdateKmsKeyRequest, VerifyRequest,
        },
        AuthMethod, Client,
    };
    use dotenvy::dotenv;

    #[tokio::test]
    #[ignore = "This test requires a running Infisical instance and valid credentials"]
    async fn test_kms_operations() {
        // This test demonstrates the KMS API usage
        // Note: This test will not actually run without proper authentication
        // It's meant to show the API structure and usage patterns

        dotenv().ok();

        let client_id =
            std::env::var("INFISICAL_CLIENT_ID").expect("INFISICAL_CLIENT_ID must be set");
        let client_secret =
            std::env::var("INFISICAL_CLIENT_SECRET").expect("INFISICAL_CLIENT_SECRET must be set");
        let project_id = std::env::var("INFISICAL_KMS_PROJECT_ID")
            .expect("INFISICAL_KMS_PROJECT_ID must be set");
        let base_url =
            std::env::var("INFISICAL_BASE_URL").unwrap_or("http://localhost:8080".to_string());

        let mut client = Client::builder().base_url(&base_url).build().await.unwrap();

        let auth_method = AuthMethod::new_universal_auth(client_id, client_secret);
        client.login(auth_method).await.unwrap();

        // Example: List KMS keys
        let list_request = ListKmsKeysRequest::builder(&project_id).build();
        let keys = client.kms().list(list_request).await.unwrap();

        // Keys list can be empty or contain keys
        println!("Found {} KMS keys", keys.len());

        // Example: Create a new KMS key for encryption
        let create_request = CreateKmsKeyRequest::builder(&project_id, "test-encryption-key")
            .description("A test key for encryption")
            .key_usage("encrypt-decrypt")
            .encryption_algorithm("aes-256-gcm")
            .build();
        let key = client.kms().create(create_request).await.unwrap();

        assert!(!key.id.is_empty());
        println!("Created encryption key with ID: {}", key.id);

        // Example: Get a KMS key by ID
        let get_request = GetKmsKeyRequest::builder(&key.id).build();
        let retrieved_key = client.kms().get(get_request).await.unwrap();

        assert_eq!(retrieved_key.id, key.id);

        // Example: Update a KMS key
        let update_request = UpdateKmsKeyRequest::builder(&key.id)
            .name("updated-encryption-key-name")
            .description("Updated description")
            .is_disabled(false)
            .build();
        let updated_key = client.kms().update(update_request).await.unwrap();

        assert!(!updated_key.id.is_empty());

        let original_data = "sensitive data";
        let encoded_data = encode_base64(original_data);

        // Example: decode_base64
        let decoded_data = decode_base64(&encoded_data).unwrap();
        assert_eq!(decoded_data, original_data);

        // Example: Encrypt data
        let encrypt_request = EncryptRequest::builder(&key.id, &encoded_data).build();
        let ciphertext = client.kms().encrypt(encrypt_request).await.unwrap();

        // Example: Decrypt data
        let decrypt_request = DecryptRequest::builder(&key.id, &ciphertext).build();
        let plaintext = client.kms().decrypt(decrypt_request).await.unwrap();

        let decoded_plaintext = decode_base64(&plaintext).unwrap();
        assert_eq!(decoded_plaintext, original_data);

        // Create a signing key
        let create_signing_request = CreateKmsKeyRequest::builder(&project_id, "test-signing-key")
            .description("A test key for signing")
            .key_usage("sign-verify")
            .encryption_algorithm("RSA_4096")
            .build();
        let signing_key = client.kms().create(create_signing_request).await.unwrap();

        assert!(!signing_key.id.is_empty());
        println!("Created signing key with ID: {}", signing_key.id);

        // Example: Sign data
        let sign_request = SignRequest::builder(&signing_key.id, encode_base64("data to sign"))
            .signing_algorithm("RSASSA_PKCS1_V1_5_SHA_256")
            .is_digest(false)
            .build();
        let signature = client.kms().sign(sign_request).await.unwrap();

        // Example: Verify signature
        let verify_request = VerifyRequest::builder(
            &signing_key.id,
            encode_base64("data to sign"),
            &signature.signature,
        )
        .signing_algorithm("RSASSA_PKCS1_V1_5_SHA_256")
        .is_digest(false)
        .build();
        let verification = client.kms().verify(verify_request).await.unwrap();

        assert!(verification.signature_valid);

        // Example: Get public key
        let public_key = client.kms().get_public_key(&signing_key.id).await.unwrap();

        // Verify the public key
        assert!(!public_key.is_empty());

        // Example: Get signing algorithms
        let algorithms = client
            .kms()
            .get_signing_algorithms(&signing_key.id)
            .await
            .unwrap();

        assert!(!algorithms.is_empty());

        println!("KMS operations test completed successfully");

        println!("Now its time for some cleanups!");

        // Cleanup: Delete the created keys
        let delete_encrypt_request =
            crate::resources::kms::DeleteKmsKeyRequest::builder(&key.id).build();
        client.kms().delete(delete_encrypt_request).await.unwrap();
        println!("Deleted encryption key with ID: {}", key.id);

        let delete_signing_request =
            crate::resources::kms::DeleteKmsKeyRequest::builder(&signing_key.id).build();
        client.kms().delete(delete_signing_request).await.unwrap();
        println!("Deleted signing key with ID: {}", signing_key.id);
    }
}
