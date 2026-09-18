pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyQuestionAnswersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveyQuestionAnswersResponseLinksItem>>,
    /// An array of answers for a question on the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<ListSurveyQuestionAnswersResponseAnswersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveyQuestionAnswersResponse {
    pub fn builder() -> ListSurveyQuestionAnswersResponseBuilder {
        <ListSurveyQuestionAnswersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyQuestionAnswersResponseBuilder {
    links: Option<Vec<ListSurveyQuestionAnswersResponseLinksItem>>,
    answers: Option<Vec<ListSurveyQuestionAnswersResponseAnswersItem>>,
    total_items: Option<i64>,
}

impl ListSurveyQuestionAnswersResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveyQuestionAnswersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn answers(mut self, value: Vec<ListSurveyQuestionAnswersResponseAnswersItem>) -> Self {
        self.answers = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyQuestionAnswersResponse`].
    pub fn build(self) -> Result<ListSurveyQuestionAnswersResponse, BuildError> {
        Ok(ListSurveyQuestionAnswersResponse {
            links: self.links,
            answers: self.answers,
            total_items: self.total_items,
        })
    }
}
