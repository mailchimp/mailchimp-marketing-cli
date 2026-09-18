pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSignupFormRequest {
    /// The signup form body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<CreateSignupFormRequestContentsItem>>,
    /// Options for customizing your signup form header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<CreateSignupFormRequestHeader>,
    /// An array of objects, each representing an element style for the signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<CreateSignupFormRequestStylesItem>>,
}

impl CreateSignupFormRequest {
    pub fn builder() -> CreateSignupFormRequestBuilder {
        <CreateSignupFormRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSignupFormRequestBuilder {
    contents: Option<Vec<CreateSignupFormRequestContentsItem>>,
    header: Option<CreateSignupFormRequestHeader>,
    styles: Option<Vec<CreateSignupFormRequestStylesItem>>,
}

impl CreateSignupFormRequestBuilder {
    pub fn contents(mut self, value: Vec<CreateSignupFormRequestContentsItem>) -> Self {
        self.contents = Some(value);
        self
    }

    pub fn header(mut self, value: CreateSignupFormRequestHeader) -> Self {
        self.header = Some(value);
        self
    }

    pub fn styles(mut self, value: Vec<CreateSignupFormRequestStylesItem>) -> Self {
        self.styles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSignupFormRequest`].
    pub fn build(self) -> Result<CreateSignupFormRequest, BuildError> {
        Ok(CreateSignupFormRequest {
            contents: self.contents,
            header: self.header,
            styles: self.styles,
        })
    }
}

