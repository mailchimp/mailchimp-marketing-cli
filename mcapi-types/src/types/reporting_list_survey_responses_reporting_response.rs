pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyResponsesReportingResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveyResponsesReportingResponseLinksItem>>,
    /// An array of responses to a survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<ListSurveyResponsesReportingResponseResponsesItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveyResponsesReportingResponse {
    pub fn builder() -> ListSurveyResponsesReportingResponseBuilder {
        <ListSurveyResponsesReportingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyResponsesReportingResponseBuilder {
    links: Option<Vec<ListSurveyResponsesReportingResponseLinksItem>>,
    responses: Option<Vec<ListSurveyResponsesReportingResponseResponsesItem>>,
    total_items: Option<i64>,
}

impl ListSurveyResponsesReportingResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveyResponsesReportingResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn responses(mut self, value: Vec<ListSurveyResponsesReportingResponseResponsesItem>) -> Self {
        self.responses = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyResponsesReportingResponse`].
    pub fn build(self) -> Result<ListSurveyResponsesReportingResponse, BuildError> {
        Ok(ListSurveyResponsesReportingResponse {
            links: self.links,
            responses: self.responses,
            total_items: self.total_items,
        })
    }
}
