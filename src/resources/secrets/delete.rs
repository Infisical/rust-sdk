/// Represents the parameters for a delete secret request.
///
/// Use the [DeleteSecretRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct DeleteSecretRequest {
    pub secret_name: String,
    pub project_id: String,
    pub environment: String,
    pub path: Option<String>,
    pub r#type: Option<String>,
}

impl DeleteSecretRequest {
    /// Creates a new builder for a delete secret request.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - The name of the secret to delete.
    /// * `project_id` - The ID of the project the secret belongs to.
    /// * `environment` - The environment slug (e.g., "dev", "prod").
    pub fn builder<SN, PID, E>(
        secret_name: SN,
        project_id: PID,
        environment: E,
    ) -> DeleteSecretBuilder<SN, PID, E>
    where
        SN: Into<String>,
        PID: Into<String>,
        E: Into<String>,
    {
        DeleteSecretBuilder::new(secret_name, project_id, environment)
    }
}

/// A builder for creating [DeleteSecretRequest] instances.
#[derive(Debug)]
pub struct DeleteSecretBuilder<SN, PID, E> {
    secret_name: SN,
    project_id: PID,
    environment: E,
    path: Option<String>,
    r#type: Option<String>,
}

impl<SN, PID, E> DeleteSecretBuilder<SN, PID, E>
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
            r#type: None,
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

    /// Builds the final [DeleteSecretRequest].
    pub fn build(self) -> DeleteSecretRequest {
        DeleteSecretRequest {
            secret_name: self.secret_name.into(),
            project_id: self.project_id.into(),
            environment: self.environment.into(),
            path: self.path,
            r#type: self.r#type,
        }
    }
}
