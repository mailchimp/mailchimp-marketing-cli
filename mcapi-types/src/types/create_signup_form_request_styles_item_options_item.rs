pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An option for Signup Form Styles.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormRequestStylesItemOptionsItem {
    /// A string that identifies the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// A string that identifies value of the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateSignupFormRequestStylesItemOptionsItem {
    pub fn builder() -> CreateSignupFormRequestStylesItemOptionsItemBuilder {
        <CreateSignupFormRequestStylesItemOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormRequestStylesItemOptionsItemBuilder {
    property: Option<String>,
    value: Option<String>,
}

impl CreateSignupFormRequestStylesItemOptionsItemBuilder {
    pub fn property(mut self, value: impl Into<String>) -> Self {
        self.property = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormRequestStylesItemOptionsItem`].
    pub fn build(self) -> Result<CreateSignupFormRequestStylesItemOptionsItem, BuildError> {
        Ok(CreateSignupFormRequestStylesItemOptionsItem {
            property: self.property,
            value: self.value,
        })
    }
}
