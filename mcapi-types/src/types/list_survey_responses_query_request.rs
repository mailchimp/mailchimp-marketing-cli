pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-survey-responses
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyResponsesQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The ID of the question that was answered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answered_question: Option<i64>,
    /// The ID of the option chosen to filter responses on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chose_answer: Option<String>,
    /// Filter survey responses by familiarity of the respondents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respondent_familiarity_is: Option<ListSurveyResponsesReportingRequestRespondentFamiliarityIs>,
}

impl ListSurveyResponsesQueryRequest {
    pub fn builder() -> ListSurveyResponsesQueryRequestBuilder {
        <ListSurveyResponsesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyResponsesQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    answered_question: Option<i64>,
    chose_answer: Option<String>,
    respondent_familiarity_is: Option<ListSurveyResponsesReportingRequestRespondentFamiliarityIs>,
}

impl ListSurveyResponsesQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn answered_question(mut self, value: i64) -> Self {
        self.answered_question = Some(value);
        self
    }

    pub fn chose_answer(mut self, value: impl Into<String>) -> Self {
        self.chose_answer = Some(value.into());
        self
    }

    pub fn respondent_familiarity_is(mut self, value: ListSurveyResponsesReportingRequestRespondentFamiliarityIs) -> Self {
        self.respondent_familiarity_is = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyResponsesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListSurveyResponsesQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListSurveyResponsesQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListSurveyResponsesQueryRequest, BuildError> {
        Ok(ListSurveyResponsesQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            answered_question: self.answered_question,
            chose_answer: self.chose_answer,
            respondent_familiarity_is: self.respondent_familiarity_is,
        })
    }
}

