pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Survey respondent details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyResponsesReportingResponseResponsesItem {
    /// Information about the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ListSurveyResponsesReportingResponseResponsesItemContact>,
    /// If this contact was added to the Mailchimp audience via this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_contact: Option<bool>,
    /// The ID for the survey response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
    /// The date and time when the survey response was submitted in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
}

impl ListSurveyResponsesReportingResponseResponsesItem {
    pub fn builder() -> ListSurveyResponsesReportingResponseResponsesItemBuilder {
        <ListSurveyResponsesReportingResponseResponsesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyResponsesReportingResponseResponsesItemBuilder {
    contact: Option<ListSurveyResponsesReportingResponseResponsesItemContact>,
    is_new_contact: Option<bool>,
    response_id: Option<String>,
    submitted_at: Option<DateTime<FixedOffset>>,
}

impl ListSurveyResponsesReportingResponseResponsesItemBuilder {
    pub fn contact(mut self, value: ListSurveyResponsesReportingResponseResponsesItemContact) -> Self {
        self.contact = Some(value);
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

    /// Consumes the builder and constructs a [`ListSurveyResponsesReportingResponseResponsesItem`].
    pub fn build(self) -> Result<ListSurveyResponsesReportingResponseResponsesItem, BuildError> {
        Ok(ListSurveyResponsesReportingResponseResponsesItem {
            contact: self.contact,
            is_new_contact: self.is_new_contact,
            response_id: self.response_id,
            submitted_at: self.submitted_at,
        })
    }
}
