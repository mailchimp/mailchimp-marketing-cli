pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSurveyQuestionsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSurveyQuestionsResponseLinksItem>>,
    /// An array of reports for each question on the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<SurveyQuestionReport>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSurveyQuestionsResponse {
    pub fn builder() -> ListSurveyQuestionsResponseBuilder {
        <ListSurveyQuestionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyQuestionsResponseBuilder {
    links: Option<Vec<ListSurveyQuestionsResponseLinksItem>>,
    questions: Option<Vec<SurveyQuestionReport>>,
    total_items: Option<i64>,
}

impl ListSurveyQuestionsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSurveyQuestionsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn questions(mut self, value: Vec<SurveyQuestionReport>) -> Self {
        self.questions = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyQuestionsResponse`].
    pub fn build(self) -> Result<ListSurveyQuestionsResponse, BuildError> {
        Ok(ListSurveyQuestionsResponse {
            links: self.links,
            questions: self.questions,
            total_items: self.total_items,
        })
    }
}
