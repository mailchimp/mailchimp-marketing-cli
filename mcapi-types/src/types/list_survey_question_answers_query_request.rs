pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-survey-question-answers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyQuestionAnswersQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Filter survey responses by familiarity of the respondents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respondent_familiarity_is: Option<ListSurveyQuestionAnswersReportingRequestRespondentFamiliarityIs>,
}

impl ListSurveyQuestionAnswersQueryRequest {
    pub fn builder() -> ListSurveyQuestionAnswersQueryRequestBuilder {
        <ListSurveyQuestionAnswersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyQuestionAnswersQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    respondent_familiarity_is: Option<ListSurveyQuestionAnswersReportingRequestRespondentFamiliarityIs>,
}

impl ListSurveyQuestionAnswersQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn respondent_familiarity_is(mut self, value: ListSurveyQuestionAnswersReportingRequestRespondentFamiliarityIs) -> Self {
        self.respondent_familiarity_is = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyQuestionAnswersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListSurveyQuestionAnswersQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListSurveyQuestionAnswersQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListSurveyQuestionAnswersQueryRequest, BuildError> {
        Ok(ListSurveyQuestionAnswersQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            respondent_familiarity_is: self.respondent_familiarity_is,
        })
    }
}

