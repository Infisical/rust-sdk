/// Represents the parameters for a list KMS keys request.
///
/// Use the [ListKmsKeysRequest::builder()] to construct this struct.
#[derive(Debug, Clone)]
pub struct ListKmsKeysRequest {
    pub project_id: String,
}

impl ListKmsKeysRequest {
    /// Creates a new builder for a list KMS keys request.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to list keys from.
    pub fn builder<PID>(project_id: PID) -> ListKmsKeysBuilder<PID>
    where
        PID: Into<String>,
    {
        ListKmsKeysBuilder::new(project_id)
    }
}

/// A builder for creating [ListKmsKeysRequest] instances.
#[derive(Debug)]
pub struct ListKmsKeysBuilder<PID> {
    project_id: PID,
}

impl<PID> ListKmsKeysBuilder<PID>
where
    PID: Into<String>,
{
    /// Creates a new builder with the required parameters.
    fn new(project_id: PID) -> Self {
        Self { project_id }
    }

    /// Builds the final [ListKmsKeysRequest].
    pub fn build(self) -> ListKmsKeysRequest {
        ListKmsKeysRequest {
            project_id: self.project_id.into(),
        }
    }
}
