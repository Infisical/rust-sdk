# infisical — The official Infisical Rust SDK

The Infisical Rust SDK ([docs.rs](https://docs.rs/infisical)) provides a convenient and ergonomic way to interact with Infisical programmatically using modern and idiomatic Rust.

### Installation

```bash
cargo add infisical
```

### Getting Started

The easiest way to get started is to use the builder pattern for both the client and your requests.

```rust
use infisical::{Client, AuthMethod};
use infisical::resources::secrets::GetSecretRequest;
use std::error::Error;

async fn fetch_secret() -> Result<(), Box<dyn Error>> {
    // 1. Build the client. You can chain methods to configure it.
    let mut client = Client::builder()
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

### Core Methods

The SDK methods are organized into the following high-level categories:

- `Client::builder()`: The main entry point for creating a client.
- `client.login()`: Allows client to make authenticated requests to the API.
- `client.secrets()`: Provides access to all CRUD operations for secrets.

### `secrets`

All secret operations are accessed via `client.secrets()`. Each operation has a dedicated request builder.

#### Create Secret

Create a new secret in your project.

**Example**

```rust
use infisical::resources::secrets::CreateSecretRequest;

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
use infisical::resources::secrets::GetSecretRequest;

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
use infisical::resources::secrets::ListSecretsRequest;

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
use infisical::resources::secrets::UpdateSecretRequest;

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
use infisical::resources::secrets::DeleteSecretRequest;

let request = DeleteSecretRequest::builder("API_KEY", "<your-project-id>", "dev")
    .path("/")
    .build();

let deleted_secret = client.secrets().delete(request).await?;
```

**Parameters**

- `secret_name`, `project_id`, `environment`: Required parameters passed to the `builder()` function.
- `.path(path)`: Optional method to set the secret's path (defaults to `/`).
- `.r#type(type)`: Optional method to set the secret type (`shared` or `personal`), defaults to `shared`.
