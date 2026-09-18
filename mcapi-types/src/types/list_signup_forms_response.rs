pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List Signup Forms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSignupFormsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSignupFormsResponseLinksItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// List signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_forms: Option<Vec<SignupForm>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSignupFormsResponse {
    pub fn builder() -> ListSignupFormsResponseBuilder {
        <ListSignupFormsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSignupFormsResponseBuilder {
    links: Option<Vec<ListSignupFormsResponseLinksItem>>,
    list_id: Option<String>,
    signup_forms: Option<Vec<SignupForm>>,
    total_items: Option<i64>,
}

impl ListSignupFormsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSignupFormsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn signup_forms(mut self, value: Vec<SignupForm>) -> Self {
        self.signup_forms = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSignupFormsResponse`].
    pub fn build(self) -> Result<ListSignupFormsResponse, BuildError> {
        Ok(ListSignupFormsResponse {
            links: self.links,
            list_id: self.list_id,
            signup_forms: self.signup_forms,
            total_items: self.total_items,
        })
    }
}
