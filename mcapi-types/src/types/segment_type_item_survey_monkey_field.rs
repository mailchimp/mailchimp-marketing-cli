pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by interaction with a SurveyMonkey survey.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSurveyMonkeyField {
    #[serde(rename = "survey_monkey")]
    SurveyMonkey,
}
impl fmt::Display for SegmentTypeItemSurveyMonkeyField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SurveyMonkey => "survey_monkey",
        };
        write!(f, "{}", s)
    }
}
