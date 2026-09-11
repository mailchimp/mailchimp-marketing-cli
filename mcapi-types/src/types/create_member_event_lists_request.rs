pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateMemberEventListsRequest {
    /// Events created with the is_syncing value set to `true` will not trigger automations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_syncing: Option<bool>,
    /// The name for this type of event ('purchased', 'visited', etc). Must be 2-30 characters in length
    #[serde(default)]
    pub name: String,
    /// The date and time the event occurred in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub occurred_at: Option<DateTime<FixedOffset>>,
    /// An optional list of properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}

impl CreateMemberEventListsRequest {
    pub fn builder() -> CreateMemberEventListsRequestBuilder {
        <CreateMemberEventListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberEventListsRequestBuilder {
    is_syncing: Option<bool>,
    name: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    properties: Option<HashMap<String, String>>,
}

impl CreateMemberEventListsRequestBuilder {
    pub fn is_syncing(mut self, value: bool) -> Self {
        self.is_syncing = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, String>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberEventListsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateMemberEventListsRequestBuilder::name)
    pub fn build(self) -> Result<CreateMemberEventListsRequest, BuildError> {
        Ok(CreateMemberEventListsRequest {
            is_syncing: self.is_syncing,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            occurred_at: self.occurred_at,
            properties: self.properties,
        })
    }
}

