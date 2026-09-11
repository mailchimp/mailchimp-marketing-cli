pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// This resource serves as a namespace for e-commerce-related resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListEcommerceResponseLinksItem>>,
}

impl ListEcommerceResponse {
    pub fn builder() -> ListEcommerceResponseBuilder {
        <ListEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEcommerceResponseBuilder {
    links: Option<Vec<ListEcommerceResponseLinksItem>>,
}

impl ListEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEcommerceResponse`].
    pub fn build(self) -> Result<ListEcommerceResponse, BuildError> {
        Ok(ListEcommerceResponse {
            links: self.links,
        })
    }
}
