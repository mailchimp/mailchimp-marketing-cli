pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Collection of Element style for List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignupFormStylesItem {
    /// A collection of options for a selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SignupFormStylesItemOptionsItem>>,
    /// A string that identifies the element selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<SignupFormStylesItemSelector>,
}

impl SignupFormStylesItem {
    pub fn builder() -> SignupFormStylesItemBuilder {
        <SignupFormStylesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignupFormStylesItemBuilder {
    options: Option<Vec<SignupFormStylesItemOptionsItem>>,
    selector: Option<SignupFormStylesItemSelector>,
}

impl SignupFormStylesItemBuilder {
    pub fn options(mut self, value: Vec<SignupFormStylesItemOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn selector(mut self, value: SignupFormStylesItemSelector) -> Self {
        self.selector = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignupFormStylesItem`].
    pub fn build(self) -> Result<SignupFormStylesItem, BuildError> {
        Ok(SignupFormStylesItem {
            options: self.options,
            selector: self.selector,
        })
    }
}
