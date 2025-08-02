// Ignored tests will load credentials from a .env file in the root of the project.
// Create a .env file with the following contents:
//
// INFISICAL_CLIENT_ID="your_client_id"
// INFISICAL_CLIENT_SECRET="your_client_secret"
// INFISICAL_BASE_URL="your_hosted_url" - defaults to http://localhost:8080
//
// INFISICAL_SECRETS_PROJECT_ID="your_project_id"
// INFISICAL_KMS_PROJECT_ID="your_project_id"
//
// cargo test -- --ignored --nocapture

use crate::{
    decode_base64, encode_base64,
    kms::{
        CreateKmsKeyRequest, DecryptRequest, EncryptRequest, EncryptionAlgorithm, GetKmsKeyRequest,
        KeyUsage, ListKmsKeysRequest, SignRequest, SigningAlgorithm, UpdateKmsKeyRequest,
        VerifyRequest,
    },
    secrets::{
        CreateSecretRequest, DeleteSecretRequest, GetSecretRequest, ListSecretsRequest,
        UpdateSecretRequest,
    },
    AuthMethod, Client,
};
use dotenvy::dotenv;

async fn setup_client() -> Client {
    dotenv().ok();

    let client_id = std::env::var("INFISICAL_CLIENT_ID").expect("INFISICAL_CLIENT_ID must be set");
    let client_secret =
        std::env::var("INFISICAL_CLIENT_SECRET").expect("INFISICAL_CLIENT_SECRET must be set");
    let base_url =
        std::env::var("INFISICAL_BASE_URL").unwrap_or("http://localhost:8080".to_string());

    let mut client = Client::builder()
        .base_url(&base_url)
        .build()
        .await
        .expect("Failed to build client");

    let auth_method = AuthMethod::new_universal_auth(client_id, client_secret);
    client.login(auth_method).await.unwrap();

    client
}

#[tokio::test]
#[ignore = "This test requires a running Infisical instance and valid credentials"]
async fn test_secrets_resource() {
    let client = setup_client().await;
    let project_id = std::env::var("INFISICAL_SECRETS_PROJECT_ID")
        .expect("INFISICAL_SECRETS_PROJECT_ID must be set");
    let environment = "dev";
    let secret_path = "/";
    let secret_key = "RUST_SDK_KEY";
    let secret_value = "RUST_SDK_VALUE";
    let secret_comment = "A secret from the Rust SDK integration test";

    // 1. Create a new secret
    println!("Creating secret...");
    let create_request =
        CreateSecretRequest::builder(secret_key, secret_value, &project_id, environment)
            .path(secret_path)
            .secret_comment(secret_comment)
            .build();
    let created_secret = client
        .secrets()
        .create(create_request)
        .await
        .expect("Failed to create secret");

    assert_eq!(created_secret.secret_key, secret_key);
    assert_eq!(created_secret.secret_value, secret_value);
    assert_eq!(created_secret.secret_comment, secret_comment);
    println!("Created secret: {created_secret:?}");

    // 2. Get the secret
    println!("Getting secret...");
    let get_request = GetSecretRequest::builder(secret_key, &project_id, environment)
        .path(secret_path)
        .build();
    let fetched_secret = client
        .secrets()
        .get(get_request)
        .await
        .expect("Failed to get secret");

    assert_eq!(fetched_secret.secret_key, secret_key);
    assert_eq!(fetched_secret.secret_value, secret_value);
    println!("Fetched secret: {fetched_secret:?}");

    // 3. List secrets
    println!("Listing secrets...");
    let list_request = ListSecretsRequest::builder(&project_id, environment)
        .path(secret_path)
        .build();
    let secrets_list = client
        .secrets()
        .list(list_request)
        .await
        .expect("Failed to list secrets");

    assert!(secrets_list
        .iter()
        .any(|s| s.secret_key == created_secret.secret_key));
    println!("Secrets list contains the new secret.");

    // 4. Update the secret
    println!("Updating secret...");
    let updated_secret_value = "UPDATED_RUST_SDK_VALUE";
    let updated_secret_comment = "This secret was updated by the Rust SDK test";
    let update_request = UpdateSecretRequest::builder(secret_key, &project_id, environment)
        .path(secret_path)
        .secret_value(updated_secret_value)
        .secret_comment(updated_secret_comment)
        .build();
    let updated_secret = client
        .secrets()
        .update(update_request)
        .await
        .expect("Failed to update secret");

    assert_eq!(updated_secret.secret_key, secret_key);
    assert_eq!(updated_secret.secret_value, updated_secret_value);
    assert_eq!(updated_secret.secret_comment, updated_secret_comment);
    println!("Updated secret: {updated_secret:?}");

    // 5. Delete the secret
    println!("Deleting secret...");
    let delete_request = DeleteSecretRequest::builder(secret_key, &project_id, environment)
        .path(secret_path)
        .build();
    let deleted_secret = client
        .secrets()
        .delete(delete_request)
        .await
        .expect("Failed to delete secret");

    assert_eq!(deleted_secret.secret_key, secret_key);
    println!("Deleted secret: {deleted_secret:?}");

    // 6. Verify deletion by trying to get it again
    println!("Verifying deletion...");
    let get_after_delete_request = GetSecretRequest::builder(secret_key, &project_id, environment)
        .path(secret_path)
        .build();
    let get_result = client.secrets().get(get_after_delete_request).await;

    assert!(get_result.is_err());
    println!("Secret deletion verified. Test complete.");
}

#[tokio::test]
#[ignore = "This test requires a running Infisical instance and valid credentials"]
async fn test_kms_resource() {
    let client = setup_client().await;
    let project_id =
        std::env::var("INFISICAL_KMS_PROJECT_ID").expect("INFISICAL_KMS_PROJECT_ID must be set");

    // --- ENCRYPTION KEY VARS ---
    let enc_key_name = "rust-sdk-kms-encryption-key";
    let enc_key_description = "An encryption KMS key from the Rust SDK integration test";

    // --- SIGNING KEY VARS ---
    let sign_key_name = "rust-sdk-kms-signing-key";
    let sign_key_description = "A signing KMS key from the Rust SDK integration test";

    // 1. Create KMS keys
    println!("Creating encryption KMS key...");
    let create_enc_request = CreateKmsKeyRequest::builder(&project_id, enc_key_name)
        .description(enc_key_description)
        .key_usage(KeyUsage::EncryptDecrypt)
        .encryption_algorithm(EncryptionAlgorithm::Aes256Gcm)
        .build();
    let created_enc_key = client
        .kms()
        .create(create_enc_request)
        .await
        .expect("Failed to create encryption KMS key");

    assert_eq!(created_enc_key.name, enc_key_name);
    assert_eq!(created_enc_key.description, enc_key_description);
    println!("Created encryption KMS key: {created_enc_key:?}");

    println!("Creating signing KMS key...");
    let create_sign_request = CreateKmsKeyRequest::builder(&project_id, sign_key_name)
        .description(sign_key_description)
        .key_usage(KeyUsage::SignVerify)
        .encryption_algorithm(EncryptionAlgorithm::Rsa4096)
        .build();
    let created_sign_key = client
        .kms()
        .create(create_sign_request)
        .await
        .expect("Failed to create signing KMS key");

    assert_eq!(created_sign_key.name, sign_key_name);
    assert_eq!(created_sign_key.description, sign_key_description);
    println!("Created signing KMS key: {created_sign_key:?}");

    // 2. Get the KMS keys
    println!("Getting encryption KMS key...");
    let get_enc_request = GetKmsKeyRequest::builder(&created_enc_key.id).build();
    let fetched_enc_key = client
        .kms()
        .get(get_enc_request)
        .await
        .expect("Failed to get encryption KMS key");
    assert_eq!(fetched_enc_key.id, created_enc_key.id);
    println!("Fetched encryption KMS key: {fetched_enc_key:?}");

    println!("Getting signing KMS key...");
    let get_sign_request = GetKmsKeyRequest::builder(&created_sign_key.id).build();
    let fetched_sign_key = client
        .kms()
        .get(get_sign_request)
        .await
        .expect("Failed to get signing KMS key");
    assert_eq!(fetched_sign_key.id, created_sign_key.id);
    println!("Fetched signing KMS key: {fetched_sign_key:?}");

    // 3. List KMS keys
    println!("Listing KMS keys...");
    let list_request = ListKmsKeysRequest::builder(&project_id).build();
    let keys_list = client
        .kms()
        .list(list_request)
        .await
        .expect("Failed to list KMS keys");

    assert!(keys_list.iter().any(|k| k.id == created_enc_key.id));
    assert!(keys_list.iter().any(|k| k.id == created_sign_key.id));
    println!("KMS keys list contains the new keys.");

    // 4a. Encrypt and Decrypt with the encryption key
    println!("Encrypting and decrypting data...");
    let original_data = "some sensitive data";
    let encoded_data = encode_base64(original_data);

    let encrypt_request = EncryptRequest::builder(&created_enc_key.id, &encoded_data).build();
    let ciphertext = client
        .kms()
        .encrypt(encrypt_request)
        .await
        .expect("Failed to encrypt");

    let decrypt_request = DecryptRequest::builder(&created_enc_key.id, &ciphertext).build();
    let plaintext_b64 = client
        .kms()
        .decrypt(decrypt_request)
        .await
        .expect("Failed to decrypt");

    let decoded_plaintext = decode_base64(&plaintext_b64).expect("Failed to decode base64");
    assert_eq!(decoded_plaintext, original_data);
    println!("Successfully encrypted and decrypted data.");

    // 4b. Sign and Verify with the signing key
    println!("Signing and verifying data...");
    let data_to_sign = "some data to sign";
    let encoded_data_to_sign = encode_base64(data_to_sign);

    let sign_request = SignRequest::builder(&created_sign_key.id, &encoded_data_to_sign)
        .signing_algorithm(SigningAlgorithm::RsassaPkcs1V15Sha256)
        .is_digest(false)
        .build();
    let signature = client
        .kms()
        .sign(sign_request)
        .await
        .expect("Failed to sign data");

    let verify_request = VerifyRequest::builder(
        &created_sign_key.id,
        &encoded_data_to_sign,
        &signature.signature,
    )
    .signing_algorithm(SigningAlgorithm::RsassaPkcs1V15Sha256)
    .is_digest(false)
    .build();
    let verification_result = client
        .kms()
        .verify(verify_request)
        .await
        .expect("Failed to verify signature");

    assert!(verification_result.signature_valid);
    println!("Successfully signed and verified data.");

    // 5. Update the KMS key
    println!("Updating encryption KMS key...");
    let updated_key_name = "updated-rust-sdk-kms-enc-key";
    let updated_key_description = "This encryption key was updated by the Rust SDK test";
    let update_request = UpdateKmsKeyRequest::builder(&created_enc_key.id)
        .name(updated_key_name)
        .description(updated_key_description)
        .build();
    let updated_key = client
        .kms()
        .update(update_request)
        .await
        .expect("Failed to update KMS key");

    assert_eq!(updated_key.name, updated_key_name);
    assert_eq!(updated_key.description, updated_key_description);
    println!("Updated encryption KMS key: {updated_key:?}");

    // 6. Delete the KMS keys
    println!("Deleting encryption KMS key...");
    let delete_enc_request =
        crate::resources::kms::DeleteKmsKeyRequest::builder(&created_enc_key.id).build();
    let deleted_enc_key = client
        .kms()
        .delete(delete_enc_request)
        .await
        .expect("Failed to delete encryption KMS key");

    assert_eq!(deleted_enc_key.id, created_enc_key.id);
    println!("Deleted encryption KMS key: {deleted_enc_key:?}");

    println!("Deleting signing KMS key...");
    let delete_sign_request =
        crate::resources::kms::DeleteKmsKeyRequest::builder(&created_sign_key.id).build();
    let deleted_sign_key = client
        .kms()
        .delete(delete_sign_request)
        .await
        .expect("Failed to delete signing KMS key");

    assert_eq!(deleted_sign_key.id, created_sign_key.id);
    println!("Deleted signing KMS key: {deleted_sign_key:?}");

    // 7. Verify deletion
    println!("Verifying encryption key deletion...");
    let get_after_delete_enc_request = GetKmsKeyRequest::builder(&created_enc_key.id).build();
    let get_enc_result = client.kms().get(get_after_delete_enc_request).await;
    assert!(get_enc_result.is_err());
    println!("Encryption KMS key deletion verified.");

    println!("Verifying signing key deletion...");
    let get_after_delete_sign_request = GetKmsKeyRequest::builder(&created_sign_key.id).build();
    let get_sign_result = client.kms().get(get_after_delete_sign_request).await;
    assert!(get_sign_result.is_err());
    println!("Signing KMS key deletion verified.");

    println!("KMS resource test complete.");
}
