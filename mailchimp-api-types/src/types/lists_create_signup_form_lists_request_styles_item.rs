pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Collection of Element style for List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormListsRequestStylesItem {
    /// A collection of options for a selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CreateSignupFormListsRequestStylesItemOptionsItem>>,
    /// A string that identifies the element selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<CreateSignupFormListsRequestStylesItemSelector>,
}

impl CreateSignupFormListsRequestStylesItem {
    pub fn builder() -> CreateSignupFormListsRequestStylesItemBuilder {
        <CreateSignupFormListsRequestStylesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormListsRequestStylesItemBuilder {
    options: Option<Vec<CreateSignupFormListsRequestStylesItemOptionsItem>>,
    selector: Option<CreateSignupFormListsRequestStylesItemSelector>,
}

impl CreateSignupFormListsRequestStylesItemBuilder {
    pub fn options(mut self, value: Vec<CreateSignupFormListsRequestStylesItemOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn selector(mut self, value: CreateSignupFormListsRequestStylesItemSelector) -> Self {
        self.selector = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormListsRequestStylesItem`].
    pub fn build(self) -> Result<CreateSignupFormListsRequestStylesItem, BuildError> {
        Ok(CreateSignupFormListsRequestStylesItem {
            options: self.options,
            selector: self.selector,
        })
    }
}
