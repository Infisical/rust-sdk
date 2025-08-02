use crate::kms::SigningAlgorithm;

/// Represents the parameters for a verify request.
///
/// Use the [VerifyRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct VerifyRequest {
    pub key_id: String,
    pub is_digest: Option<bool>,
    pub data: String,
    pub signature: String,
    pub signing_algorithm: Option<String>,
}

impl VerifyRequest {
    /// Creates a new builder for a verify request.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to use for verification.
    /// * `data` - The data that was signed.
    /// * `signature` - The signature to verify.
    pub fn builder<KID, D, S>(key_id: KID, data: D, signature: S) -> VerifyBuilder<KID, D, S>
    where
        KID: Into<String>,
        D: Into<String>,
        S: Into<String>,
    {
        VerifyBuilder::new(key_id, data, signature)
    }
}

/// A builder for creating [VerifyRequest] instances.
#[derive(Debug)]
pub struct VerifyBuilder<KID, D, S> {
    key_id: KID,
    data: D,
    signature: S,
    is_digest: Option<bool>,
    signing_algorithm: Option<SigningAlgorithm>,
}

impl<KID, D, S> VerifyBuilder<KID, D, S>
where
    KID: Into<String>,
    D: Into<String>,
    S: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(key_id: KID, data: D, signature: S) -> Self {
        Self {
            key_id,
            data,
            signature,
            is_digest: None,
            signing_algorithm: None,
        }
    }

    /// Sets whether the data is a digest. Defaults to `false`.
    #[must_use]
    pub fn is_digest(mut self, is_digest: bool) -> Self {
        self.is_digest = Some(is_digest);
        self
    }

    /// Sets the signing algorithm. Defaults to `"RSASSA_PKCS1_V1_5_SHA_256"`.
    #[must_use]
    pub fn signing_algorithm(mut self, signing_algorithm: SigningAlgorithm) -> Self {
        self.signing_algorithm = Some(signing_algorithm);
        self
    }

    /// Builds the final [VerifyRequest].
    pub fn build(self) -> VerifyRequest {
        VerifyRequest {
            key_id: self.key_id.into(),
            data: self.data.into(),
            signature: self.signature.into(),
            is_digest: self.is_digest,
            signing_algorithm: self.signing_algorithm.map(|e| e.to_string()),
        }
    }
}
