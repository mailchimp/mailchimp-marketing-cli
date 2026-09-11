pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSegmentMemberListsRequest {
    /// Email address for a subscriber.
    #[serde(default)]
    pub email_address: String,
}

impl CreateSegmentMemberListsRequest {
    pub fn builder() -> CreateSegmentMemberListsRequestBuilder {
        <CreateSegmentMemberListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSegmentMemberListsRequestBuilder {
    email_address: Option<String>,
}

impl CreateSegmentMemberListsRequestBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSegmentMemberListsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](CreateSegmentMemberListsRequestBuilder::email_address)
    pub fn build(self) -> Result<CreateSegmentMemberListsRequest, BuildError> {
        Ok(CreateSegmentMemberListsRequest {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
        })
    }
}

