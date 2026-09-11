pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segmenting based off of a subscriber's language.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemLanguageField {
    #[serde(rename = "language")]
    Language,
}
impl fmt::Display for SegmentTypeItemLanguageField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Language => "language",
        };
        write!(f, "{}", s)
    }
}
