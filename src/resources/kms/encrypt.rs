/// Represents the parameters for an encrypt request.
///
/// Use the [EncryptRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct EncryptRequest {
    pub key_id: String,
    pub plaintext: String,
}

impl EncryptRequest {
    /// Creates a new builder for an encrypt request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to use for encryption.
    /// * `plaintext` - The plaintext to encrypt.
    pub fn builder<KID, PT>(key_id: KID, plaintext: PT) -> EncryptBuilder<KID, PT>
    where
        KID: Into<String>,
        PT: Into<String>,
    {
        EncryptBuilder::new(key_id, plaintext)
    }
}

/// A builder for creating [EncryptRequest] instances.
#[derive(Debug)]
pub struct EncryptBuilder<KID, PT> {
    key_id: KID,
    plaintext: PT,
}

impl<KID, PT> EncryptBuilder<KID, PT>
where
    KID: Into<String>,
    PT: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID, plaintext: PT) -> Self {
        Self { key_id, plaintext }
    }

    /// Builds the final [EncryptRequest].
    pub fn build(self) -> EncryptRequest {
        EncryptRequest {
            key_id: self.key_id.into(),
            plaintext: self.plaintext.into(),
        }
    }
} 