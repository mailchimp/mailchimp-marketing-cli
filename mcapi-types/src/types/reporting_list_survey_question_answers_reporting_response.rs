pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyQuestionAnswersReportingResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveyQuestionAnswersReportingResponseLinksItem>>,
    /// An array of answers for a question on the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<ListSurveyQuestionAnswersReportingResponseAnswersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveyQuestionAnswersReportingResponse {
    pub fn builder() -> ListSurveyQuestionAnswersReportingResponseBuilder {
        <ListSurveyQuestionAnswersReportingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyQuestionAnswersReportingResponseBuilder {
    links: Option<Vec<ListSurveyQuestionAnswersReportingResponseLinksItem>>,
    answers: Option<Vec<ListSurveyQuestionAnswersReportingResponseAnswersItem>>,
    total_items: Option<i64>,
}

impl ListSurveyQuestionAnswersReportingResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveyQuestionAnswersReportingResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn answers(mut self, value: Vec<ListSurveyQuestionAnswersReportingResponseAnswersItem>) -> Self {
        self.answers = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyQuestionAnswersReportingResponse`].
    pub fn build(self) -> Result<ListSurveyQuestionAnswersReportingResponse, BuildError> {
        Ok(ListSurveyQuestionAnswersReportingResponse {
            links: self.links,
            answers: self.answers,
            total_items: self.total_items,
        })
    }
}
