pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormListsRequest {
    /// The signup form body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<CreateSignupFormListsRequestContentsItem>>,
    /// Options for customizing your signup form header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<CreateSignupFormListsRequestHeader>,
    /// An array of objects, each representing an element style for the signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<CreateSignupFormListsRequestStylesItem>>,
}

impl CreateSignupFormListsRequest {
    pub fn builder() -> CreateSignupFormListsRequestBuilder {
        <CreateSignupFormListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormListsRequestBuilder {
    contents: Option<Vec<CreateSignupFormListsRequestContentsItem>>,
    header: Option<CreateSignupFormListsRequestHeader>,
    styles: Option<Vec<CreateSignupFormListsRequestStylesItem>>,
}

impl CreateSignupFormListsRequestBuilder {
    pub fn contents(mut self, value: Vec<CreateSignupFormListsRequestContentsItem>) -> Self {
        self.contents = Some(value);
        self
    }

    pub fn header(mut self, value: CreateSignupFormListsRequestHeader) -> Self {
        self.header = Some(value);
        self
    }

    pub fn styles(mut self, value: Vec<CreateSignupFormListsRequestStylesItem>) -> Self {
        self.styles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormListsRequest`].
    pub fn build(self) -> Result<CreateSignupFormListsRequest, BuildError> {
        Ok(CreateSignupFormListsRequest {
            contents: self.contents,
            header: self.header,
            styles: self.styles,
        })
    }
}

