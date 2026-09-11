pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchSubscribeOrUnsubscribeListsResponseErrorsItem {
    /// The email address that could not be added or updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The error message indicating why the email address could not be added or updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// A unique code that identifies this specifc error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<BatchSubscribeOrUnsubscribeListsResponseErrorsItemErrorCode>,
    /// If the error is field-related, information about which field is at issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Message indicating how to resolve a field-related error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_message: Option<String>,
}

impl BatchSubscribeOrUnsubscribeListsResponseErrorsItem {
    pub fn builder() -> BatchSubscribeOrUnsubscribeListsResponseErrorsItemBuilder {
        <BatchSubscribeOrUnsubscribeListsResponseErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchSubscribeOrUnsubscribeListsResponseErrorsItemBuilder {
    email_address: Option<String>,
    error: Option<String>,
    error_code: Option<BatchSubscribeOrUnsubscribeListsResponseErrorsItemErrorCode>,
    field: Option<String>,
    field_message: Option<String>,
}

impl BatchSubscribeOrUnsubscribeListsResponseErrorsItemBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn error_code(mut self, value: BatchSubscribeOrUnsubscribeListsResponseErrorsItemErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn field_message(mut self, value: impl Into<String>) -> Self {
        self.field_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchSubscribeOrUnsubscribeListsResponseErrorsItem`].
    pub fn build(self) -> Result<BatchSubscribeOrUnsubscribeListsResponseErrorsItem, BuildError> {
        Ok(BatchSubscribeOrUnsubscribeListsResponseErrorsItem {
            email_address: self.email_address,
            error: self.error,
            error_code: self.error_code,
            field: self.field,
            field_message: self.field_message,
        })
    }
}
