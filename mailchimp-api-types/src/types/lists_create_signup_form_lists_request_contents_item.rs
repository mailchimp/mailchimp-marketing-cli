pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Collection of Content for List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormListsRequestContentsItem {
    /// The content section name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<CreateSignupFormListsRequestContentsItemSection>,
    /// The content section text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateSignupFormListsRequestContentsItem {
    pub fn builder() -> CreateSignupFormListsRequestContentsItemBuilder {
        <CreateSignupFormListsRequestContentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormListsRequestContentsItemBuilder {
    section: Option<CreateSignupFormListsRequestContentsItemSection>,
    value: Option<String>,
}

impl CreateSignupFormListsRequestContentsItemBuilder {
    pub fn section(mut self, value: CreateSignupFormListsRequestContentsItemSection) -> Self {
        self.section = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormListsRequestContentsItem`].
    pub fn build(self) -> Result<CreateSignupFormListsRequestContentsItem, BuildError> {
        Ok(CreateSignupFormListsRequestContentsItem {
            section: self.section,
            value: self.value,
        })
    }
}
