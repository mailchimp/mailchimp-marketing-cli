pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionVerifyRequest {
    /// The code that was sent to the email address provided when adding a new domain to verify.
    #[serde(default)]
    pub code: String,
}

impl CreateActionVerifyRequest {
    pub fn builder() -> CreateActionVerifyRequestBuilder {
        <CreateActionVerifyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionVerifyRequestBuilder {
    code: Option<String>,
}

impl CreateActionVerifyRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateActionVerifyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CreateActionVerifyRequestBuilder::code)
    pub fn build(self) -> Result<CreateActionVerifyRequest, BuildError> {
        Ok(CreateActionVerifyRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}

