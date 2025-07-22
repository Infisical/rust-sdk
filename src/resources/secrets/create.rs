/// Represents the parameters for a create secret request.
///
/// Use the [CreateSecretRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct CreateSecretRequest {
    pub secret_name: String,
    pub secret_value: String,
    pub project_id: String,
    pub environment: String,
    pub path: Option<String>,
    pub r#type: Option<String>,
    pub secret_comment: Option<String>,
    pub skip_multiline_encoding: Option<bool>,
}

impl CreateSecretRequest {
    /// Creates a new builder for a create secret request.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - The name of the secret to create.
    /// * `secret_value` - The value of the secret to create.
    /// * `project_id` - The ID of the project to insert the secret into.
    /// * `environment` - The environment slug (e.g., "dev", "prod").
    pub fn builder<SN, SV, PID, E>(
        secret_name: SN,
        secret_value: SV,
        project_id: PID,
        environment: E,
    ) -> CreateSecretBuilder<SN, SV, PID, E>
    where
        SN: Into<String>,
        SV: Into<String>,
        PID: Into<String>,
        E: Into<String>,
    {
        CreateSecretBuilder::new(secret_name, secret_value, project_id, environment)
    }
}

/// A builder for creating [CreateSecretRequest] instances.
#[derive(Debug)]
pub struct CreateSecretBuilder<SN, SV, PID, E> {
    secret_name: SN,
    secret_value: SV,
    project_id: PID,
    environment: E,
    path: Option<String>,
    r#type: Option<String>,
    secret_comment: Option<String>,
    skip_multiline_encoding: Option<bool>,
}

impl<SN, SV, PID, E> CreateSecretBuilder<SN, SV, PID, E>
where
    SN: Into<String>,
    SV: Into<String>,
    PID: Into<String>,
    E: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(secret_name: SN, secret_value: SV, project_id: PID, environment: E) -> Self {
        Self {
            secret_name,
            secret_value,
            project_id,
            environment,
            path: None,
            r#type: None,
            secret_comment: None,
            skip_multiline_encoding: None,
        }
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

    /// Builds the final [CreateSecretRequest].
    pub fn build(self) -> CreateSecretRequest {
        CreateSecretRequest {
            secret_name: self.secret_name.into(),
            secret_value: self.secret_value.into(),
            project_id: self.project_id.into(),
            environment: self.environment.into(),
            path: self.path,
            r#type: self.r#type,
            secret_comment: self.secret_comment,
            skip_multiline_encoding: self.skip_multiline_encoding,
        }
    }
}
