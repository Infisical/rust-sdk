use crate::{error::Result, Client, Secret};

pub async fn build_base_request(
    client: &mut Client,
    url: &str,
    method: reqwest::Method,
) -> Result<reqwest::RequestBuilder> {
    let base_request = client
        .http_client
        .request(method, url)
        // Setting JSON as the content type is OK since we only work with JSON.
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    Ok(base_request)
}

// It takes in a URL string, and a hashmap of query parameters.
pub fn build_url(url: String, query_params: &serde_json::Value) -> String {
    let mut url = url.to_string();

    if query_params.is_null() {
        return url;
    }

    let query_params = query_params.as_object().unwrap();

    if !query_params.is_empty() {
        url += "?";

        for (key, value) in query_params {
            // The value might be an option, so we need to make sure its not

            let val = value.as_str().unwrap_or("");

            if val.is_empty() {
                continue;
            }

            url.push_str(&format!("{key}={val}&"));
        }

        // Remove the last "&"
        url.pop();
    }

    url.to_string()
}

pub fn get_fallback_env_secret(key: &str) -> Option<Secret> {
    let fallback = std::env::var(key);

    let default_secret = Secret {
        is_fallback: true,
        version: 0,
        workspace: "".to_string(),
        secret_comment: "".to_string(),
        r#type: "".to_string(),
        environment: "".to_string(),
        secret_path: None,

        secret_key: key.to_string(),
        secret_value: "".to_string(),
    };

    match fallback {
        Ok(val) => Some(Secret {
            secret_value: val,
            ..default_secret
        }),
        Err(_) => None,
    }
}

pub fn set_env_vars(should_attach_envs: bool, secrets: &Vec<Secret>) {
    if !should_attach_envs {
        return;
    }

    for secret in secrets {
        // check if a env variable with the same name already exists, if it does, skip
        if std::env::var(&secret.secret_key).is_ok() {
            continue;
        }

        std::env::set_var(&secret.secret_key, &secret.secret_value);
    }
}

pub fn ensure_unique_secrets_by_key(secrets: &mut Vec<Secret>) {
    let mut secret_map = std::collections::HashMap::new();

    // Use the loop to overwrite the entry with the last secret of the same key
    for secret in std::mem::take(secrets) {
        secret_map.insert(secret.secret_key.clone(), secret);
    }

    // Clear the original vector and extend it with the unique secrets
    secrets.clear();
    secrets.extend(secret_map.into_values());
}
