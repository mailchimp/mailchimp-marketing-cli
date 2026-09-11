pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A [merge field](https://mailchimp.com/developer/marketing/docs/merge-fields/) for an audience.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurveyQuestionReportMergeField {
    /// An unchanging id for the merge field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The [label](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for the merge field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The [type](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for the merge field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SurveyQuestionReportMergeFieldType>,
}

impl SurveyQuestionReportMergeField {
    pub fn builder() -> SurveyQuestionReportMergeFieldBuilder {
        <SurveyQuestionReportMergeFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveyQuestionReportMergeFieldBuilder {
    id: Option<i64>,
    label: Option<String>,
    r#type: Option<SurveyQuestionReportMergeFieldType>,
}

impl SurveyQuestionReportMergeFieldBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SurveyQuestionReportMergeFieldType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurveyQuestionReportMergeField`].
    pub fn build(self) -> Result<SurveyQuestionReportMergeField, BuildError> {
        Ok(SurveyQuestionReportMergeField {
            id: self.id,
            label: self.label,
            r#type: self.r#type,
        })
    }
}
