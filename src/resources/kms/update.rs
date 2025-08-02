/// Represents the parameters for an update KMS key request.
///
/// Use the [UpdateKmsKeyRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct UpdateKmsKeyRequest {
    pub key_id: String,
    pub name: Option<String>,
    pub is_disabled: Option<bool>,
    pub description: Option<String>,
}

impl UpdateKmsKeyRequest {
    /// Creates a new builder for an update KMS key request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to update.
    pub fn builder<KID>(key_id: KID) -> UpdateKmsKeyBuilder<KID>
    where
        KID: Into<String>,
    {
        UpdateKmsKeyBuilder::new(key_id)
    }
}

/// A builder for creating [UpdateKmsKeyRequest] instances.
#[derive(Debug)]
pub struct UpdateKmsKeyBuilder<KID> {
    key_id: KID,
    name: Option<String>,
    is_disabled: Option<bool>,
    description: Option<String>,
}

impl<KID> UpdateKmsKeyBuilder<KID>
where
    KID: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID) -> Self {
        Self {
            key_id,
            name: None,
            is_disabled: None,
            description: None,
        }
    }

    /// Sets the key name.
    #[must_use]
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets whether the key is disabled.
    #[must_use]
    pub fn is_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = Some(is_disabled);
        self
    }

    /// Sets the key description.
    #[must_use]
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Builds the final [UpdateKmsKeyRequest].
    pub fn build(self) -> UpdateKmsKeyRequest {
        UpdateKmsKeyRequest {
            key_id: self.key_id.into(),
            name: self.name,
            is_disabled: self.is_disabled,
            description: self.description,
        }
    }
}
