pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single question and the response to that question.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSurveyResponsResponseResultsItem {
    /// The answer to this survey question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// The survey question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The unique ID for this question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    /// The type of question this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_type: Option<GetSurveyResponsResponseResultsItemQuestionType>,
}

impl GetSurveyResponsResponseResultsItem {
    pub fn builder() -> GetSurveyResponsResponseResultsItemBuilder {
        <GetSurveyResponsResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSurveyResponsResponseResultsItemBuilder {
    answer: Option<String>,
    query: Option<String>,
    question_id: Option<String>,
    question_type: Option<GetSurveyResponsResponseResultsItemQuestionType>,
}

impl GetSurveyResponsResponseResultsItemBuilder {
    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn question_id(mut self, value: impl Into<String>) -> Self {
        self.question_id = Some(value.into());
        self
    }

    pub fn question_type(mut self, value: GetSurveyResponsResponseResultsItemQuestionType) -> Self {
        self.question_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSurveyResponsResponseResultsItem`].
    pub fn build(self) -> Result<GetSurveyResponsResponseResultsItem, BuildError> {
        Ok(GetSurveyResponsResponseResultsItem {
            answer: self.answer,
            query: self.query,
            question_id: self.question_id,
            question_type: self.question_type,
        })
    }
}
