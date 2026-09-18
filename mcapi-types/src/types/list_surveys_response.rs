pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveysResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveysResponseLinksItem>>,
    /// The surveys that have reports available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surveys: Option<Vec<ListSurveysResponseSurveysItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveysResponse {
    pub fn builder() -> ListSurveysResponseBuilder {
        <ListSurveysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveysResponseBuilder {
    links: Option<Vec<ListSurveysResponseLinksItem>>,
    surveys: Option<Vec<ListSurveysResponseSurveysItem>>,
    total_items: Option<i64>,
}

impl ListSurveysResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveysResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn surveys(mut self, value: Vec<ListSurveysResponseSurveysItem>) -> Self {
        self.surveys = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveysResponse`].
    pub fn build(self) -> Result<ListSurveysResponse, BuildError> {
        Ok(ListSurveysResponse {
            links: self.links,
            surveys: self.surveys,
            total_items: self.total_items,
        })
    }
}
