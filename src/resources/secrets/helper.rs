use crate::resources::secrets::Secret;

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
