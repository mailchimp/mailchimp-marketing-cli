pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The details of a survey question's report.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SurveyQuestionReport {
    /// The average rating for this range question.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_rating: Option<f64>,
    /// For email question types, how many are new, known, or unknown contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_counts: Option<SurveyQuestionReportContactCounts>,
    /// Whether this survey question has an 'other' option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_other: Option<bool>,
    /// The ID of the survey question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this survey question is required to answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// A [merge field](https://mailchimp.com/developer/marketing/docs/merge-fields/) for an audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_field: Option<SurveyQuestionReportMergeField>,
    /// The answer choices for this question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SurveyQuestionReportOptionsItem>>,
    /// Label used for the 'other' option of this survey question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_label: Option<String>,
    /// Placeholder text for this survey question's answer box.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_label: Option<String>,
    /// The query of the survey question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Label for the high end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_high_label: Option<String>,
    /// Label for the low end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_low_label: Option<String>,
    /// Whether the subscribe checkbox is shown for this email question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_checkbox_enabled: Option<bool>,
    /// Label used for the subscribe checkbox for this email question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_checkbox_label: Option<String>,
    /// The unique ID of the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
    /// The total number of responses to this question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_responses: Option<i64>,
    /// The response type of the survey question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SurveyQuestionReportType>,
}

impl SurveyQuestionReport {
    pub fn builder() -> SurveyQuestionReportBuilder {
        <SurveyQuestionReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveyQuestionReportBuilder {
    average_rating: Option<f64>,
    contact_counts: Option<SurveyQuestionReportContactCounts>,
    has_other: Option<bool>,
    id: Option<String>,
    is_required: Option<bool>,
    merge_field: Option<SurveyQuestionReportMergeField>,
    options: Option<Vec<SurveyQuestionReportOptionsItem>>,
    other_label: Option<String>,
    placeholder_label: Option<String>,
    query: Option<String>,
    range_high_label: Option<String>,
    range_low_label: Option<String>,
    subscribe_checkbox_enabled: Option<bool>,
    subscribe_checkbox_label: Option<String>,
    survey_id: Option<String>,
    total_responses: Option<i64>,
    r#type: Option<SurveyQuestionReportType>,
}

impl SurveyQuestionReportBuilder {
    pub fn average_rating(mut self, value: f64) -> Self {
        self.average_rating = Some(value);
        self
    }

    pub fn contact_counts(mut self, value: SurveyQuestionReportContactCounts) -> Self {
        self.contact_counts = Some(value);
        self
    }

    pub fn has_other(mut self, value: bool) -> Self {
        self.has_other = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn merge_field(mut self, value: SurveyQuestionReportMergeField) -> Self {
        self.merge_field = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<SurveyQuestionReportOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn other_label(mut self, value: impl Into<String>) -> Self {
        self.other_label = Some(value.into());
        self
    }

    pub fn placeholder_label(mut self, value: impl Into<String>) -> Self {
        self.placeholder_label = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn range_high_label(mut self, value: impl Into<String>) -> Self {
        self.range_high_label = Some(value.into());
        self
    }

    pub fn range_low_label(mut self, value: impl Into<String>) -> Self {
        self.range_low_label = Some(value.into());
        self
    }

    pub fn subscribe_checkbox_enabled(mut self, value: bool) -> Self {
        self.subscribe_checkbox_enabled = Some(value);
        self
    }

    pub fn subscribe_checkbox_label(mut self, value: impl Into<String>) -> Self {
        self.subscribe_checkbox_label = Some(value.into());
        self
    }

    pub fn survey_id(mut self, value: impl Into<String>) -> Self {
        self.survey_id = Some(value.into());
        self
    }

    pub fn total_responses(mut self, value: i64) -> Self {
        self.total_responses = Some(value);
        self
    }

    pub fn r#type(mut self, value: SurveyQuestionReportType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurveyQuestionReport`].
    pub fn build(self) -> Result<SurveyQuestionReport, BuildError> {
        Ok(SurveyQuestionReport {
            average_rating: self.average_rating,
            contact_counts: self.contact_counts,
            has_other: self.has_other,
            id: self.id,
            is_required: self.is_required,
            merge_field: self.merge_field,
            options: self.options,
            other_label: self.other_label,
            placeholder_label: self.placeholder_label,
            query: self.query,
            range_high_label: self.range_high_label,
            range_low_label: self.range_low_label,
            subscribe_checkbox_enabled: self.subscribe_checkbox_enabled,
            subscribe_checkbox_label: self.subscribe_checkbox_label,
            survey_id: self.survey_id,
            total_responses: self.total_responses,
            r#type: self.r#type,
        })
    }
}
