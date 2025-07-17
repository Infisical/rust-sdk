#![allow(deprecated)]

use std::time::Duration;

use infisical_rs::{
    ClientBuilder, CreateSecretOptions, DeleteSecretOptions, GetSecretOptions, ListSecretsOptions,
    UniversalAuthLoginOptions, UpdateSecretOptions,
};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    pub ua_client_id: String,
    pub ua_client_secret: String,
    pub project_id: String,
    pub environment: String,
    pub host_url: String,
}

fn get_env_vars() -> EnvironmentVariables {
    EnvironmentVariables {
        ua_client_id: std::env::var("UA_CLIENT_ID")
            .unwrap_or_else(|_| panic!("UA_CLIENT_ID is not set")),
        ua_client_secret: std::env::var("UA_CLIENT_SECRET")
            .unwrap_or_else(|_| panic!("UA_CLIENT_SECRET is not set")),
        project_id: std::env::var("PROJECT_ID").unwrap_or_else(|_| panic!("PROJECT_ID is not set")),
        environment: std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| panic!("ENVIRONMENT is not set")),
        host_url: std::env::var("HOST_URL").unwrap_or_else(|_| panic!("HOST_URL is not set")),
    }
}

fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[tokio::test]
async fn e2e_test() {
    let env_vars = get_env_vars();

    let client = ClientBuilder::new()
        .with_host_url(env_vars.host_url)
        .with_request_timeout(Duration::from_secs(10))
        .build();

    if client.is_err() {
        panic!("Failed to build client: {:?}", client.err());
    }

    let mut client = client.unwrap();

    let login_response = client
        .auth()
        .universal()
        .login(UniversalAuthLoginOptions {
            client_id: env_vars.ua_client_id.clone(),
            client_secret: env_vars.ua_client_secret.clone(),
        })
        .await;

    if login_response.is_err() {
        panic!("Failed to login: {:?}", login_response.err());
    }

    let secret_name = random_string(10);
    let secret_value = random_string(10);

    let updated_secret_name = random_string(10);
    let updated_secret_value = random_string(10);

    let created_secret = client
        .secrets()
        .create(CreateSecretOptions {
            project_id: env_vars.project_id.clone(),
            path: Some("/".to_string()),
            environment: env_vars.environment.clone(),
            secret_name: secret_name.clone(),
            secret_value: secret_value.clone(),
            secret_comment: None,
            skip_multiline_encoding: None,
            r#type: None,
        })
        .await;

    println!("Create Secret: OK");

    if created_secret.is_err() {
        panic!("Failed to create secret: {:?}", created_secret.err());
    }

    let get_secret_res = client
        .secrets()
        .get(GetSecretOptions {
            secret_name: secret_name.clone(),
            path: None,
            expand_secret_references: None,
            r#type: None,
            environment: env_vars.environment.clone(),
            project_id: env_vars.project_id.clone(),
        })
        .await;

    if get_secret_res.is_err() {
        panic!("Failed to get secret: {:?}", get_secret_res.err());
    }

    let get_secret_res = get_secret_res.unwrap();

    assert_eq!(get_secret_res.secret_key, secret_name);
    assert_eq!(get_secret_res.secret_value, secret_value);

    println!("Get Secret: OK");

    let list_secrets_res = client
        .secrets()
        .list(ListSecretsOptions {
            environment: env_vars.environment.clone(),
            project_id: env_vars.project_id.clone(),
            path: None,
            expand_secret_references: None,
            recursive: None,
            attach_to_process_env: None,
        })
        .await;

    if list_secrets_res.is_err() {
        panic!("Failed to list secrets: {:?}", list_secrets_res.err());
    }

    let list_secrets_res = list_secrets_res.unwrap();

    assert!(list_secrets_res.iter().any(|s| s.secret_key == secret_name));
    assert_eq!(list_secrets_res.len(), 1);

    println!("List Secrets: OK");

    let update_secret_res = client
        .secrets()
        .update(UpdateSecretOptions {
            secret_name: secret_name.clone(),
            environment: env_vars.environment.clone(),
            project_id: env_vars.project_id.clone(),
            new_secret_name: Some(updated_secret_name.clone()),
            path: None,
            secret_value: updated_secret_value.clone(),
            skip_multiline_encoding: None,
            r#type: None,
        })
        .await;

    println!("Update Secret: OK");

    if update_secret_res.is_err() {
        panic!("Failed to update secret: {:?}", update_secret_res.err());
    }

    let update_secret_res = update_secret_res.unwrap();

    assert_eq!(update_secret_res.secret_key, updated_secret_name);
    assert_eq!(update_secret_res.secret_value, updated_secret_value);

    let get_secret_res = client
        .secrets()
        .get(GetSecretOptions {
            secret_name: updated_secret_name.clone(),
            environment: env_vars.environment.clone(),
            project_id: env_vars.project_id.clone(),
            path: None,
            expand_secret_references: None,
            r#type: None,
        })
        .await;

    if get_secret_res.is_err() {
        panic!("Failed to get secret: {:?}", get_secret_res.err());
    }

    let get_secret_res = get_secret_res.unwrap();

    assert_eq!(get_secret_res.secret_key, updated_secret_name);
    assert_eq!(get_secret_res.secret_value, updated_secret_value);

    println!("Get Secret: OK");

    let delete_secret_res = client
        .secrets()
        .delete(DeleteSecretOptions {
            path: None,
            r#type: None,
            secret_name: updated_secret_name.clone(),
            environment: env_vars.environment.clone(),
            project_id: env_vars.project_id.clone(),
        })
        .await;

    if delete_secret_res.is_err() {
        panic!("Failed to delete secret: {:?}", delete_secret_res.err());
    }

    let delete_secret_res = delete_secret_res.unwrap();

    assert_eq!(delete_secret_res.secret_key, updated_secret_name);

    println!("Delete Secret: OK");

    let list_secrets_res = client
        .secrets()
        .list(ListSecretsOptions {
            environment: env_vars.environment.clone(),
            project_id: env_vars.project_id.clone(),
            path: None,
            expand_secret_references: None,
            recursive: None,
            attach_to_process_env: None,
        })
        .await;

    if list_secrets_res.is_err() {
        panic!("Failed to list secrets: {:?}", list_secrets_res.err());
    }

    let list_secrets_res = list_secrets_res.unwrap();

    assert_eq!(list_secrets_res.len(), 0);

    println!("List Secrets: OK");
}
