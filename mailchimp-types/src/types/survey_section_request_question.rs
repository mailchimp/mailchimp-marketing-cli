pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A survey question. On PATCH, include the question id to update it. Omitting question id creates a new question; it does not delete an existing one. To delete a question, omit its section from the sections array.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurveySectionRequestQuestion {
    /// The question ID. On PATCH, include to update an existing question; omit to add a new question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The question text.
    #[serde(default)]
    pub query: String,
    /// The response type of the survey question.
    pub r#type: SurveySectionRequestQuestionType,
    /// Whether this question is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// Whether this question has an 'other' option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_other: Option<bool>,
    /// Label for the 'other' option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_label: Option<String>,
    /// Label for the low end of a range question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_low_label: Option<String>,
    /// Label for the high end of a range question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_high_label: Option<String>,
    /// Low value for a range question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_low_value: Option<i64>,
    /// High value for a range question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_high_value: Option<i64>,
    /// How a range question is presented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_presentation: Option<String>,
    /// Placeholder text for text or email questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_label: Option<String>,
    /// Whether the subscribe checkbox is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_checkbox_enabled: Option<bool>,
    /// Label for the subscribe checkbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_checkbox_label: Option<String>,
    /// Whether responses should automatically apply tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_auto_tag: Option<bool>,
    /// Answer options for pickOne, pickMany, or dropdown questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SurveySectionRequestQuestionOptionsItem>>,
    /// Merge field mapping for contact information questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_field: Option<HashMap<String, serde_json::Value>>,
}

impl SurveySectionRequestQuestion {
    pub fn builder() -> SurveySectionRequestQuestionBuilder {
        <SurveySectionRequestQuestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveySectionRequestQuestionBuilder {
    id: Option<String>,
    query: Option<String>,
    r#type: Option<SurveySectionRequestQuestionType>,
    is_required: Option<bool>,
    has_other: Option<bool>,
    other_label: Option<String>,
    range_low_label: Option<String>,
    range_high_label: Option<String>,
    range_low_value: Option<i64>,
    range_high_value: Option<i64>,
    range_presentation: Option<String>,
    placeholder_label: Option<String>,
    subscribe_checkbox_enabled: Option<bool>,
    subscribe_checkbox_label: Option<String>,
    should_auto_tag: Option<bool>,
    options: Option<Vec<SurveySectionRequestQuestionOptionsItem>>,
    merge_field: Option<HashMap<String, serde_json::Value>>,
}

impl SurveySectionRequestQuestionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SurveySectionRequestQuestionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn has_other(mut self, value: bool) -> Self {
        self.has_other = Some(value);
        self
    }

    pub fn other_label(mut self, value: impl Into<String>) -> Self {
        self.other_label = Some(value.into());
        self
    }

    pub fn range_low_label(mut self, value: impl Into<String>) -> Self {
        self.range_low_label = Some(value.into());
        self
    }

    pub fn range_high_label(mut self, value: impl Into<String>) -> Self {
        self.range_high_label = Some(value.into());
        self
    }

    pub fn range_low_value(mut self, value: i64) -> Self {
        self.range_low_value = Some(value);
        self
    }

    pub fn range_high_value(mut self, value: i64) -> Self {
        self.range_high_value = Some(value);
        self
    }

    pub fn range_presentation(mut self, value: impl Into<String>) -> Self {
        self.range_presentation = Some(value.into());
        self
    }

    pub fn placeholder_label(mut self, value: impl Into<String>) -> Self {
        self.placeholder_label = Some(value.into());
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

    pub fn should_auto_tag(mut self, value: bool) -> Self {
        self.should_auto_tag = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<SurveySectionRequestQuestionOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn merge_field(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.merge_field = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurveySectionRequestQuestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SurveySectionRequestQuestionBuilder::query)
    /// - [`r#type`](SurveySectionRequestQuestionBuilder::r#type)
    pub fn build(self) -> Result<SurveySectionRequestQuestion, BuildError> {
        Ok(SurveySectionRequestQuestion {
            id: self.id,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            is_required: self.is_required,
            has_other: self.has_other,
            other_label: self.other_label,
            range_low_label: self.range_low_label,
            range_high_label: self.range_high_label,
            range_low_value: self.range_low_value,
            range_high_value: self.range_high_value,
            range_presentation: self.range_presentation,
            placeholder_label: self.placeholder_label,
            subscribe_checkbox_enabled: self.subscribe_checkbox_enabled,
            subscribe_checkbox_label: self.subscribe_checkbox_label,
            should_auto_tag: self.should_auto_tag,
            options: self.options,
            merge_field: self.merge_field,
        })
    }
}
