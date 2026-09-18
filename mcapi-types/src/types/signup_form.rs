pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List signup form.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignupForm {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SignupFormLinksItem>>,
    /// The signup form body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<SignupFormContentsItem>>,
    /// Options for customizing your signup form header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<SignupFormHeader>,
    /// The signup form's list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Signup form URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_form_url: Option<String>,
    /// An array of objects, each representing an element style for the signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<SignupFormStylesItem>>,
}

impl SignupForm {
    pub fn builder() -> SignupFormBuilder {
        <SignupFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignupFormBuilder {
    links: Option<Vec<SignupFormLinksItem>>,
    contents: Option<Vec<SignupFormContentsItem>>,
    header: Option<SignupFormHeader>,
    list_id: Option<String>,
    signup_form_url: Option<String>,
    styles: Option<Vec<SignupFormStylesItem>>,
}

impl SignupFormBuilder {
    pub fn links(mut self, value: Vec<SignupFormLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn contents(mut self, value: Vec<SignupFormContentsItem>) -> Self {
        self.contents = Some(value);
        self
    }

    pub fn header(mut self, value: SignupFormHeader) -> Self {
        self.header = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn signup_form_url(mut self, value: impl Into<String>) -> Self {
        self.signup_form_url = Some(value.into());
        self
    }

    pub fn styles(mut self, value: Vec<SignupFormStylesItem>) -> Self {
        self.styles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignupForm`].
    pub fn build(self) -> Result<SignupForm, BuildError> {
        Ok(SignupForm {
            links: self.links,
            contents: self.contents,
            header: self.header,
            list_id: self.list_id,
            signup_form_url: self.signup_form_url,
            styles: self.styles,
        })
    }
}
