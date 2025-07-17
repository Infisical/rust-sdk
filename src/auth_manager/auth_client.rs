use crate::universal_auth::UniversalAuth;
use crate::Client;

pub struct AuthClient<'a> {
    client: &'a mut Client,
}

impl<'a> AuthClient<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub fn universal(&mut self) -> UniversalAuth {
        UniversalAuth::new(self.client)
    }
}
