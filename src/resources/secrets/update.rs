/// Represents the parameters for a update secret request.
///
/// Use the [UpdateSecretRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct UpdateSecretRequest {
    pub secret_name: String,
    pub project_id: String,
    pub environment: String,
    pub new_secret_name: Option<String>,
    pub secret_value: Option<String>,
    pub path: Option<String>,
    pub r#type: Option<String>,
    pub secret_comment: Option<String>,
    pub skip_multiline_encoding: Option<bool>,
}

impl UpdateSecretRequest {
    /// Creates a new builder for a update secret request.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - The name of the secret to update.
    /// * `project_id` - The ID of the project to insert the secret into.
    /// * `environment` - The environment slug (e.g., "dev", "prod").
    pub fn builder<SN, PID, E>(
        secret_name: SN,
        project_id: PID,
        environment: E,
    ) -> UpdateSecretBuilder<SN, PID, E>
    where
        SN: Into<String>,
        PID: Into<String>,
        E: Into<String>,
    {
        UpdateSecretBuilder::new(secret_name, project_id, environment)
    }
}

/// A builder for creating [UpdateSecretRequest] instances.
#[derive(Debug)]
pub struct UpdateSecretBuilder<SN, PID, E> {
    secret_name: SN,
    project_id: PID,
    environment: E,
    new_secret_name: Option<String>,
    secret_value: Option<String>,
    path: Option<String>,
    r#type: Option<String>,
    secret_comment: Option<String>,
    skip_multiline_encoding: Option<bool>,
}

impl<SN, PID, E> UpdateSecretBuilder<SN, PID, E>
where
    SN: Into<String>,
    PID: Into<String>,
    E: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(secret_name: SN, project_id: PID, environment: E) -> Self {
        Self {
            secret_name,
            project_id,
            environment,
            new_secret_name: None,
            secret_value: None,
            path: None,
            r#type: None,
            secret_comment: None,
            skip_multiline_encoding: None,
        }
    }

    /// Sets a new name for the secret.
    #[must_use]
    pub fn new_secret_name<S: Into<String>>(mut self, new_secret_name: S) -> Self {
        self.new_secret_name = Some(new_secret_name.into());
        self
    }

    /// Sets a new value for the secret.
    #[must_use]
    pub fn secret_value<S: Into<String>>(mut self, secret_value: S) -> Self {
        self.secret_value = Some(secret_value.into());
        self
    }

    /// Sets the secret's path. Defaults to `"/"`.
    #[must_use]
    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Sets the secret type (`shared` or `personal`). Defaults to `"shared"`.
    #[must_use]
    pub fn r#type<S: Into<String>>(mut self, r#type: S) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Sets the secret comment.
    #[must_use]
    pub fn secret_comment<S: Into<String>>(mut self, secret_comment: S) -> Self {
        self.secret_comment = Some(secret_comment.into());
        self
    }

    /// Sets whether to skip multiline encoding. Defaults to `false`.
    #[must_use]
    pub fn skip_multiline_encoding(mut self, expand: bool) -> Self {
        self.skip_multiline_encoding = Some(expand);
        self
    }

    /// Builds the final [UpdateSecretRequest].
    pub fn build(self) -> UpdateSecretRequest {
        UpdateSecretRequest {
            secret_name: self.secret_name.into(),
            project_id: self.project_id.into(),
            environment: self.environment.into(),
            new_secret_name: self.new_secret_name,
            secret_value: self.secret_value,
            path: self.path,
            r#type: self.r#type,
            secret_comment: self.secret_comment,
            skip_multiline_encoding: self.skip_multiline_encoding,
        }
    }
}
