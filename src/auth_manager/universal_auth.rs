use crate::{AccessTokenSuccessResponse, Client, Error, Result, UniversalAuthLoginOptions};

pub struct UniversalAuth<'a> {
    client: &'a mut Client,
}

impl<'a> UniversalAuth<'a> {
    pub(crate) fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub async fn login(
        &mut self,
        request: UniversalAuthLoginOptions,
    ) -> Result<AccessTokenSuccessResponse> {
        let url = format!("{}/api/v1/auth/universal-auth/login", self.client.base_url);

        let response = self
            .client
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(Error::Reqwest)?;

        if !response.status().is_success() {
            return Err(Error::ResponseContent {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let login_response = response
            .json::<AccessTokenSuccessResponse>()
            .await
            .map_err(Error::Reqwest)?;

        let _ = self
            .client
            .update_http_client_auth_header(&login_response.access_token);

        Ok(login_response)
    }
}
