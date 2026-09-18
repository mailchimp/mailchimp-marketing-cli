pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyResponsesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveyResponsesResponseLinksItem>>,
    /// An array of responses to a survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<ListSurveyResponsesResponseResponsesItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveyResponsesResponse {
    pub fn builder() -> ListSurveyResponsesResponseBuilder {
        <ListSurveyResponsesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyResponsesResponseBuilder {
    links: Option<Vec<ListSurveyResponsesResponseLinksItem>>,
    responses: Option<Vec<ListSurveyResponsesResponseResponsesItem>>,
    total_items: Option<i64>,
}

impl ListSurveyResponsesResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveyResponsesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn responses(mut self, value: Vec<ListSurveyResponsesResponseResponsesItem>) -> Self {
        self.responses = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyResponsesResponse`].
    pub fn build(self) -> Result<ListSurveyResponsesResponse, BuildError> {
        Ok(ListSurveyResponsesResponse {
            links: self.links,
            responses: self.responses,
            total_items: self.total_items,
        })
    }
}
