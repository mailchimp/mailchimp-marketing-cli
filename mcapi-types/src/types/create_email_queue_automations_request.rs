pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEmailQueueAutomationsRequest {
    /// The list member's email address.
    #[serde(default)]
    pub email_address: String,
}

impl CreateEmailQueueAutomationsRequest {
    pub fn builder() -> CreateEmailQueueAutomationsRequestBuilder {
        <CreateEmailQueueAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailQueueAutomationsRequestBuilder {
    email_address: Option<String>,
}

impl CreateEmailQueueAutomationsRequestBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEmailQueueAutomationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](CreateEmailQueueAutomationsRequestBuilder::email_address)
    pub fn build(self) -> Result<CreateEmailQueueAutomationsRequest, BuildError> {
        Ok(CreateEmailQueueAutomationsRequest {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
        })
    }
}

