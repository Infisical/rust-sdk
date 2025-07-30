/// Represents the parameters for a decrypt request.
///
/// Use the [DecryptRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct DecryptRequest {
    pub key_id: String,
    pub ciphertext: String,
}

impl DecryptRequest {
    /// Creates a new builder for a decrypt request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to use for decryption.
    /// * `ciphertext` - The ciphertext to decrypt.
    pub fn builder<KID, CT>(key_id: KID, ciphertext: CT) -> DecryptBuilder<KID, CT>
    where
        KID: Into<String>,
        CT: Into<String>,
    {
        DecryptBuilder::new(key_id, ciphertext)
    }
}

/// A builder for creating [DecryptRequest] instances.
#[derive(Debug)]
pub struct DecryptBuilder<KID, CT> {
    key_id: KID,
    ciphertext: CT,
}

impl<KID, CT> DecryptBuilder<KID, CT>
where
    KID: Into<String>,
    CT: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID, ciphertext: CT) -> Self {
        Self { key_id, ciphertext }
    }

    /// Builds the final [DecryptRequest].
    pub fn build(self) -> DecryptRequest {
        DecryptRequest {
            key_id: self.key_id.into(),
            ciphertext: self.ciphertext.into(),
        }
    }
} 