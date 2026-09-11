pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by predicted gender.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemPredictedGenderField {
    #[serde(rename = "predicted_gender")]
    PredictedGender,
}
impl fmt::Display for SegmentTypeItemPredictedGenderField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PredictedGender => "predicted_gender",
        };
        write!(f, "{}", s)
    }
}
