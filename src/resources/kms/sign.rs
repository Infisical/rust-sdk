use crate::kms::SigningAlgorithm;

/// Represents the parameters for a sign request.
///
/// Use the [SignRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct SignRequest {
    pub key_id: String,
    pub signing_algorithm: Option<String>,
    pub is_digest: Option<bool>,
    pub data: String,
}

impl SignRequest {
    /// Creates a new builder for a sign request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to use for signing.
    /// * `data` - The data to sign.
    pub fn builder<KID, D>(key_id: KID, data: D) -> SignBuilder<KID, D>
    where
        KID: Into<String>,
        D: Into<String>,
    {
        SignBuilder::new(key_id, data)
    }
}

/// A builder for creating [SignRequest] instances.
#[derive(Debug)]
pub struct SignBuilder<KID, D> {
    key_id: KID,
    data: D,
    signing_algorithm: Option<SigningAlgorithm>,
    is_digest: Option<bool>,
}

impl<KID, D> SignBuilder<KID, D>
where
    KID: Into<String>,
    D: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID, data: D) -> Self {
        Self {
            key_id,
            data,
            signing_algorithm: None,
            is_digest: None,
        }
    }

    /// Sets the signing algorithm. Defaults to `"RSASSA_PKCS1_V1_5_SHA_256"`.
    #[must_use]
    pub fn signing_algorithm(mut self, signing_algorithm: SigningAlgorithm) -> Self {
        self.signing_algorithm = Some(signing_algorithm);
        self
    }

    /// Sets whether the data is a digest. Defaults to `false`.
    #[must_use]
    pub fn is_digest(mut self, is_digest: bool) -> Self {
        self.is_digest = Some(is_digest);
        self
    }

    /// Builds the final [SignRequest].
    pub fn build(self) -> SignRequest {
        SignRequest {
            key_id: self.key_id.into(),
            data: self.data.into(),
            signing_algorithm: self.signing_algorithm.map(|e| e.to_string()),
            is_digest: self.is_digest,
        }
    }
}
