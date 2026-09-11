pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAccountExportsRequest {
    /// The stages of an account export to include.
    #[serde(default)]
    pub include_stages: Vec<CreateAccountExportsRequestIncludeStagesItem>,
    /// An ISO 8601 date that will limit the export to only records created after a given time. For instance, the reports stage will contain any campaign sent after the given timestamp. Audiences, however, are excluded from this limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_timestamp: Option<DateTime<FixedOffset>>,
}

impl CreateAccountExportsRequest {
    pub fn builder() -> CreateAccountExportsRequestBuilder {
        <CreateAccountExportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAccountExportsRequestBuilder {
    include_stages: Option<Vec<CreateAccountExportsRequestIncludeStagesItem>>,
    since_timestamp: Option<DateTime<FixedOffset>>,
}

impl CreateAccountExportsRequestBuilder {
    pub fn include_stages(mut self, value: Vec<CreateAccountExportsRequestIncludeStagesItem>) -> Self {
        self.include_stages = Some(value);
        self
    }

    pub fn since_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAccountExportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`include_stages`](CreateAccountExportsRequestBuilder::include_stages)
    pub fn build(self) -> Result<CreateAccountExportsRequest, BuildError> {
        Ok(CreateAccountExportsRequest {
            include_stages: self.include_stages.ok_or_else(|| BuildError::missing_field("include_stages"))?,
            since_timestamp: self.since_timestamp,
        })
    }
}

