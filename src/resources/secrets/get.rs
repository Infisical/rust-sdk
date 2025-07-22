/// Represents the parameters for a get secret request.
///
/// Use the [GetSecretRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct GetSecretRequest {
    pub secret_name: String,
    pub project_id: String,
    pub environment: String,
    pub path: Option<String>,
    pub expand_secret_references: Option<bool>,
    pub r#type: Option<String>,
}

impl GetSecretRequest {
    /// Creates a new builder for a get secret request.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - The name of the secret to retrieve.
    /// * `project_id` - The ID of the project the secret belongs to.
    /// * `environment` - The environment slug (e.g., "dev", "prod").
    pub fn builder<SN, PID, E>(
        secret_name: SN,
        project_id: PID,
        environment: E,
    ) -> GetSecretBuilder<SN, PID, E>
    where
        SN: Into<String>,
        PID: Into<String>,
        E: Into<String>,
    {
        GetSecretBuilder::new(secret_name, project_id, environment)
    }
}

/// A builder for creating [GetSecretRequest] instances.
#[derive(Debug)]
pub struct GetSecretBuilder<SN, PID, E> {
    secret_name: SN,
    project_id: PID,
    environment: E,
    path: Option<String>,
    expand_secret_references: Option<bool>,
    r#type: Option<String>,
}

impl<SN, PID, E> GetSecretBuilder<SN, PID, E>
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
            path: None,
            expand_secret_references: None,
            r#type: None,
        }
    }

    /// Sets the secret's path. Defaults to `"/"`.
    #[must_use]
    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Sets whether to expand secret references. Defaults to `true`.
    #[must_use]
    pub fn expand_secret_references(mut self, expand: bool) -> Self {
        self.expand_secret_references = Some(expand);
        self
    }

    /// Sets the secret type (`shared` or `personal`). Defaults to `"shared"`.
    #[must_use]
    pub fn r#type<S: Into<String>>(mut self, r#type: S) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Builds the final [GetSecretRequest].
    pub fn build(self) -> GetSecretRequest {
        GetSecretRequest {
            secret_name: self.secret_name.into(),
            project_id: self.project_id.into(),
            environment: self.environment.into(),
            path: self.path,
            expand_secret_references: self.expand_secret_references,
            r#type: self.r#type,
        }
    }
}
