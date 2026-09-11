pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionVerifyVerifiedDomainsRequest {
    /// The code that was sent to the email address provided when adding a new domain to verify.
    #[serde(default)]
    pub code: String,
}

impl CreateActionVerifyVerifiedDomainsRequest {
    pub fn builder() -> CreateActionVerifyVerifiedDomainsRequestBuilder {
        <CreateActionVerifyVerifiedDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionVerifyVerifiedDomainsRequestBuilder {
    code: Option<String>,
}

impl CreateActionVerifyVerifiedDomainsRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateActionVerifyVerifiedDomainsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CreateActionVerifyVerifiedDomainsRequestBuilder::code)
    pub fn build(self) -> Result<CreateActionVerifyVerifiedDomainsRequest, BuildError> {
        Ok(CreateActionVerifyVerifiedDomainsRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}

