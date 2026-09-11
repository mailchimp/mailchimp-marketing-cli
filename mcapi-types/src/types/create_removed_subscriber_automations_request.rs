pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRemovedSubscriberAutomationsRequest {
    /// The list member's email address.
    #[serde(default)]
    pub email_address: String,
}

impl CreateRemovedSubscriberAutomationsRequest {
    pub fn builder() -> CreateRemovedSubscriberAutomationsRequestBuilder {
        <CreateRemovedSubscriberAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRemovedSubscriberAutomationsRequestBuilder {
    email_address: Option<String>,
}

impl CreateRemovedSubscriberAutomationsRequestBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRemovedSubscriberAutomationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](CreateRemovedSubscriberAutomationsRequestBuilder::email_address)
    pub fn build(self) -> Result<CreateRemovedSubscriberAutomationsRequest, BuildError> {
        Ok(CreateRemovedSubscriberAutomationsRequest {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
        })
    }
}

