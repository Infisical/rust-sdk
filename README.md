# infisical — The official Infisical Rust SDK

The Infisical Rust SDK ([docs.rs](https://docs.rs/infisical)) provides a convenient and ergonomic way to interact with Infisical programmatically using modern and idiomatic Rust.

### Installation

```bash
cargo add infisical
```

### Getting Started

The easiest way to get started is to use the builder pattern for both the client and your requests.

```rust
use infisical::{AuthMethod, Client, InfisicalError, encode_base64, decode_base64};
use infisical::secrets::GetSecretRequest;

async fn fetch_secret() -> Result<(), InfisicalError> {
    // 1. Build the client. You can chain methods to configure it.
    let mut client = Client::builder()
        .base_url("https://app.infisical.com") // Optional: defaults to https://app.infisical.com
        .build()
        .await?;

    // 2. Set up your authentication method and log in.
    let auth_method = AuthMethod::new_universal_auth("<your-client-id>", "<your-client-secret>");
    client.login(auth_method).await?;

    // 3. Build a request to get a secret.
    // Required parameters (name, project_id, environment) are passed to `builder()`.
    let request = GetSecretRequest::builder("API_KEY", "<your-project-id>", "dev")
        .path("/") // Optional parameters are set with builder methods.
        .build();

    // 4. Make the API call.
    let secret = client.secrets().get(request).await?;

    println!("Fetched secret key: {}", secret.secret_key);
    // For security, avoid printing the secret value in production code!
    // println!("Secret value: {}", secret.secret_value);

    Ok(())
}
```

### Client Configuration

The `Client::builder()` provides several configuration options:

```rust
let mut client = Client::builder()
    .base_url("https://app.infisical.com") // Optional: set custom Infisical instance URL
    .build()
    .await?;
```

**Parameters**
- `.base_url(url)`: Optional method to set the Infisical instance URL. Defaults to `https://app.infisical.com` for Infisical Cloud. Use `https://eu.infisical.com` for EU and `http://localhost:8080` for local development.

### Core Methods

The SDK methods are organized into the following high-level categories:

- `Client::builder()`: The main entry point for creating a client.
- `client.login()`: Allows client to make authenticated requests to the API.
- `client.secrets()`: Provides access to all CRUD operations for secrets.
- `client.kms()`: Provides access to all KMS (Key Management Service) operations.

### Helper Functions

The SDK provides utility functions for common operations:

```rust
use infisical::{encode_base64, decode_base64};

// Base64 encode a string
let encoded = encode_base64("sensitive data");
println!("Encoded: {}", encoded);

// Base64 decode a string
let decoded = decode_base64(&encoded)?;
println!("Decoded: {}", decoded);
```

**Available Functions**
- `encode_base64(data: &str) -> String`: Encodes a string as base64
- `decode_base64(data: &str) -> Result<String, InfisicalError>`: Decodes a base64 string

### `secrets`

All secret operations are accessed via `client.secrets()`. Each operation has a dedicated request builder.

#### Create Secret

Create a new secret in your project.

**Example**

```rust
use infisical::secrets::CreateSecretRequest;

let request = CreateSecretRequest::builder(
    "API_KEY",
    "your-secret-value",
    "<your-project-id>",
    "dev"
)
.path("/")
.secret_comment("A comment for the new secret")
.build();

let created_secret = client.secrets().create(request).await?;
```

**Parameters**

- `secret_name`, `secret_value`, `project_id`, `environment`: Required parameters passed to the `builder()` function.
- `.path(path)`: Optional method to set the secret's path (defaults to `/`).
- `.secret_comment(comment)`: Optional method to add a comment.
- `.skip_multiline_encoding(bool)`: Optional method to control multiline encoding (defaults to `false`).
- `.r#type(type)`: Optional method to set the secret type (`shared` or `personal`), defaults to `shared`.

#### Get Secret

Retrieve a specific secret by name.

**Example**

```rust
use infisical::secrets::GetSecretRequest;

let request = GetSecretRequest::builder("API_KEY", "<your-project-id>", "dev")
    .path("/")
    .build();

let secret = client.secrets().get(request).await?;
```

**Parameters**

- `secret_name`, `project_id`, `environment`: Required parameters passed to the `builder()` function.
- `.path(path)`: Optional method to set the secret's path (defaults to `/`).
- `.expand_secret_references(bool)`: Optional method to control secret reference expansion (defaults to `true`).
- `.r#type(type)`: Optional method to set the secret type (`shared` or `personal`), defaults to `shared`.

#### List Secrets

List all secrets in a project and environment.

**Example**

```rust
use infisical::secrets::ListSecretsRequest;

let request = ListSecretsRequest::builder("<your-project-id>", "dev")
    .path("/")
    .recursive(true)
    .build();

let secrets = client.secrets().list(request).await?;
```

**Parameters**

- `project_id`, `environment`: Required parameters passed to the `builder()` function.
- `.path(path)`: Optional method to set the path from which to list secrets (defaults to `/`).
- `.expand_secret_references(bool)`: Optional method to control secret reference expansion (defaults to `true`).
- `.recursive(bool)`: Optional method to recursively list secrets from sub-folders (defaults to `false`).
- `.attach_to_process_env(bool)`: Optional method to attach fetched secrets to the current process's environment variables (defaults to `false`).

#### Update Secret

Update an existing secret.

**Example**

```rust
use infisical::secrets::UpdateSecretRequest;

let request = UpdateSecretRequest::builder("API_KEY", "<your-project-id>", "dev")
    .secret_value("new-secret-value") // Set the new value
    .build();

let updated_secret = client.secrets().update(request).await?;
```

**Parameters**

- `secret_name`, `project_id`, `environment`: Required parameters passed to the `builder()` function.
- `.new_secret_name(name)`: Optional method to rename the secret.
- `.secret_value(value)`: Optional method to set a new value for the secret.
- `.path(path)`: Optional method to set the secret's path.
- `.secret_comment(comment)`: Optional method to add or change the comment.
- `.skip_multiline_encoding(bool)`: Optional method to control multiline encoding.
- `.r#type(type)`: Optional method to set the secret type (`shared` or `personal`).

#### Delete Secret

Delete a secret from your project.

**Example**

```rust
use infisical::secrets::DeleteSecretRequest;

let request = DeleteSecretRequest::builder("API_KEY", "<your-project-id>", "dev")
    .path("/")
    .build();

let deleted_secret = client.secrets().delete(request).await?;
```

**Parameters**

- `secret_name`, `project_id`, `environment`: Required parameters passed to the `builder()` function.
- `.path(path)`: Optional method to set the secret's path (defaults to `/`).
- `.r#type(type)`: Optional method to set the secret type (`shared` or `personal`), defaults to `shared`.

### `kms`

All KMS (Key Management Service) operations are accessed via `client.kms()`. Each operation has a dedicated request builder.

#### List KMS Keys

List all KMS keys in a project.

**Example**

```rust
use infisical::kms::ListKmsKeysRequest;

let request = ListKmsKeysRequest::builder("<your-project-id>").build();

let keys = client.kms().list(request).await?;
```

**Parameters**

- `project_id`: Required parameter passed to the `builder()` function.

#### Get KMS Key

Retrieve a specific KMS key by ID.

**Example**

```rust
use infisical::kms::GetKmsKeyRequest;

let request = GetKmsKeyRequest::builder("<key-id>").build();

let key = client.kms().get(request).await?;
```

**Parameters**

- `key_id`: Required parameter passed to the `builder()` function.

#### Get KMS Key by Name

Retrieve a specific KMS key by name.

**Example**

```rust
use infisical::kms::GetKmsKeyByNameRequest;

let request = GetKmsKeyByNameRequest::builder("<key-name>").build();

let key = client.kms().get_by_name(request).await?;
```

**Parameters**

- `key_name`: Required parameter passed to the `builder()` function.

#### Create KMS Key

Create a new KMS key in your project.

**Example**

```rust
use infisical::kms::{CreateKmsKeyRequest, EncryptionAlgorithm, KeyUsage};

let request = CreateKmsKeyRequest::builder("<your-project-id>", "my-key")
    .description("A key for encryption operations")
    .key_usage(KeyUsage::EncryptDecrypt)
    .encryption_algorithm(EncryptionAlgorithm::Aes256Gcm)
    .build();

let created_key = client.kms().create(request).await?;
```

**Parameters**

- `project_id`, `name`: Required parameters passed to the `builder()` function.
- `.description(description)`: Optional method to set the key description.
- `.key_usage(usage)`: Optional method to set the key usage using the `KeyUsage` enum (defaults to `KeyUsage::EncryptDecrypt`).
- `.encryption_algorithm(algorithm)`: Optional method to set the encryption algorithm using the `EncryptionAlgorithm` enum (defaults to `EncryptionAlgorithm::Aes256Gcm`).

#### Update KMS Key

Update an existing KMS key.

**Example**

```rust
use infisical::kms::UpdateKmsKeyRequest;

let request = UpdateKmsKeyRequest::builder("<key-id>")
    .name("updated-key-name")
    .description("Updated description")
    .is_disabled(false)
    .build();

let updated_key = client.kms().update(request).await?;
```

**Parameters**

- `key_id`: Required parameter passed to the `builder()` function.
- `.name(name)`: Optional method to rename the key.
- `.description(description)`: Optional method to update the key description.
- `.is_disabled(disabled)`: Optional method to enable or disable the key.

#### Delete KMS Key

Delete a KMS key from your project.

**Example**

```rust
use infisical::kms::DeleteKmsKeyRequest;

let request = DeleteKmsKeyRequest::builder("<key-id>").build();

let deleted_key = client.kms().delete(request).await?;
```

**Parameters**

- `key_id`: Required parameter passed to the `builder()` function.

#### Encrypt Data

Encrypt data using a KMS key.

**Example**

```rust
use infisical::kms::EncryptRequest;

let request = EncryptRequest::builder("<key-id>", "sensitive data").build();

let ciphertext = client.kms().encrypt(request).await?;
```

**Parameters**

- `key_id`, `plaintext`: Required parameters passed to the `builder()` function.

#### Decrypt Data

Decrypt data using a KMS key.

**Example**

```rust
use infisical::kms::DecryptRequest;

let request = DecryptRequest::builder("<key-id>", "encrypted-data").build();

let plaintext = client.kms().decrypt(request).await?;
```

**Parameters**

- `key_id`, `ciphertext`: Required parameters passed to the `builder()` function.

#### Sign Data

Sign data using a KMS key.

**Example**

```rust
use infisical::kms::{SigningAlgorithm, SignRequest};

let request = SignRequest::builder("<key-id>", "data to sign")
    .signing_algorithm(SigningAlgorithm::RsassaPkcs1V15Sha256)
    .is_digest(false)
    .build();

let signature = client.kms().sign(request).await?;
```

**Parameters**

- `key_id`, `data`: Required parameters passed to the `builder()` function.
- `.signing_algorithm(algorithm)`: Optional method to set the signing algorithm using the `SigningAlgorithm` enum (defaults to `SigningAlgorithm::RsassaPkcs1V15Sha256`).
- `.is_digest(is_digest)`: Optional method to indicate if the data is a digest (defaults to `false`).

#### Verify Signature

Verify a signature using a KMS key.

**Example**

```rust
use infisical::kms::{SigningAlgorithm, VerifyRequest};

let request = VerifyRequest::builder("<key-id>", "data to sign", "signature")
    .signing_algorithm(SigningAlgorithm::RsassaPkcs1V15Sha256)
    .is_digest(false)
    .build();

let verification = client.kms().verify(request).await?;
```

**Parameters**

- `key_id`, `data`, `signature`: Required parameters passed to the `builder()` function.
- `.signing_algorithm(algorithm)`: Optional method to set the signing algorithm using the `SigningAlgorithm` enum (defaults to `SigningAlgorithm::RsassaPkcs1V15Sha256`).
- `.is_digest(is_digest)`: Optional method to indicate if the data is a digest (defaults to `false`).

#### Get Public Key

Get the public key for a KMS key.

**Example**

```rust
let public_key = client.kms().get_public_key("<key-id>").await?;
```

**Parameters**

- `key_id`: The ID of the key to get the public key for.

#### Get Signing Algorithms

Get the available signing algorithms for a KMS key.

**Example**

```rust
let algorithms = client.kms().get_signing_algorithms("<key-id>").await?;
```

**Parameters**

- `key_id`: The ID of the key to get signing algorithms for.

## Development and Testing

### Environment Setup

For development and testing, you'll need to set up environment variables. Create a `.env` file in your project root:

```env
INFISICAL_CLIENT_ID=your_client_id_here
INFISICAL_CLIENT_SECRET=your_client_secret_here
INFISICAL_BASE_URL=http://localhost:8080  # Optional: for local development

# Project IDs for different resources
INFISICAL_SECRETS_PROJECT_ID=your_project_id_here
INFISICAL_KMS_PROJECT_ID=your_project_id_here
```

### Getting Credentials

To obtain the required credentials:

1. **Client ID and Secret**: Create a Universal Auth machine identity in your Infisical project settings
2. **Project ID**: Found in your project settings or URL when viewing a project in the Infisical dashboard

### Running Tests

Tests that require authentication are marked with `#[ignore]` and need valid credentials:

```bash
# Run ignored tests (requires .env file with valid credentials)
cargo test -- --ignored --nocapture

# Run a specific test
cargo test test_kms_resource -- --ignored --nocapture
```

**Note**: Integration tests require a running Infisical instance and valid authentication credentials.
