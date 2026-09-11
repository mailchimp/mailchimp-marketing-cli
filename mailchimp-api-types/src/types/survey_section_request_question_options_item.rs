pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurveySectionRequestQuestionOptionsItem {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl SurveySectionRequestQuestionOptionsItem {
    pub fn builder() -> SurveySectionRequestQuestionOptionsItemBuilder {
        <SurveySectionRequestQuestionOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveySectionRequestQuestionOptionsItemBuilder {
    label: Option<String>,
    value: Option<String>,
}

impl SurveySectionRequestQuestionOptionsItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SurveySectionRequestQuestionOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](SurveySectionRequestQuestionOptionsItemBuilder::label)
    /// - [`value`](SurveySectionRequestQuestionOptionsItemBuilder::value)
    pub fn build(self) -> Result<SurveySectionRequestQuestionOptionsItem, BuildError> {
        Ok(SurveySectionRequestQuestionOptionsItem {
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
