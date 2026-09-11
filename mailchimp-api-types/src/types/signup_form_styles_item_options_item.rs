pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An option for Signup Form Styles.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignupFormStylesItemOptionsItem {
    /// A string that identifies the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// A string that identifies value of the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SignupFormStylesItemOptionsItem {
    pub fn builder() -> SignupFormStylesItemOptionsItemBuilder {
        <SignupFormStylesItemOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignupFormStylesItemOptionsItemBuilder {
    property: Option<String>,
    value: Option<String>,
}

impl SignupFormStylesItemOptionsItemBuilder {
    pub fn property(mut self, value: impl Into<String>) -> Self {
        self.property = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignupFormStylesItemOptionsItem`].
    pub fn build(self) -> Result<SignupFormStylesItemOptionsItem, BuildError> {
        Ok(SignupFormStylesItemOptionsItem {
            property: self.property,
            value: self.value,
        })
    }
}
