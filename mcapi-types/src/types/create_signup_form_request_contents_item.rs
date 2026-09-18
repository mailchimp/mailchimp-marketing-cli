pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Collection of Content for List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormRequestContentsItem {
    /// The content section name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<CreateSignupFormRequestContentsItemSection>,
    /// The content section text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateSignupFormRequestContentsItem {
    pub fn builder() -> CreateSignupFormRequestContentsItemBuilder {
        <CreateSignupFormRequestContentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormRequestContentsItemBuilder {
    section: Option<CreateSignupFormRequestContentsItemSection>,
    value: Option<String>,
}

impl CreateSignupFormRequestContentsItemBuilder {
    pub fn section(mut self, value: CreateSignupFormRequestContentsItemSection) -> Self {
        self.section = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormRequestContentsItem`].
    pub fn build(self) -> Result<CreateSignupFormRequestContentsItem, BuildError> {
        Ok(CreateSignupFormRequestContentsItem {
            section: self.section,
            value: self.value,
        })
    }
}
