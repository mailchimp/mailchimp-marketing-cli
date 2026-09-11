pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Collection of Content for List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignupFormContentsItem {
    /// The content section name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<SignupFormContentsItemSection>,
    /// The content section text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SignupFormContentsItem {
    pub fn builder() -> SignupFormContentsItemBuilder {
        <SignupFormContentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignupFormContentsItemBuilder {
    section: Option<SignupFormContentsItemSection>,
    value: Option<String>,
}

impl SignupFormContentsItemBuilder {
    pub fn section(mut self, value: SignupFormContentsItemSection) -> Self {
        self.section = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignupFormContentsItem`].
    pub fn build(self) -> Result<SignupFormContentsItem, BuildError> {
        Ok(SignupFormContentsItem {
            section: self.section,
            value: self.value,
        })
    }
}
