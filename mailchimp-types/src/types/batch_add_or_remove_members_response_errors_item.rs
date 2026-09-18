pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchAddOrRemoveMembersResponseErrorsItem {
    /// Email addresses added to the static segment or removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<String>>,
    /// The error message indicating why the email addresses could not be added or updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BatchAddOrRemoveMembersResponseErrorsItem {
    pub fn builder() -> BatchAddOrRemoveMembersResponseErrorsItemBuilder {
        <BatchAddOrRemoveMembersResponseErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchAddOrRemoveMembersResponseErrorsItemBuilder {
    email_addresses: Option<Vec<String>>,
    error: Option<String>,
}

impl BatchAddOrRemoveMembersResponseErrorsItemBuilder {
    pub fn email_addresses(mut self, value: Vec<String>) -> Self {
        self.email_addresses = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchAddOrRemoveMembersResponseErrorsItem`].
    pub fn build(self) -> Result<BatchAddOrRemoveMembersResponseErrorsItem, BuildError> {
        Ok(BatchAddOrRemoveMembersResponseErrorsItem {
            email_addresses: self.email_addresses,
            error: self.error,
        })
    }
}
