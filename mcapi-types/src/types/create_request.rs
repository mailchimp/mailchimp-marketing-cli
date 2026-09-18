pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRequest {
    /// The e-mail address at the domain you want to verify. This will receive a two-factor challenge to be used in the verify action.
    #[serde(default)]
    pub verification_email: String,
}

impl CreateRequest {
    pub fn builder() -> CreateRequestBuilder {
        <CreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestBuilder {
    verification_email: Option<String>,
}

impl CreateRequestBuilder {
    pub fn verification_email(mut self, value: impl Into<String>) -> Self {
        self.verification_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verification_email`](CreateRequestBuilder::verification_email)
    pub fn build(self) -> Result<CreateRequest, BuildError> {
        Ok(CreateRequest {
            verification_email: self.verification_email.ok_or_else(|| BuildError::missing_field("verification_email"))?,
        })
    }
}

