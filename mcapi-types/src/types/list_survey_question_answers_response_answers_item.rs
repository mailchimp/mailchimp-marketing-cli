pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The details of a survey question's answer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyQuestionAnswersResponseAnswersItem {
    /// Information about the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ListSurveyQuestionAnswersResponseAnswersItemContact>,
    /// The ID of the answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If this contact was added to the Mailchimp audience via this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_contact: Option<bool>,
    /// The ID of the survey response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
    /// The date and time when the survey response was submitted in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
    /// The raw text answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListSurveyQuestionAnswersResponseAnswersItem {
    pub fn builder() -> ListSurveyQuestionAnswersResponseAnswersItemBuilder {
        <ListSurveyQuestionAnswersResponseAnswersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyQuestionAnswersResponseAnswersItemBuilder {
    contact: Option<ListSurveyQuestionAnswersResponseAnswersItemContact>,
    id: Option<String>,
    is_new_contact: Option<bool>,
    response_id: Option<String>,
    submitted_at: Option<DateTime<FixedOffset>>,
    value: Option<String>,
}

impl ListSurveyQuestionAnswersResponseAnswersItemBuilder {
    pub fn contact(mut self, value: ListSurveyQuestionAnswersResponseAnswersItemContact) -> Self {
        self.contact = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_new_contact(mut self, value: bool) -> Self {
        self.is_new_contact = Some(value);
        self
    }

    pub fn response_id(mut self, value: impl Into<String>) -> Self {
        self.response_id = Some(value.into());
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyQuestionAnswersResponseAnswersItem`].
    pub fn build(self) -> Result<ListSurveyQuestionAnswersResponseAnswersItem, BuildError> {
        Ok(ListSurveyQuestionAnswersResponseAnswersItem {
            contact: self.contact,
            id: self.id,
            is_new_contact: self.is_new_contact,
            response_id: self.response_id,
            submitted_at: self.submitted_at,
            value: self.value,
        })
    }
}
