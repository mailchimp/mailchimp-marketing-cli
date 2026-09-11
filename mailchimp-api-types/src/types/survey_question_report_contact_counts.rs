pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// For email question types, how many are new, known, or unknown contacts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurveyQuestionReportContactCounts {
    /// The number of known contacts that responded to this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known: Option<i64>,
    /// The number of new contacts that responded to this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<i64>,
    /// The number of unknown contacts that responded to this survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

impl SurveyQuestionReportContactCounts {
    pub fn builder() -> SurveyQuestionReportContactCountsBuilder {
        <SurveyQuestionReportContactCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveyQuestionReportContactCountsBuilder {
    known: Option<i64>,
    new: Option<i64>,
    unknown: Option<i64>,
}

impl SurveyQuestionReportContactCountsBuilder {
    pub fn known(mut self, value: i64) -> Self {
        self.known = Some(value);
        self
    }

    pub fn new(mut self, value: i64) -> Self {
        self.new = Some(value);
        self
    }

    pub fn unknown(mut self, value: i64) -> Self {
        self.unknown = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurveyQuestionReportContactCounts`].
    pub fn build(self) -> Result<SurveyQuestionReportContactCounts, BuildError> {
        Ok(SurveyQuestionReportContactCounts {
            known: self.known,
            new: self.new,
            unknown: self.unknown,
        })
    }
}
