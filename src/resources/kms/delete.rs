/// Represents the parameters for a delete KMS key request.
///
/// Use the [DeleteKmsKeyRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct DeleteKmsKeyRequest {
    pub key_id: String,
}

impl DeleteKmsKeyRequest {
    /// Creates a new builder for a delete KMS key request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to delete.
    pub fn builder<KID>(key_id: KID) -> DeleteKmsKeyBuilder<KID>
    where
        KID: Into<String>,
    {
        DeleteKmsKeyBuilder::new(key_id)
    }
}

/// A builder for creating [DeleteKmsKeyRequest] instances.
#[derive(Debug)]
pub struct DeleteKmsKeyBuilder<KID> {
    key_id: KID,
}

impl<KID> DeleteKmsKeyBuilder<KID>
where
    KID: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID) -> Self {
        Self { key_id }
    }

    /// Builds the final [DeleteKmsKeyRequest].
    pub fn build(self) -> DeleteKmsKeyRequest {
        DeleteKmsKeyRequest {
            key_id: self.key_id.into(),
        }
    }
}
