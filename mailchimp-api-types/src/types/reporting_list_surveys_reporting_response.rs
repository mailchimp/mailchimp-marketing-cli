pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveysReportingResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveysReportingResponseLinksItem>>,
    /// The surveys that have reports available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surveys: Option<Vec<ListSurveysReportingResponseSurveysItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveysReportingResponse {
    pub fn builder() -> ListSurveysReportingResponseBuilder {
        <ListSurveysReportingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveysReportingResponseBuilder {
    links: Option<Vec<ListSurveysReportingResponseLinksItem>>,
    surveys: Option<Vec<ListSurveysReportingResponseSurveysItem>>,
    total_items: Option<i64>,
}

impl ListSurveysReportingResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveysReportingResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn surveys(mut self, value: Vec<ListSurveysReportingResponseSurveysItem>) -> Self {
        self.surveys = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveysReportingResponse`].
    pub fn build(self) -> Result<ListSurveysReportingResponse, BuildError> {
        Ok(ListSurveysReportingResponse {
            links: self.links,
            surveys: self.surveys,
            total_items: self.total_items,
        })
    }
}
