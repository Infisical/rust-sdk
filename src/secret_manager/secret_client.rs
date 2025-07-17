use crate::delete_secret::delete_secret;
use crate::list_secrets::list_secrets;
use crate::update_secret::update_secret;
use crate::{create_secret::create_secret, get_secret::get_secret, Client, CreateSecretOptions};
use crate::{
    DeleteSecretOptions, GetSecretOptions, ListSecretsOptions, Result, Secret, UpdateSecretOptions,
};

pub struct SecretClient<'a> {
    client: &'a mut Client,
}

impl<'a> SecretClient<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub async fn create(&mut self, request: CreateSecretOptions) -> Result<Secret> {
        create_secret(self.client, request).await
    }

    pub async fn get(&mut self, request: GetSecretOptions) -> Result<Secret> {
        get_secret(self.client, request).await
    }

    pub async fn list(&mut self, request: ListSecretsOptions) -> Result<Vec<Secret>> {
        list_secrets(self.client, request).await
    }

    pub async fn delete(&mut self, request: DeleteSecretOptions) -> Result<Secret> {
        delete_secret(self.client, request).await
    }

    pub async fn update(&mut self, request: UpdateSecretOptions) -> Result<Secret> {
        update_secret(self.client, request).await
    }
}
