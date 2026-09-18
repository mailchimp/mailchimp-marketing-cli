pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by predicted age.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemPredictedAgeField {
    #[serde(rename = "predicted_age_range")]
    PredictedAgeRange,
}
impl fmt::Display for SegmentTypeItemPredictedAgeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PredictedAgeRange => "predicted_age_range",
        };
        write!(f, "{}", s)
    }
}
