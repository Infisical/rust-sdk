/// Represents the parameters for a list secrets request.
///
/// Use the [ListSecretsRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct ListSecretsRequest {
    pub project_id: String,
    pub environment: String,
    pub path: Option<String>,
    pub expand_secret_references: Option<bool>,
    pub recursive: Option<bool>,
    pub attach_to_process_env: Option<bool>,
}

impl ListSecretsRequest {
    /// Creates a new builder for a list secrets request.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project the secrets belong to.
    /// * `environment` - The environment slug (e.g., "dev", "prod").
    pub fn builder<PID, E>(project_id: PID, environment: E) -> ListSecretsBuilder<PID, E>
    where
        PID: Into<String>,
        E: Into<String>,
    {
        ListSecretsBuilder::new(project_id, environment)
    }
}

/// A builder for creating [ListSecretsRequest] instances.
#[derive(Debug)]
pub struct ListSecretsBuilder<PID, E> {
    project_id: PID,
    environment: E,
    path: Option<String>,
    expand_secret_references: Option<bool>,
    recursive: Option<bool>,
    attach_to_process_env: Option<bool>,
}

impl<PID, E> ListSecretsBuilder<PID, E>
where
    PID: Into<String>,
    E: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(project_id: PID, environment: E) -> Self {
        Self {
            project_id,
            environment,
            path: None,
            expand_secret_references: None,
            recursive: None,
            attach_to_process_env: None,
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

    /// Sets whether to fetch recursively. Defaults to `false`.
    #[must_use]
    pub fn recursive(mut self, expand: bool) -> Self {
        self.recursive = Some(expand);
        self
    }

    /// Sets whether to attach to process env. Defaults to `false`.
    #[must_use]
    pub fn attach_to_process_env(mut self, expand: bool) -> Self {
        self.attach_to_process_env = Some(expand);
        self
    }

    /// Builds the final [ListSecretsRequest].
    pub fn build(self) -> ListSecretsRequest {
        ListSecretsRequest {
            project_id: self.project_id.into(),
            environment: self.environment.into(),
            path: self.path,
            expand_secret_references: self.expand_secret_references,
            recursive: self.recursive,
            attach_to_process_env: self.attach_to_process_env,
        }
    }
}
