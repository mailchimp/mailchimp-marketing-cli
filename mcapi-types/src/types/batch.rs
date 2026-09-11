pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of a batch request
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Batch {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BatchLinksItem>>,
    /// The date and time when all operations in the batch request completed in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<BatchCompletedAt>,
    /// The number of completed operations that returned an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored_operations: Option<i64>,
    /// The number of completed operations. This includes operations that returned an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_operations: Option<i64>,
    /// A string that uniquely identifies this batch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL of the gzipped archive of the results of all the operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body_url: Option<String>,
    /// The status of the batch call. [Learn more](https://mailchimp.com/developer/marketing/guides/run-async-requests-batch-endpoint/#check-the-status-of-a-batch-operation) about the batch operation status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BatchStatus>,
    /// The date and time when the server received the batch request in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
    /// The total number of operations to complete as part of this batch request. For GET requests requiring pagination, each page counts as a separate operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_operations: Option<i64>,
}

impl Batch {
    pub fn builder() -> BatchBuilder {
        <BatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchBuilder {
    links: Option<Vec<BatchLinksItem>>,
    completed_at: Option<BatchCompletedAt>,
    errored_operations: Option<i64>,
    finished_operations: Option<i64>,
    id: Option<String>,
    response_body_url: Option<String>,
    status: Option<BatchStatus>,
    submitted_at: Option<DateTime<FixedOffset>>,
    total_operations: Option<i64>,
}

impl BatchBuilder {
    pub fn links(mut self, value: Vec<BatchLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn completed_at(mut self, value: BatchCompletedAt) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn errored_operations(mut self, value: i64) -> Self {
        self.errored_operations = Some(value);
        self
    }

    pub fn finished_operations(mut self, value: i64) -> Self {
        self.finished_operations = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn response_body_url(mut self, value: impl Into<String>) -> Self {
        self.response_body_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: BatchStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn total_operations(mut self, value: i64) -> Self {
        self.total_operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Batch`].
    pub fn build(self) -> Result<Batch, BuildError> {
        Ok(Batch {
            links: self.links,
            completed_at: self.completed_at,
            errored_operations: self.errored_operations,
            finished_operations: self.finished_operations,
            id: self.id,
            response_body_url: self.response_body_url,
            status: self.status,
            submitted_at: self.submitted_at,
            total_operations: self.total_operations,
        })
    }
}
