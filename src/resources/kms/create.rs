use crate::kms::{EncryptionAlgorithm, KeyUsage};

/// Represents the parameters for a create KMS key request.
///
/// Use the [CreateKmsKeyRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct CreateKmsKeyRequest {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub key_usage: Option<String>,
    pub encryption_algorithm: Option<String>,
}

impl CreateKmsKeyRequest {
    /// Creates a new builder for a create KMS key request.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to create the key in.
    /// * `name` - The name of the key to create.
    pub fn builder<PID, N>(project_id: PID, name: N) -> CreateKmsKeyBuilder<PID, N>
    where
        PID: Into<String>,
        N: Into<String>,
    {
        CreateKmsKeyBuilder::new(project_id, name)
    }
}

/// A builder for creating [CreateKmsKeyRequest] instances.
#[derive(Debug)]
pub struct CreateKmsKeyBuilder<PID, N> {
    project_id: PID,
    name: N,
    description: Option<String>,
    key_usage: Option<KeyUsage>,
    encryption_algorithm: Option<EncryptionAlgorithm>,
}

impl<PID, N> CreateKmsKeyBuilder<PID, N>
where
    PID: Into<String>,
    N: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(project_id: PID, name: N) -> Self {
        Self {
            project_id,
            name,
            description: None,
            key_usage: None,
            encryption_algorithm: None,
        }
    }

    /// Sets the key description.
    #[must_use]
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the key usage. Defaults to `"encrypt-decrypt"`.
    #[must_use]
    pub fn key_usage(mut self, key_usage: KeyUsage) -> Self {
        self.key_usage = Some(key_usage);
        self
    }

    /// Sets the encryption algorithm. Defaults to `"aes-256-gcm"`.
    #[must_use]
    pub fn encryption_algorithm(mut self, encryption_algorithm: EncryptionAlgorithm) -> Self {
        self.encryption_algorithm = Some(encryption_algorithm);
        self
    }

    /// Builds the final [CreateKmsKeyRequest].
    pub fn build(self) -> CreateKmsKeyRequest {
        CreateKmsKeyRequest {
            project_id: self.project_id.into(),
            name: self.name.into(),
            description: self.description,
            key_usage: self.key_usage.map(|e| e.to_string()),
            encryption_algorithm: self.encryption_algorithm.map(|e| e.to_string()),
        }
    }
}
