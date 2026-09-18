pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSegmentMemberRequest {
    /// Email address for a subscriber.
    #[serde(default)]
    pub email_address: String,
}

impl CreateSegmentMemberRequest {
    pub fn builder() -> CreateSegmentMemberRequestBuilder {
        <CreateSegmentMemberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSegmentMemberRequestBuilder {
    email_address: Option<String>,
}

impl CreateSegmentMemberRequestBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSegmentMemberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](CreateSegmentMemberRequestBuilder::email_address)
    pub fn build(self) -> Result<CreateSegmentMemberRequest, BuildError> {
        Ok(CreateSegmentMemberRequest {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
        })
    }
}

