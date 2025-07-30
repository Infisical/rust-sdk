/// Represents the parameters for a get KMS key request.
///
/// Use the [GetKmsKeyRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct GetKmsKeyRequest {
    pub key_id: String,
}

impl GetKmsKeyRequest {
    /// Creates a new builder for a get KMS key request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to retrieve.
    pub fn builder<KID>(key_id: KID) -> GetKmsKeyBuilder<KID>
    where
        KID: Into<String>,
    {
        GetKmsKeyBuilder::new(key_id)
    }
}

/// A builder for creating [GetKmsKeyRequest] instances.
#[derive(Debug)]
pub struct GetKmsKeyBuilder<KID> {
    key_id: KID,
}

impl<KID> GetKmsKeyBuilder<KID>
where
    KID: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID) -> Self {
        Self { key_id }
    }

    /// Builds the final [GetKmsKeyRequest].
    pub fn build(self) -> GetKmsKeyRequest {
        GetKmsKeyRequest {
            key_id: self.key_id.into(),
        }
    }
}

/// Represents the parameters for a get KMS key by name request.
///
/// Use the [GetKmsKeyByNameRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct GetKmsKeyByNameRequest {
    pub key_name: String,
}

impl GetKmsKeyByNameRequest {
    /// Creates a new builder for a get KMS key by name request.
    ///
    /// # Arguments
    ///
    /// * `key_name` - The name of the key to retrieve.
    pub fn builder<KN>(key_name: KN) -> GetKmsKeyByNameBuilder<KN>
    where
        KN: Into<String>,
    {
        GetKmsKeyByNameBuilder::new(key_name)
    }
}

/// A builder for creating [GetKmsKeyByNameRequest] instances.
#[derive(Debug)]
pub struct GetKmsKeyByNameBuilder<KN> {
    key_name: KN,
}

impl<KN> GetKmsKeyByNameBuilder<KN>
where
    KN: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_name: KN) -> Self {
        Self { key_name }
    }

    /// Builds the final [GetKmsKeyByNameRequest].
    pub fn build(self) -> GetKmsKeyByNameRequest {
        GetKmsKeyByNameRequest {
            key_name: self.key_name.into(),
        }
    }
} 