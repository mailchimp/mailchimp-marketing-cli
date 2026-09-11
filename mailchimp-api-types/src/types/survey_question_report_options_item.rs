pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurveyQuestionReportOptionsItem {
    /// The count of responses that selected this survey question option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The ID for this survey question option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The label for this survey question option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl SurveyQuestionReportOptionsItem {
    pub fn builder() -> SurveyQuestionReportOptionsItemBuilder {
        <SurveyQuestionReportOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveyQuestionReportOptionsItemBuilder {
    count: Option<i64>,
    id: Option<String>,
    label: Option<String>,
}

impl SurveyQuestionReportOptionsItemBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SurveyQuestionReportOptionsItem`].
    pub fn build(self) -> Result<SurveyQuestionReportOptionsItem, BuildError> {
        Ok(SurveyQuestionReportOptionsItem {
            count: self.count,
            id: self.id,
            label: self.label,
        })
    }
}
