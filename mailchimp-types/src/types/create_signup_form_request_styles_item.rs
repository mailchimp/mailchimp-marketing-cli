pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Collection of Element style for List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormRequestStylesItem {
    /// A collection of options for a selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CreateSignupFormRequestStylesItemOptionsItem>>,
    /// A string that identifies the element selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<CreateSignupFormRequestStylesItemSelector>,
}

impl CreateSignupFormRequestStylesItem {
    pub fn builder() -> CreateSignupFormRequestStylesItemBuilder {
        <CreateSignupFormRequestStylesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormRequestStylesItemBuilder {
    options: Option<Vec<CreateSignupFormRequestStylesItemOptionsItem>>,
    selector: Option<CreateSignupFormRequestStylesItemSelector>,
}

impl CreateSignupFormRequestStylesItemBuilder {
    pub fn options(mut self, value: Vec<CreateSignupFormRequestStylesItemOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn selector(mut self, value: CreateSignupFormRequestStylesItemSelector) -> Self {
        self.selector = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormRequestStylesItem`].
    pub fn build(self) -> Result<CreateSignupFormRequestStylesItem, BuildError> {
        Ok(CreateSignupFormRequestStylesItem {
            options: self.options,
            selector: self.selector,
        })
    }
}
