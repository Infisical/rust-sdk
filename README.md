# infisical-rs — The official Infisical Rust SDK

The Infisical Rust SDK provides a convenient way to interact with Infisical programmatically.

### Installation
```bash
cargo add infisical
```

### Getting Started
```rust
async fn fetch_secret() {
    let client = ClientBuilder::new()
        .with_host_url("https://app.infisical.com")
        .with_request_timeout(Duration::from_secs(10))
        .build();

    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            panic!("Failed to build client: {:?}", e);
        }
    };

    let login_response = client
        .auth()
        .universal()
        .login(UniversalAuthLoginOptions {
            client_id: "<machine-identity-client-id>".to_string(),
            client_secret: "<machine-identity-client-secret>".to_string(),
        })
        .await;

    if let Err(e) = login_response {
        panic!("Failed to login: {:?}", e);
    }

    let secret = client
        .secrets()
        .get(GetSecretOptions {
            secret_name: "API_KEY".to_string(),
            path: Some("/".to_string()),
            environment: "dev".to_string(),
            project_id: "<your-project-id>".to_string(),
            expand_secret_references: None,
            r#type: None,
        })
        .await;

    let secret = match secret {
        Ok(secret) => secret,
        Err(e) => {
            panic!("Failed to get secret: {:?}", e);
        }
    };

    println!("Secret: {:?}", secret);
}
```

### Core Methods

The SDK methods are organized into the following high-level categories:

- `auth`: Handles authentication flows.
- `secrets`: Perform CRUD operations for secrets.


### `auth`

#### Universal Auth


**Example**
```rust
let res = client
    .auth()
    .universal()
    .login(UniversalAuthLoginOptions {
        client_id: "<machine-identity-client-id>".to_string(),
        client_secret: "<machine-identity-client-secret>".to_string(),
    })
    .await;
```

**Parameters**
* `options` (object):
  * `client_id` (string): The client ID of your Machine Identity.
  * `client_secret` (string): The client secret of your Machine Identity.



### `secrets`

#### Create Secret

Create a new secret in your project.

**Example**
```rust
let created_secret = client
    .secrets()
    .create(CreateSecretOptions {
        project_id: "<your-project-id>".to_string(),
        path: Some("/".to_string()),
        environment: "dev".to_string(),
        secret_name: "API_KEY".to_string(),
        secret_value: "your-secret-value".to_string(),
        secret_comment: None,
        skip_multiline_encoding: None,
        r#type: None,
    })
    .await?;
```

**Parameters**
* `options` (object):
  * `project_id` (string): The ID of your project.
  * `path` (Option<string>): The path where the secret should be stored (e.g., "/").
  * `environment` (string): The environment name (e.g., "dev", "prod").
  * `secret_name` (string): The name of the secret.
  * `secret_value` (string): The value of the secret.
  * `secret_comment` (Option<string>): Optional comment for the secret.
  * `skip_multiline_encoding` (Option<bool>): Whether to skip multiline encoding.
  * `r#type` (Option<string>): The type of the secret. `(shared|personal)`, defaults to `shared`.

#### Get Secret

Retrieve a specific secret by name.

**Example**
```rust
let secret = client
    .secrets()
    .get(GetSecretOptions {
        secret_name: "API_KEY".to_string(),
        path: Some("/".to_string()),
        environment: "dev".to_string(),
        project_id: "<your-project-id>".to_string(),
        expand_secret_references: None,
        r#type: None,
    })
    .await?;
```

**Parameters**
* `options` (object):
  * `secret_name` (string): The name of the secret to retrieve.
  * `path` (Option<string>): The path where the secret is stored.
  * `environment` (string): The environment name.
  * `project_id` (string): The ID of your project.
  * `expand_secret_references` (Option<bool>): Whether to expand secret references.
  * `r#type` (Option<string>): The type of the secret. `(shared|personal)`, defaults to `shared`.

#### List Secrets

List all secrets in a project environment.

**Example**
```rust
let secrets = client
    .secrets()
    .list(ListSecretsOptions {
        environment: "dev".to_string(),
        project_id: "<your-project-id>".to_string(),
        path: None,
        expand_secret_references: None,
        recursive: None,
        attach_to_process_env: None,
    })
    .await?;
```

**Parameters**
* `options` (object):
  * `environment` (string): The environment name.
  * `project_id` (string): The ID of your project.
  * `path` (Option<string>): The path to list secrets from.
  * `expand_secret_references` (Option<bool>): Whether to expand secret references.
  * `recursive` (Option<bool>): Whether to recursively list secrets from sub-folders.
  * `attach_to_process_env` (Option<bool>): Whether to attach secrets to the process environment.

#### Update Secret

Update an existing secret's name and/or value.

**Example**
```rust
let updated_secret = client
    .secrets()
    .update(UpdateSecretOptions {
        secret_name: "API_KEY".to_string(),
        environment: "dev".to_string(),
        project_id: "<your-project-id>".to_string(),
        new_secret_name: Some("NEW_API_KEY".to_string()),
        path: None,
        secret_value: "new-secret-value".to_string(),
        skip_multiline_encoding: None,
        r#type: None,
    })
    .await?;
```

**Parameters**
* `options` (object):
  * `secret_name` (string): The current name of the secret to update.
  * `environment` (string): The environment name.
  * `project_id` (string): The ID of your project.
  * `new_secret_name` (Option<string>): The new name for the secret (optional).
  * `path` (Option<string>): The path where the secret is stored.
  * `secret_value` (string): The new value for the secret.
  * `skip_multiline_encoding` (Option<bool>): Whether to skip multiline encoding.
  * `r#type` (Option<string>): The type of the secret. `(shared|personal)`, defaults to `shared`.

#### Delete Secret

Delete a secret from your project.

**Example**
```rust
let deleted_secret = client
    .secrets()
    .delete(DeleteSecretOptions {
        path: None,
        r#type: None,
        secret_name: "API_KEY".to_string(),
        environment: "dev".to_string(),
        project_id: "<your-project-id>".to_string(),
    })
    .await?;
```

**Parameters**
* `options` (object):
  * `path` (Option<string>): The path where the secret is stored.
  * `r#type` (Option<string>): The type of the secret. `(shared|personal)`, defaults to `shared`.
  * `secret_name` (string): The name of the secret to delete.
  * `environment` (string): The environment name.
  * `project_id` (string): The ID of your project.