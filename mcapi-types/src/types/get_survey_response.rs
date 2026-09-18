pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The report for a survey.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSurveyResponse {
    /// The date and time the survey was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// A string that uniquely identifies this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the list connected to this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The name of the list connected to this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// The date and time the survey was published in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_at: Option<DateTime<FixedOffset>>,
    /// The survey's status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetSurveyResponseStatus>,
    /// The title of the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The total number of responses to this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_responses: Option<i64>,
    /// The date and time the survey was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// The URL for the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The ID used in the Mailchimp web application. View this survey report in your Mailchimp account at `https://{dc}.admin.mailchimp.com/lists/surveys/results?survey_id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl GetSurveyResponse {
    pub fn builder() -> GetSurveyResponseBuilder {
        <GetSurveyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSurveyResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    list_id: Option<String>,
    list_name: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    status: Option<GetSurveyResponseStatus>,
    title: Option<String>,
    total_responses: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
    web_id: Option<i64>,
}

impl GetSurveyResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
        self
    }

    pub fn published_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn status(mut self, value: GetSurveyResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn total_responses(mut self, value: i64) -> Self {
        self.total_responses = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSurveyResponse`].
    pub fn build(self) -> Result<GetSurveyResponse, BuildError> {
        Ok(GetSurveyResponse {
            created_at: self.created_at,
            id: self.id,
            list_id: self.list_id,
            list_name: self.list_name,
            published_at: self.published_at,
            status: self.status,
            title: self.title,
            total_responses: self.total_responses,
            updated_at: self.updated_at,
            url: self.url,
            web_id: self.web_id,
        })
    }
}
